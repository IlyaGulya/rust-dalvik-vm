use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;

use crate::dex::dex_file::{AccessFlags, ClassDefinition, DexFile};
use crate::runtime::class::{Class, FieldDescription, MethodDefinition, RuntimeClass, RuntimeField, RuntimeMethod};
use crate::runtime::value::Value;

pub trait ClassLoader: Debug {
    fn get_class(&mut self, class_name: &str) -> Option<Rc<RefCell<dyn Class>>>;
}

#[derive(Debug)]
pub struct DexClassLoader {
    pub dex_file: RuntimeDexFile,
    // TODO: use Rc<str> as key somehow instead of string cloning?
    pub loaded_classes: HashMap<String, Rc<RefCell<RuntimeClass>>>,
}

impl ClassLoader for DexClassLoader {
    fn get_class(&mut self, class_name: &str) -> Option<Rc<RefCell<dyn Class>>> {
        let class = self.loaded_classes.get(class_name);
        if let Some(class) = class {
            return Some(class.clone());
        }

        let class_definition = self.dex_file.classes.get(class_name)?;

        let class_data = &class_definition.class_data;

        let methods: HashMap<MethodDefinition, Rc<RuntimeMethod>> =
            if let Some(class_data) = class_data {
                class_data.direct_methods.iter()
                    .map(|m| {
                        let mut descriptor = String::new();

                        descriptor.push('(');

                        for parameter in &m.method.prototype.parameters {
                            descriptor.push_str(parameter);
                        }
                        descriptor.push(')');
                        descriptor.push_str(m.method.prototype.return_type.as_str());

                        let descriptor = Rc::new(descriptor);
                        (
                            MethodDefinition {
                                name: m.method.name.clone(),
                                descriptor: descriptor.clone(),
                            },
                            Rc::new(
                                RuntimeMethod {
                                    name: m.method.name.clone(),
                                    descriptor,
                                    method: m.clone(),
                                    is_static: m.access_flags.contains(AccessFlags::ACC_STATIC),
                                }
                            )
                        )
                    })
                    .collect()
            } else {
                Default::default()
            };

        let fields: HashMap<FieldDescription, Rc<RuntimeField>> =
            if let Some(class_data) = class_data {
                class_data.static_fields.iter()
                    .map(|f| {
                        (
                            FieldDescription {
                                name: f.field.name.clone(),
                                descriptor: f.field.type_.clone(),
                            },
                            Rc::new(
                                RuntimeField {
                                    definition: f.field.clone(),
                                    is_static: true,
                                    value: Value::Default, // TODO: initialize with default value
                                }
                            )
                        )
                    })
                    .collect()
            } else {
                Default::default()
            };

        let class = Rc::new(RefCell::new(
            RuntimeClass {
                name: class_name.to_string(),
                definition: class_definition.clone(),
                methods,
                fields,
            }
        ));

        self.loaded_classes.insert(class_name.to_string(), class.clone());

        Some(class)
    }
}

#[derive(Debug)]
pub struct RuntimeDexFile {
    pub dex_file: Rc<DexFile>,
    pub classes: HashMap<String, Rc<ClassDefinition>>,
}