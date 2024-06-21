use std::sync::Arc;

use tokio::sync::RwLock;

use crate::bytecode::instructions::{Instruction, InvokeOp, InvokeOpData};
use crate::dex::dex_file::{Code, Method, Prototype};
use crate::runtime::instance::Instance;
use crate::runtime::value::Value;

#[derive(Debug, Clone)]
pub struct Register {
    pub value: Value,
}

impl Default for Register {
    fn default() -> Self {
        return Register {
            value: Value::Null,
        };
    }
}

#[derive(Debug, Clone)]
pub struct FrameState {
    pub registers: Vec<Register>,
    pub last_return_value: Option<Value>,
    pub current_instruction: usize,
}

impl FrameState {
    fn new(registers_size: usize) -> FrameState {
        return FrameState {
            registers: vec![Register::default(); registers_size],
            last_return_value: None,
            current_instruction: 0,
        };
    }
}

#[derive(Debug, Clone)]
pub struct Frame {
    pub method: Arc<Method>,
    pub code: Arc<Code>,
    pub current_frame_state: Arc<RwLock<FrameState>>,
    // Debug only
    pub state_history: Vec<Arc<RwLock<FrameState>>>,
}

impl Frame {
    pub fn new<'a>(method: Arc<Method>, code: Arc<Code>) -> Frame {
        let registers_size = code.registers_size as usize;
        return Frame {
            method,
            code,
            current_frame_state: Arc::new(RwLock::new(FrameState::new(registers_size))),
            state_history: vec![],
        };
    }

    pub(crate) fn public_static_main(main_class_name: &String) -> Frame {
        Frame::new(
            Arc::new(
                Method {
                    definer: Arc::new("<dummy>".to_string()),
                    prototype: Arc::new(Prototype {
                        shorty: Arc::new("()V".to_string()),
                        return_type: Arc::new("V".to_string()),
                        parameters: vec![],
                    }),
                    name: Arc::new("".to_string()),
                }
            ),
            Arc::new(Code {
                file_offset: 0,
                registers_size: 2,
                ins_size: 0,
                outs_size: 0,
                debug_info: None,
                raw_instructions: vec![],
                instructions: vec![
                    Instruction::Invoke(
                        InvokeOpData {
                            method: Arc::new(Method {
                                definer: Arc::new(main_class_name.clone()),
                                prototype: Arc::new(Prototype {
                                    shorty: Arc::new("([Ljava/lang/String;)V".to_string()),
                                    parameters: vec![Arc::new("[Ljava/lang/String;".to_string())],
                                    return_type: Arc::new("V".to_string()),
                                }),
                                name: Arc::new("main".to_string()),
                            }),
                            args_registers: vec![], // TOOO: args to main are not supperted for now
                        },
                        InvokeOp::INVOKE_STATIC,
                    )
                ],
                tries: vec![],
                handlers: vec![],
            }))
    }

    pub async fn move_to_next_instruction(&mut self) {
        self.maybe_push_to_history().await;
        self.current_frame_state.write().await.current_instruction += 1;
    }

    pub async fn current_instruction(&self) -> &Instruction {
        return &self.code.instructions[self.current_frame_state.write().await.current_instruction];
    }

    pub async fn offset_instruction(&mut self, offset: isize) {
        self.maybe_push_to_history().await;
        let mut state = self.current_frame_state.write().await;
        let current_instruction = state.current_instruction as isize;
        state.current_instruction = (current_instruction + offset) as usize;
    }

    pub async fn is_completed(&self) -> bool {
        return self.current_frame_state.read().await.current_instruction == self.code.instructions.len();
    }

    async fn maybe_push_to_history(&mut self) {
        let state = self.current_frame_state.read().await.clone();
        self.state_history.push(Arc::new(RwLock::new(state)))
    }

    pub(crate) fn has_history(&self) -> bool {
        return self.state_history.len() > 0;
    }

    pub(crate) fn pop_history(&mut self) {
        if let Some(state) = self.state_history.pop() {
            self.current_frame_state = state;
        }
    }

    pub async fn get_register(&self, idx: u8) -> Value {
        return self.current_frame_state.read().await.registers[idx as usize].value.clone();
    }

    pub async fn set_register(&mut self, idx: u8, value: Value) {
        self.maybe_push_to_history().await;
        self.current_frame_state.write().await.registers[idx as usize].value = value;
    }

    pub async fn get_int_register(&self, idx: u8) -> Option<i32> {
        let value = &self.current_frame_state.read().await.registers[idx as usize].value;
        if let Value::Int(int) = value {
            return Some(*int);
        }
        println!("Attempt to get int register from non-int register: {:?}", value);
        None
    }

    pub async fn get_object_register(&self, idx: u8) -> Option<Arc<RwLock<dyn Instance>>> {
        let value = &self.current_frame_state.read().await.registers[idx as usize].value;
        if let Value::Object(object) = value {
            return Some(object.clone());
        }
        println!("Attempt to get object register from non-object register: {:?}", value);
        None
    }
}