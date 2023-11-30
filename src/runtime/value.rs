use std::cell::RefCell;
use std::rc::Rc;

use crate::runtime::instance::Instance;

#[derive(Debug, Clone)]
pub struct ArrayInstance {
    pub type_: Rc<String>,
    pub array: Vec<Value>,
}

#[derive(Debug, Clone)]
pub enum Value {
    Null,
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    Object(Rc<RefCell<dyn Instance>>),
    Array(Rc<RefCell<ArrayInstance>>),
    String(Rc<String>),
}