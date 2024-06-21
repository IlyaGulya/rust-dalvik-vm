use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

use tokio::sync::RwLock;

use crate::dex::access_flags::AccessFlags;
use crate::dex::dex_file::{ClassDefinition, DexFile};
use crate::runtime::class::{Class, MethodDefinition, RuntimeClass, RuntimeField, RuntimeMethod};
use crate::runtime::frame::Frame;

pub trait ClassLoader: Debug + Send + Sync {
    fn get_class(&mut self, class_name: Arc<String>) -> Option<Arc<RwLock<dyn Class>>>;
}

#[derive(Debug)]
pub struct DexClassLoader {
    pub dex_file: RuntimeDexFile,
    // TODO: use Rc<str> as key somehow instead of string cloning?
    pub loaded_classes: HashMap<String, Arc<RwLock<RuntimeClass>>>,
}

#[derive(Debug)]
pub enum ClassLoadResult {
    AlreadyLoaded(Arc<RwLock<dyn Class>>),
    Loaded(Arc<RwLock<dyn Class>>),
    RequiresInitialization(Arc<RwLock<dyn Class>>, Frame),
}

unsafe impl Send for DexClassLoader {}

unsafe impl Sync for DexClassLoader {}

impl ClassLoader for DexClassLoader {
    fn get_class(&mut self, class_name: Arc<String>) -> Option<Arc<RwLock<(dyn Class + 'static)>>> {
        let class = self.loaded_classes.get(class_name.as_ref());
        if let Some(class) = class {
            return Some(class.clone());
        }

        let class_definition = self.dex_file.classes.get(class_name.as_ref())?;

        let class_data = &class_definition.class_data;

        let methods: HashMap<MethodDefinition, Arc<RuntimeMethod>> =
            if let Some(class_data) = class_data {
                let direct = class_data.direct_methods.iter();
                let virt = class_data.virtual_methods.iter();
                let all_methods = direct.chain(virt);
                all_methods
                    .map(|m| {
                        let mut descriptor = String::new();

                        descriptor.push('(');

                        for parameter in &m.method.prototype.parameters {
                            descriptor.push_str(parameter);
                        }
                        descriptor.push(')');
                        descriptor.push_str(m.method.prototype.return_type.as_str());

                        let descriptor = Arc::new(descriptor);
                        (
                            MethodDefinition {
                                name: m.method.name.clone(),
                                descriptor: descriptor.clone(),
                            },
                            Arc::new(
                                RuntimeMethod {
                                    method_def: m.method.clone(),
                                    name: m.method.name.clone(),
                                    descriptor,
                                    method: m.clone(),
                                    is_static: m.access_flags.contains(AccessFlags::ACC_STATIC),
                                    is_native: m.access_flags.contains(AccessFlags::ACC_NATIVE),
                                }
                            )
                        )
                    })
                    .collect()
            } else {
                Default::default()
            };

        let fields: HashMap<Arc<String>, Arc<RuntimeField>> =
            if let Some(class_data) = class_data {
                let static_ = class_data.static_fields.iter();
                let instance = class_data.instance_fields.iter();
                let all_fields = static_.chain(instance);
                all_fields
                    .map(|f| {
                        (
                            f.field.name.clone(),
                            Arc::new(
                                RuntimeField {
                                    definition: f.field.clone(),
                                    is_static: true,
                                }
                            )
                        )
                    })
                    .collect()
            } else {
                Default::default()
            };

        let class = Arc::new(RwLock::new(
            RuntimeClass {
                name: class_name.clone(),
                definition: class_definition.clone(),
                methods,
                fields,
                static_field_values: Default::default(),
                initialized: false,
            }
        ));

        self.loaded_classes.insert(class_name.to_string(), class.clone());


        Some(class)
    }
}

#[derive(Debug)]
pub struct RuntimeDexFile {
    pub dex_file: Arc<DexFile>,
    pub classes: HashMap<String, Arc<ClassDefinition>>,
}