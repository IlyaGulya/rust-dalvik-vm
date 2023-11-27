use std::rc::Rc;

use crate::runtime::class::RuntimeClass;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Void,
    Default,
    Int(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    Object(Rc<RuntimeClass>),
    String(Rc<String>),
}
