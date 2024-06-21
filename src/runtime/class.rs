use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

use tokio::sync::RwLock;

use crate::dex::dex_file::{ClassDefinition, EncodedMethod, Field, Method};
use crate::runtime::frame::Frame;
use crate::runtime::instance::{Instance, RuntimeInstance};
use crate::runtime::value::Value;

pub enum InvokeResult {
    Native(Arc<Method>),
    Frame(Frame),
}

pub trait Class: Debug + Send {
    fn new_instance(&self) -> Arc<RwLock<dyn Instance>>;
    fn invoke_virtual(&self, method: MethodDefinition, args: &[Value]) -> Frame;
    fn invoke_static(&self, method: MethodDefinition, args: &[Value]) -> InvokeResult;
    fn invoke_direct(&self, method: MethodDefinition, args: &[Value]) -> Frame;
    fn get_initializer(&self) -> Option<Frame>;
    fn is_initialized(&self) -> bool;
    fn set_initialized(&mut self);
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct MethodDefinition {
    pub name: Arc<String>,
    pub descriptor: MethodDescriptor,
}

impl MethodDefinition {
    pub fn class_initializer() -> MethodDefinition {
        MethodDefinition {
            name: Arc::new("<clinit>".to_string()),
            descriptor: Arc::new("()V".to_string()),
        }
    }

    pub fn name_with_descriptor(&self) -> String {
        format!("{}{}", self.name, self.descriptor)
    }
}

#[derive(Debug)]
pub struct RuntimeClass {
    pub name: Arc<String>,
    pub definition: Arc<ClassDefinition>,
    pub methods: HashMap<MethodDefinition, Arc<RuntimeMethod>>,
    pub fields: HashMap<Arc<String>, Arc<RuntimeField>>,
    pub static_field_values: HashMap<Arc<String>, Value>,
    pub initialized: bool,
}

impl Class for RuntimeClass {
    fn new_instance(&self) -> Arc<RwLock<(dyn Instance + 'static)>> {
        let instance = RuntimeInstance::new(self.name.clone());
        return Arc::new(RwLock::new(instance));
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
    pub name: Arc<String>,
    pub descriptor: Arc<String>,
}

pub type MethodDescriptor = Arc<String>;

#[derive(Debug, PartialEq)]
pub struct RuntimeMethod {
    pub method_def: Arc<Method>,
    pub name: Arc<String>,
    pub descriptor: MethodDescriptor,
    pub method: Arc<EncodedMethod>,
    pub is_static: bool,
    pub is_native: bool,
}

#[derive(Debug)]
pub struct RuntimeField {
    pub definition: Arc<Field>,
    pub is_static: bool,
}
