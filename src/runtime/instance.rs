use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;
use std::sync::Arc;

use crate::runtime::value::Value;

pub trait Instance: Debug + Send + Sync {
    fn get_class(&self) -> Arc<String>;
    fn get_field(&self, field: Arc<String>) -> Value;
    fn set_field(&mut self, name: Arc<String>, value: Value);
}

unsafe impl Send for RuntimeInstance {}
unsafe impl Sync for RuntimeInstance {}

impl Instance for RuntimeInstance {
    fn get_class(&self) -> Arc<String> {
        return self.class.clone();
    }

    fn get_field(&self, field: Arc<String>) -> Value {
        let value = self.fields.get(&*field);
        if let Some(value) = value {
            return value.clone();
        }
        // TODO: return default values for primitives here
        return Value::Null;
    }

    fn set_field(&mut self, name: Arc<String>, value: Value) {
        self.fields.insert(name, value);
    }
}

#[derive(Debug)]
pub struct RuntimeInstance {
    pub class: Arc<String>,
    pub fields: HashMap<Arc<String>, Value>,
}

impl RuntimeInstance {
    pub fn new(class: Arc<String>) -> Self {
        RuntimeInstance {
            class,
            fields: Default::default(),
        }
    }
}