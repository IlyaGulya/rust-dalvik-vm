use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;

use crate::dex::dex_file::{ClassDefinition, EncodedMethod, Field, Method};
use crate::runtime::frame::Frame;
use crate::runtime::instance::{Instance, RuntimeInstance};
use crate::runtime::value::Value;

pub enum InvokeResult {
    Native(Rc<Method>),
    Frame(Frame),
}

pub trait Class: Debug {
    fn new_instance(&self) -> Rc<RefCell<dyn Instance>>;
    fn invoke_virtual(&self, method: MethodDefinition, args: &[Value]) -> Frame;
    fn invoke_static(&self, method: MethodDefinition, args: &[Value]) -> InvokeResult;
    fn invoke_direct(&self, method: MethodDefinition, args: &[Value]) -> Frame;
    fn get_initializer(&self) -> Option<Frame>;
    fn is_initialized(&self) -> bool;
    fn set_initialized(&mut self);
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct MethodDefinition {
    pub name: Rc<String>,
    pub descriptor: MethodDescriptor,
}

impl MethodDefinition {
    pub fn class_initializer() -> MethodDefinition {
        MethodDefinition {
            name: Rc::new("<clinit>".to_string()),
            descriptor: Rc::new("()V".to_string()),
        }
    }

    pub fn name_with_descriptor(&self) -> String {
        format!("{}{}", self.name, self.descriptor)
    }
}

#[derive(Debug)]
pub struct RuntimeClass {
    pub name: Rc<String>,
    pub definition: Rc<ClassDefinition>,
    pub methods: HashMap<MethodDefinition, Rc<RuntimeMethod>>,
    pub fields: HashMap<Rc<String>, Rc<RuntimeField>>,
    pub static_field_values: HashMap<Rc<String>, Value>,
    pub initialized: bool,
}

impl Class for RuntimeClass {
    fn new_instance(&self) -> Rc<RefCell<dyn Instance>> {
        let instance = RuntimeInstance::new(self.name.clone());
        return Rc::new(RefCell::new(instance));
    }
    fn invoke_virtual(&self, method: MethodDefinition, args: &[Value]) -> Frame {
        // TODO: implement correct method lookup
        self.invoke_direct(method, args)
    }
    fn invoke_static(&self, method: MethodDefinition, args: &[Value]) -> InvokeResult {
        if let Some(method) = self.methods.get(&method) {
            if !method.is_static {
                panic!("Attempt to invoke non-static method as static");
            }
            if method.is_native {
                return InvokeResult::Native(method.method_def.clone());
            }
        }
        InvokeResult::Frame(self.invoke_direct(method, args))
    }
    fn invoke_direct(&self, method: MethodDefinition, args: &[Value]) -> Frame {
        let method =
            self.methods
                .get(&method)
                // TODO: throw MethodNotFoundException
                .expect(&format!("Method {:?} not found", method));

        let code = method.method.code.as_ref()
            .expect(&format!("Attempt to invoke method {} with no code", method.name));

        let mut frame = Frame::new(method.method_def.clone(), code.clone());

        args.into_iter().enumerate().for_each(|(i, arg)| {
            frame.set_register(i as u8, arg.clone());
        });

        frame
    }

    fn get_initializer(&self) -> Option<Frame> {
        self.methods.get(&MethodDefinition::class_initializer())
            .map(|method| {
                let code = method.method.code.as_ref()
                    .expect(&format!("Attempt to invoke method {} with no code", method.name));

                Frame::new(method.method_def.clone(), code.clone())
            })
    }

    fn is_initialized(&self) -> bool {
        self.initialized
    }

    fn set_initialized(&mut self) {
        self.initialized = true;
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
    pub method_def: Rc<Method>,
    pub name: Rc<String>,
    pub descriptor: MethodDescriptor,
    pub method: Rc<EncodedMethod>,
    pub is_static: bool,
    pub is_native: bool,
}

#[derive(Debug)]
pub struct RuntimeField {
    pub definition: Rc<Field>,
    pub is_static: bool,
}
