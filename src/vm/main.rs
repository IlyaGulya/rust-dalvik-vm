use std::ops::DerefMut;
use std::panic;
use std::sync::Arc;

use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::RwLock;

use crate::bytecode::instructions::InvokeOp;
use crate::dex::dex_file::Method;
use crate::entrypoint;
use crate::runtime::class::InvokeResult;
use crate::runtime::frame::Frame;
use crate::runtime::interpreter::{Interpreter, InterpreterSuspendReason};
use crate::runtime::runtime::Runtime;
use crate::runtime::value::Value;
use crate::vm::debugger::{DebuggerCommand, DebuggerState};

pub struct Debugger {
    pub debugger_side: DebuggerSide,
    pub vm_side: DebuggerVmSide,
}

pub struct DebuggerSide {
    pub state_receiver: Receiver<Arc<DebuggerState>>,
    pub commands_sender: Sender<DebuggerCommand>,
}

pub struct DebuggerVmSide {
    pub state_sender: Sender<Arc<DebuggerState>>,
    pub commands_receiver: Receiver<DebuggerCommand>,
}

impl Debugger {
    pub(crate) fn maybe_update_state(&self) {}
}

impl Debugger {
    pub fn new() -> Debugger {
        let (mut debugger_state_tx, mut debugger_state_rx) = tokio::sync::mpsc::channel::<Arc<DebuggerState>>(100);
        let (debugger_cmds_tx, mut debugger_cmds_rx) = tokio::sync::mpsc::channel::<DebuggerCommand>(100);

        Debugger {
            debugger_side: DebuggerSide {
                state_receiver: debugger_state_rx,
                commands_sender: debugger_cmds_tx,
            },
            vm_side: DebuggerVmSide {
                state_sender: debugger_state_tx,
                commands_receiver: debugger_cmds_rx,
            },
        }
    }
}

pub struct VmState {
    pub runtime: Arc<Runtime>,
    pub debugger_state_sender: Option<Sender<Arc<DebuggerState>>>,
    pub debugger_cmds: Option<Receiver<DebuggerCommand>>,
    pub is_paused: bool,
    pub frames: Vec<Arc<RwLock<Frame>>>,
    pub current_frame: Option<Arc<RwLock<Frame>>>,
}

impl VmState {
    pub(crate) fn maybe_notify_debugger(&self) {
        if let Some(sender) = &self.debugger_state_sender {
            sender.try_send(Arc::new(self.create_debugger_state())).unwrap();
        }
    }

    pub(crate) async fn execute_debugger_commands(&mut self) {
        let mut receiver = self.debugger_cmds.take();
        if let Some(receiver) = &mut receiver {
            if self.is_paused {
                while let Some(cmd) = receiver.recv().await {
                    match cmd {
                        DebuggerCommand::StepOver => {
                            return;
                        }
                        DebuggerCommand::StepBack => {
                            let current_frame = self.current_frame.take();
                            if let Some(frame) = &current_frame {
                                let mut frame = frame.write().await;
                                let frame = frame.deref_mut();
                                if frame.has_history() {
                                    frame.pop_history();
                                } else if self.frames.len() > 0 {
                                    self.current_frame = self.frames.pop();
                                    return;
                                } else {
                                    panic!("Cannot step back because there are no frames left")
                                }
                            } else {
                                panic!("Cannot step back because current frame is not set")
                            }
                            self.current_frame = current_frame;
                            self.maybe_notify_debugger();
                        }
                    }
                }
            }
        }
    }

    fn create_debugger_state(&self) -> DebuggerState {
        DebuggerState {
            frames: self.frames.clone(),
            current_frame: self.current_frame.clone(),
        }
    }
}

struct Vm {
    state: VmState,
    debugger: Option<DebuggerVmSide>,
}

impl VmState {
    fn push_frame(&mut self, frame: Frame) {
        self.frames.push(Arc::new(RwLock::new(frame)));
    }

    fn pop_frame(&mut self) -> Option<Arc<RwLock<Frame>>> {
        let frame = self.frames.pop();
        self.maybe_notify_debugger();
        frame
    }

    fn is_paused() -> bool {
        true
    }
}

pub(crate) async fn start(debugger: Option<DebuggerVmSide>) {
    let main_class_name = "HelloWorld";
    let main_class_name = format!("L{};", main_class_name);

    let mut vm_state = VmState {
        runtime: Arc::new(Runtime::default()),
        debugger_state_sender: debugger.as_ref().map(|debugger| debugger.state_sender.clone()),
        debugger_cmds: debugger.map(|debugger| debugger.commands_receiver),
        is_paused: true,
        frames: vec![
            Arc::new(Frame::public_static_main(&main_class_name).into()),
            Arc::new(entrypoint::entrypoint_frame().into()),
        ],
        current_frame: None,
    };

    let vm_result = vm_loop(&mut vm_state).await;

    match vm_result {
        Err(err) => {
            let stacktrace = get_stacktrace(&vm_state.frames).await;
            panic!("Error: {}\nStacktrace:\n{}", err, stacktrace);
        }
        _ => {}
    }
}

