use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;

use crate::runtime::value::Value;

pub trait Instance: Debug {
    fn get_class(&self) -> Rc<String>;
    fn get_field(&self, field: Rc<String>) -> Value;
    fn set_field(&mut self, name: Rc<String>, value: Value);
}

impl Instance for RuntimeInstance {
    fn get_class(&self) -> Rc<String> {
        return self.class.clone();
    }

    fn get_field(&self, field: Rc<String>) -> Value {
        let value = self.fields.get(&*field);
        if let Some(value) = value {
            return value.clone();
        }
        // TODO: return default values for primitives here
        return Value::Null;
    }

    fn set_field(&mut self, name: Rc<String>, value: Value) {
        self.fields.insert(name, value);
    }
}

#[derive(Debug)]
pub struct RuntimeInstance {
    pub class: Rc<String>,
    pub fields: HashMap<Rc<String>, Value>,
}

impl RuntimeInstance {
    pub fn new(class: Rc<String>) -> Self {
        RuntimeInstance {
            class,
            fields: Default::default(),
        }
    }
}