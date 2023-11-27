use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;

use crate::dex::dex_file::{ClassDefinition, EncodedMethod, Field};
use crate::runtime::frame::Frame;
use crate::runtime::interpreter::Interpreter;
use crate::runtime::runtime::Runtime;
use crate::runtime::value::Value;

pub trait Class {
    fn invoke_virtual(&self, interpreter: &Interpreter, runtime: &mut Runtime, method: MethodDefinition, args: &[Value]) -> Value;
    fn invoke_direct(&self, interpreter: &Interpreter, runtime: &mut Runtime, method: MethodDefinition, args: &[Value]) -> Value;
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct MethodDefinition {
    pub name: Rc<String>,
    pub descriptor: MethodDescriptor,
}

#[derive(Debug, PartialEq)]
pub struct RuntimeClass {
    pub name: String,
    pub definition: Rc<ClassDefinition>,
    pub methods: HashMap<MethodDefinition, Rc<RuntimeMethod>>,
    pub fields: HashMap<FieldDescription, Rc<RuntimeField>>,
}

impl Class for RuntimeClass {
    fn invoke_virtual(&self, interpreter: &Interpreter, runtime: &mut Runtime, method: MethodDefinition, args: &[Value]) -> Value {
        // TODO: implement correct method lookup
        self.invoke_direct(interpreter, runtime, method, args)
    }
    fn invoke_direct(&self, interpreter: &Interpreter, runtime: &mut Runtime, method: MethodDefinition, args: &[Value]) -> Value {
        let method =
            self.methods
                .get(&method)
                // TODO: throw MethodNotFoundException
                .expect(&format!("Method {:?} not found", method));

        let code = method.method.code.as_ref()
            .expect(&format!("Attempt to invoke method {} with no code", method.name));

        let mut frame = Frame::new(code.clone());

        args.into_iter().enumerate().for_each(|(i, arg)| {
            frame.set_register(i as u8, arg.clone());
        });

        interpreter.interpret(runtime, frame)
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct FieldDescription {
    pub name: Rc<String>,
    pub descriptor: Rc<String>,
}

pub type MethodDescriptor = Rc<String>;

#[derive(Debug, PartialEq)]
pub struct RuntimeMethod {
    pub name: Rc<String>,
    pub descriptor: MethodDescriptor,
    pub method: Rc<EncodedMethod>,
    pub is_static: bool,
}

#[derive(Debug, PartialEq)]
pub struct RuntimeField {
    pub definition: Rc<Field>,
    pub is_static: bool,
    pub value: Value,
}
