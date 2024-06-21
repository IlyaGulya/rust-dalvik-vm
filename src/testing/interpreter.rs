use std::fs::File;
use std::io::Write;
use std::rc::Rc;
use std::sync::Arc;

use cmd_lib::run_cmd;
use dex;
use indoc::formatdoc;

use crate::runtime::class::MethodDefinition;
use crate::runtime::frame::Frame;
use crate::runtime::runtime::Runtime;

pub async fn get_method_frame(smali: &str) -> Frame {
    let path = create_dex(smali);

    let mut runtime = Runtime::default();
    runtime.load_dex(path);
    let frame =
        runtime
            .get_class(Arc::new("LTest;".to_string())).await.expect("Could not find LTest; class in tmp_test.dex")
            .blocking_read()
            .invoke_direct(MethodDefinition {
                name: Arc::new("main".to_string()),
                descriptor: Arc::new("([Ljava/lang/String;)V".to_string()),
            }, &vec![]);

    frame
}

pub fn create_dex(smali: &str) -> &'static str {
    let string = formatdoc! {"\
        .class public LTest;
        .super Ljava/lang/Object;
        .method public static main([Ljava/lang/String;)V

        #smali from test
        {}
        #/smali from test

        .end method
    ",
    smali};
    File::create("toolkit/tmp.smali")
        .expect("Unable to open temp smali file")
        .write_all(string.as_bytes())
        .expect("Unable to write temp smali file");
    run_cmd!(
        cd toolkit;
        smali a tmp.smali -o tmp_test.dex
    ).expect("Unable to d8 temp smali file");

    "./toolkit/tmp_test.dex"
}