async fn vm_loop(vm_state: &mut VmState) -> Result<(), String> {
    let mut runtime = Runtime::default();

    runtime.load_dex("runtime/build/entrypoint.dex");
    runtime.load_dex("toolkit/runtime.dex");
    runtime.load_dex("tests/vm/hello.dex");

    let interpreter = Interpreter::new();
    loop {
        let frame = vm_state.frames.last();
        
        let frame =
            if let Some(frame) = frame {
                frame.clone()
            } else {
                break;
            };

        let mut frame = frame.write().await;
        let mut frame = frame.deref_mut();

        if frame.is_completed().await {
            vm_state.pop_frame();
            continue;
        }

        let suspend_reason = interpreter.interpret_current_frame(vm_state, &mut frame).await?;

        match suspend_reason {
            InterpreterSuspendReason::EndOfFrame => {
                vm_state.pop_frame();
            }
            InterpreterSuspendReason::ReturnVoid => {
                vm_state.pop_frame();
            }
            InterpreterSuspendReason::ReturnWide(_) => {
                panic!("ReturnWide is not implemented yet!");
            }
            InterpreterSuspendReason::Throw => {}
            InterpreterSuspendReason::NewInstance { register, class } => {
                let class = runtime.get_class(class).await.unwrap();
                let instance = class.read().await.new_instance();
                frame.set_register(register, Value::Object(instance)).await;
                vm_state.maybe_notify_debugger();
                frame.move_to_next_instruction().await;
            }
            InterpreterSuspendReason::Invoke { op, definer, args, method } => {
                frame.move_to_next_instruction().await;
                println!("Invoking method {}->{} with args {:?}", definer, method.name_with_descriptor(), args);
                let class =
                    runtime
                        .get_class(definer.clone()).await
                        .expect(format!("Class not found: {:?}", definer).as_str());

                let mut class = class.write().await;
                if !class.is_initialized() {
                    if let Some(frame) = class.get_initializer() {
                        vm_state.push_frame(frame);
                        /* TODO
                            not sure if this is correct place to set initialized.
                            Maybe we should wait until frame is finished?
                         */
                        class.set_initialized();
                        continue;
                    }
                }

                match op {
                    InvokeOp::INVOKE_VIRTUAL => {
                        let frame = class.invoke_virtual(method, args.as_slice());
                        vm_state.push_frame(frame);
                    }
                    // InvokeOp::INVOKE_SUPER => {}
                    InvokeOp::INVOKE_DIRECT => {
                        let frame = class.invoke_direct(method, args.as_slice());
                        vm_state.push_frame(frame);
                    }
                    InvokeOp::INVOKE_STATIC => {
                        let result = class.invoke_static(method, args.as_slice());
                        match result {
                            InvokeResult::Native(method) => {
                                invoke_native(frame, method.clone(), args);
                            }
                            InvokeResult::Frame(frame) => {
                                vm_state.push_frame(frame);
                            }
                        }
                    }
                    // InvokeOp::INVOKE_INTERFACE => {}
                    instruction => panic!("Not implemented: {:?}", instruction)
                }
            }
        }
    }
    Ok(())
}

async fn get_stacktrace(frames: &Vec<Arc<RwLock<Frame>>>) -> String {
    let mut stacktrace = String::new();
    for frame in frames.iter().rev() {
        // TODO: execute frame.code.debug_info.bytecode to get source file name and line for current instruction
        // let source_file =
        //     frame.code.debug_info
        //         .map(|debug| {
        //             debug.bytecode
        //         })

        let frame = frame.read().await;
        stacktrace.push_str(&format!("    at {}\n", frame.method.full_name_human_readable()));
    }
    stacktrace
}

pub fn invoke_native(frame: &mut Frame, method: Arc<Method>, args: Vec<Value>) {
    let skipped_methods = [
        "Ljava/lang/Object;->registerNatives()V"
    ];
    let full_name = method.full_name();
    if skipped_methods.contains(&full_name.as_str()) {
        eprintln!("Skipping native method execution: {}", full_name);
        return;
    }
    if method.definer.as_str() == "Lme/gulya/rust_dalvik_vm/Entrypoint;" {
        let value = args.get(0).unwrap();
        if let Value::Int(value) = value {
            let value = *value;
            let value = value as u8 as char;
            print!("{}", value);
            return;
        }
        panic!("Expected int argument for Entrypoint.writeStdout(). Got {:?} instead", value);
    }

    panic!("JNI is not implemented yet! Attempt to invoke native method {}", full_name);
}

pub fn vm_panic(message: &str) {
    eprintln!("VM PANIC: {}", message);
    std::process::exit(1);
}