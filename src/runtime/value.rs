use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::runtime::instance::Instance;

#[derive(Debug, Clone)]
pub struct ArrayInstance {
    pub type_: Arc<String>,
    pub array: Vec<Value>,
}

#[derive(Debug, Clone)]
pub enum Value {
    Null,
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    Object(Arc<RwLock<dyn Instance>>),
    Array(Arc<RwLock<ArrayInstance>>),
    String(Arc<String>),
}