use std::rc::Rc;
use std::sync::Arc;

use crate::bytecode::instructions::{Instruction, InvokeOp, InvokeOpData};
use crate::dex::dex_file::{Code, Method, Prototype};
use crate::runtime::frame::Frame;

pub fn entrypoint_frame() -> Frame {
    Frame::new(
        Arc::new(
            Method {
                definer: Arc::new("<vm entrypoint>".to_string()),
                prototype: Arc::new(Prototype {
                    shorty: Arc::new("()V".to_string()),
                    return_type: Arc::new("V".to_string()),
                    parameters: vec![],
                }),
                name: Arc::new("".to_string()),
            }
        ),
        Arc::new(Code {
            file_offset: 0,
            registers_size: 2,
            ins_size: 0,
            outs_size: 0,
            debug_info: None,
            raw_instructions: vec![],
            instructions: vec![
                Instruction::Invoke(
                    InvokeOpData {
                        method: Arc::new(Method {
                            definer: Arc::new("Lme/gulya/rust_dalvik_vm/Entrypoint;".to_string()),
                            prototype: Arc::new(Prototype {
                                shorty: Arc::new("()V".to_string()),
                                parameters: vec![],
                                return_type: Arc::new("V".to_string()),
                            }),
                            name: Arc::new("configureEnvironment".to_string()),
                        }),
                        args_registers: vec![],
                    },
                    InvokeOp::INVOKE_STATIC,
                )
            ],
            tries: vec![],
            handlers: vec![],
        }))
}
