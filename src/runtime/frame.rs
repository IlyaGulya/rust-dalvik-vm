use std::cell::RefCell;
use std::rc::Rc;

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
pub struct Frame {
    pub method: Rc<Method>,
    pub code: Rc<Code>,
    pub registers: Vec<Register>,
    pub last_return_value: Option<Value>,
    pub current_instruction: usize,
}

impl Frame {
    pub(crate) fn public_static_main(main_class_name: &String) -> Frame {
        Frame::new(
            Rc::new(
                Method {
                    definer: Rc::new("<dummy>".to_string()),
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
                                definer: Rc::new(main_class_name.clone()),
                                prototype: Rc::new(Prototype {
                                    shorty: Rc::new("([Ljava/lang/String;)V".to_string()),
                                    parameters: vec![Rc::new("[Ljava/lang/String;".to_string())],
                                    return_type: Rc::new("V".to_string()),
                                }),
                                name: Rc::new("main".to_string()),
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
}

impl Frame {
    pub fn is_completed(&self) -> bool {
        return self.current_instruction == self.code.instructions.len();
    }
}

impl Frame {
    pub fn new<'a>(method: Rc<Method>, code: Rc<Code>) -> Frame {
        let registers_size = code.registers_size as usize;
        return Frame {
            method,
            code,
            registers: vec![Register::default(); registers_size],
            last_return_value: None,
            current_instruction: 0,
        };
    }
    pub fn get_register(&self, idx: u8) -> &Value {
        return &self.registers[idx as usize].value;
    }

    pub fn set_register(&mut self, idx: u8, value: Value) {
        self.registers[idx as usize].value = value;
    }

    pub fn get_int_register(&self, idx: u8) -> Option<i32> {
        let value = &self.registers[idx as usize].value;
        if let Value::Int(int) = value {
            return Some(*int);
        }
        println!("Attempt to get int register from non-int register: {:?}", value);
        None
    }

    pub fn get_object_register(&self, idx: u8) -> Option<Rc<RefCell<dyn Instance>>> {
        let value = &self.registers[idx as usize].value;
        if let Value::Object(object) = value {
            return Some(object.clone());
        }
        println!("Attempt to get object register from non-object register: {:?}", value);
        None
    }
}