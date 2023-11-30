use std::io::Read;
use std::os::unix::fs::MetadataExt;
use std::rc::Rc;

use buffered_reader::BufferedReader;
use byteorder::{ByteOrder, ReadBytesExt};

use crate::bytecode::instructions::{Instruction, InvokeOp, InvokeOpData};
use crate::dex::dex_file;
use crate::dex::dex_file::{Code, Method, Prototype};
use crate::runtime::class::{Class, InvokeResult};
use crate::runtime::frame::Frame;
use crate::runtime::interpreter::{Interpreter, InterpreterSuspendReason};
use crate::runtime::runtime::{ClassDefinitionExt, Runtime, RuntimeExt};
use crate::runtime::value::Value;

mod dex;
mod bytecode;
mod runtime;


fn main() {
    // let buf = fs::read("toolkit/runtime.dex").unwrap();
    // let dex = dexparser::parse(&buf);
    let main_class_name = "HelloWorld";
    let main_class_name = format!("L{};", main_class_name);

    let mut runtime = Runtime::default();

    runtime.load_dex("runtime/build/entrypoint.dex");
    runtime.load_dex("toolkit/runtime.dex");
    runtime.load_dex("tests/vm/hello.dex");

    let mut frames = vec![
        Frame::public_static_main(&main_class_name),
        entrypoint_frame(),
    ];

    let interpreter = Interpreter::new();

    loop {
        let frame = frames.last_mut();
        let frame =
            if let Some(frame) = frame {
                frame
            } else {
                break;
            };

        if frame.is_completed() {
            frames.pop();
            continue;
        }
        let suspend_reason =
            match interpreter.interpret_frame(&mut runtime, frame) {
                Ok(reason) => reason,
                Err(err) => {
                    let stacktrace = get_stacktrace(&frames);
                    panic!("Error: {}\nStacktrace:\n{}", err, stacktrace);
                }
            };

        match suspend_reason {
            InterpreterSuspendReason::EndOfFrame => {
                frames.pop();
            }
            InterpreterSuspendReason::ReturnVoid => {
                frames.pop();
            }
            InterpreterSuspendReason::ReturnWide(_) => {
                panic!("Not implemented");
            }
            InterpreterSuspendReason::Throw => {}
            InterpreterSuspendReason::NewInstance { register, class } => {
                let class = runtime.get_class(class).unwrap();
                let instance = class.borrow().new_instance();
                frame.set_register(register, Value::Object(instance));
                frame.current_instruction += 1;
            }
            InterpreterSuspendReason::Invoke { op, definer, args, method } => {
                println!("Invoking method {}->{} with args {:?}", definer, method.name_with_descriptor(), args);
                let class =
                    runtime
                        .get_class(definer.clone())
                        .expect(format!("Class not found: {:?}", definer).as_str());

                let mut class = class.borrow_mut();
                if !class.is_initialized() {
                    if let Some(frame) = class.get_initializer() {
                        frames.push(frame);
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
                        frames.push(frame);
                    }
                    // InvokeOp::INVOKE_SUPER => {}
                    InvokeOp::INVOKE_DIRECT => {
                        let frame = class.invoke_direct(method, args.as_slice());
                        frames.push(frame);
                    }
                    InvokeOp::INVOKE_STATIC => {
                        let result = class.invoke_static(method, args.as_slice());
                        match result {
                            InvokeResult::Native(method) => {
                                invoke_native(frame, method.clone(), args);
                            }
                            InvokeResult::Frame(frame) => {
                                frames.push(frame);
                            }
                        }
                    }
                    // InvokeOp::INVOKE_INTERFACE => {}
                    instruction => panic!("Not implemented: {:?}", instruction)
                }
            }
        }
    }
}

fn get_stacktrace(frames: &Vec<Frame>) -> String {
    let mut stacktrace = String::new();
    for frame in frames.iter().rev() {
        // TODO: execute frame.code.debug_info.bytecode to get source file name and line for current instruction
        // let source_file =
        //     frame.code.debug_info
        //         .map(|debug| {
        //             debug.bytecode
        //         })
        stacktrace.push_str(&format!("    at {}\n", frame.method.full_name_human_readable()));
    }
    stacktrace
}

pub fn invoke_native(frame: &mut Frame, method: Rc<Method>, args: Vec<Value>) {
    let skipped_methods = [
        "Ljava/lang/Object;->registerNatives()V",
    ];
    let full_name = method.full_name();
    if skipped_methods.contains(&full_name.as_str()) {
        eprintln!("Skipping native method execution: {}", full_name);
        frame.current_instruction += 1;
        return;
    }
    if method.definer.as_str() == "Lme/gulya/rust_dalvik_vm/Entrypoint;" {
        let value = args.get(0).unwrap();
        if let Value::Int(value) = value {
            let value = *value;
            let value = value as u8 as char;
            print!("{}", value);
            frame.current_instruction += 1;
            return;
        }
        panic!("Expected int argument for Entrypoint.writeStdout()");
    }

    panic!("JNI is not implemented yet! Attempt to invoke native method {}", full_name);
}

pub fn entrypoint_frame() -> Frame {
    Frame::new(
        Rc::new(
            Method {
                definer: Rc::new("<vm entrypoint>".to_string()),
                prototype: Rc::new(Prototype {
                    shorty: Rc::new("()V".to_string()),
                    return_type: Rc::new("V".to_string()),
                    parameters: vec![],
                }),
                name: Rc::new("".to_string()),
            }
        ),
        Rc::new(Code {
            file_offset: 0,
            registers_size: 2,
            ins_size: 0,
            outs_size: 0,
            debug_info: None,
            instructions: vec![
                Instruction::Invoke(
                    InvokeOpData {
                        method: Rc::new(Method {
                            definer: Rc::new("Lme/gulya/rust_dalvik_vm/Entrypoint;".to_string()),
                            prototype: Rc::new(Prototype {
                                shorty: Rc::new("()V".to_string()),
                                parameters: vec![],
                                return_type: Rc::new("V".to_string()),
                            }),
                            name: Rc::new("configureEnvironment".to_string()),
                        }),
                        args_registers: vec![],
                    },
                    InvokeOp::INVOKE_STATIC,
                )
            ],
            tries: vec![],
            handlers: vec![],
        }))
}




