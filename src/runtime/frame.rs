use std::rc::Rc;

use crate::dex::dex_file::Code;
use crate::runtime::value::Value;

#[derive(Debug, Clone)]
pub struct Register {
    pub value: Value,
}

impl Default for Register {
    fn default() -> Self {
        return Register {
            value: Value::Void
        };
    }
}

#[derive(Debug, Clone)]
pub struct Frame {
    pub code: Rc<Code>,
    pub registers: Vec<Register>,
    pub last_return_value: Option<Value>,
}

impl Frame {
    pub fn new<'a>(code: Rc<Code>) -> Frame {
        let registers_size = code.registers_size as usize;
        return Frame {
            code,
            registers: vec![Register::default(); registers_size],
            last_return_value: None,
        };
    }
    pub fn get_register(&self, idx: u8) -> &Value {
        return &self.registers[idx as usize].value;
    }

    pub fn set_register(&mut self, idx: u8, value: Value) {
        self.registers[idx as usize].value = value;
    }
}