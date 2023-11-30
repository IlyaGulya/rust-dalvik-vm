use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::rc::Rc;

use crate::dex::dex_file::{EncodedMethod, parse_dex_file};
use crate::dex_file::ClassDefinition;
use crate::runtime::class::Class;
use crate::runtime::class_loader::{ClassLoader, DexClassLoader, RuntimeDexFile};

#[derive(Debug)]
pub struct Runtime {
    pub class_loaders: Vec<Rc<RefCell<dyn ClassLoader>>>,
}

impl Default for Runtime {
    fn default() -> Self {
        Runtime {
            class_loaders: vec![],
        }
    }
}

impl Runtime {
    pub fn load_dex(&mut self, path: &str) {
        let raw_dex_file = crate::dex::raw_dex_file::parse_raw_dex_file(path);
        let mut opened = fs::File::open(path).unwrap();
        let dex_file = Rc::new(parse_dex_file(raw_dex_file, &mut opened));

        let classes: HashMap<String, Rc<ClassDefinition>> =
            dex_file
                .classes
                .iter().map(|c| (c.class_type.to_string(), c.clone()))
                .collect();

        self.class_loaders.push(
            Rc::new(
                RefCell::new(
                    DexClassLoader {
                        dex_file: RuntimeDexFile {
                            dex_file: dex_file.clone(),
                            classes,
                        },
                        loaded_classes: Default::default(),
                    }
                )
            )
        );
    }

    pub fn get_class(&mut self, class_name: Rc<String>) -> Option<Rc<RefCell<dyn Class>>> {
        self.class_loaders
            .iter_mut()
            .find_map(|class_loader| {
                class_loader.borrow_mut().get_class(class_name.clone()).clone()
            })
    }
}

pub trait RuntimeExt {
    fn get_class(&self, class_name: &str) -> Option<&ClassDefinition>;
}

impl RuntimeExt for DexClassLoader {
    fn get_class(&self, class_name: &str) -> Option<&ClassDefinition> {
        self.dex_file.classes
            .get(class_name)
            .map(|c| c.as_ref())
    }
}

pub trait ClassDefinitionExt {
    fn get_method(&self, method_name: &str) -> Option<&EncodedMethod>;
}

impl ClassDefinitionExt for ClassDefinition {
    fn get_method(&self, method_name: &str) -> Option<&EncodedMethod> {
        let method = self.class_data
            .as_ref()
            .map(|c| c.direct_methods.iter().find(|m| m.method.name.as_str().eq(method_name)))
            .flatten()?;

        Some(method)
    }
}