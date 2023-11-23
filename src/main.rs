use std::cell::RefCell;
use std::io::Read;
use std::os::unix::fs::MetadataExt;
use std::rc::Rc;

use buffered_reader::BufferedReader;
use byteorder::{ByteOrder, ReadBytesExt};

use crate::dex::dex_file;
use crate::runtime::class::{Class, MethodDefinition};
use crate::runtime::interpreter::Interpreter;
use crate::runtime::runtime::{ClassDefinitionExt, Runtime, RuntimeExt};

mod dex;
mod bytecode;
mod runtime;


fn main() {
    let mut runtime = Runtime::default();

    // runtime.load_dex("toolkit/runtime.dex");
    runtime.load_dex("tests/vm/hello.dex");

    let main_class_name = "HelloWorld";

    let main_class_name = format!("L{};", main_class_name);
    let main_class: Rc<RefCell<dyn Class>> =
        runtime
            .get_class(&main_class_name)
            .expect(format!("Class {} not found", main_class_name).as_str());

    let interpreter = Interpreter::new();

    let main_method = MethodDefinition {
        name: Rc::new("main".to_string()),
        descriptor: Rc::new("([Ljava/lang/String;)V".to_string()),
    };

    main_class.borrow().invoke_direct(&interpreter, &mut runtime, main_method, &[]);
}






