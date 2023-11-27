use std::rc::Rc;

use crate::bytecode::instructions::{ConstOp, IfTestZOp, Instruction, InvokeOp, StaticFieldOp};
use crate::runtime::class::MethodDefinition;
use crate::runtime::frame::Frame;
use crate::runtime::runtime::Runtime;
use crate::runtime::value::Value;

pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {}
    }

    pub fn interpret(&self, runtime: &mut Runtime, mut frame: Frame) -> Value {
        let mut instruction_idx = 0;

        loop {
            let instruction = &frame.code.instructions[instruction_idx];
            match instruction {
                // Instruction::NOP => {}
                // Instruction::RETURN_VOID =>
                Instruction::MONITOR_ENTER(register) => {
                    let object = frame.get_register(*register);
                    // no-op for now, since we are single-threaded yet
                    // TODO: implement when we will get threads
                }
                Instruction::IfTestZ(instruction) => {
                    match instruction {
                        // IfTestZOp::IF_EQZ(data) => {}
                        IfTestZOp::IF_NEZ(data) => {
                            let register = data.register_a;
                            let value = frame.get_register(register);
                            if Value::Int(0) == *value {
                                instruction_idx = (instruction_idx as isize + data.offset as isize) as usize;
                                continue;
                            }
                        }
                        // IfTestZOp::IF_LTZ(data) => {}
                        // IfTestZOp::IF_GEZ(data) => {}
                        // IfTestZOp::IF_GTZ(data) => {}
                        // IfTestZOp::IF_LEZ(data) => {}
                        _ => panic!("Not implemented: {:?}", instruction)
                    }
                }
                Instruction::Const(instruction) => {
                    match instruction {
                        ConstOp::CONST_STRING { dest_register: register, string } => {
                            frame.set_register(*register, Value::String(string.clone()));
                        }
                        instruction => panic!("Not implemented: {:?}", instruction)
                    }
                }
                Instruction::Static(instruction) => {
                    match instruction {
                        // StaticFieldOp::SGET(_) => {}
                        // StaticFieldOp::SPUT(_) => {}
                        StaticFieldOp::SGET_OBJECT(_) => {}
                        instruction => panic!("Not implemented: {:?}", instruction)
                    }
                }
                Instruction::Invoke(instruction) => {
                    match instruction {
                        InvokeOp::INVOKE_VIRTUAL(instruction) => {
                            let args: Vec<Value> =
                                instruction.args_registers.
                                    iter().map(|&idx| frame.get_register(idx).clone())
                                    .collect();

                            let class =
                                runtime.get_class(instruction.method.definer.as_str())
                                    // TODO: throw ClassNotFoundException
                                    .expect(format!("Class {} not found", instruction.method.definer).as_str());

                            let method = MethodDefinition {
                                name: instruction.method.name.clone(),
                                descriptor: Rc::new(instruction.method.full_descriptor()),
                            };

                            let return_value =
                                class
                                    .borrow()
                                    .invoke_direct(
                                        &self,
                                        runtime,
                                        method,
                                        args.as_slice(),
                                    );

                            frame.last_return_value = Some(return_value);
                        }
                        // InvokeOp::INVOKE_SUPER(_) => {}
                        InvokeOp::INVOKE_DIRECT(instruction) => {
                            let args: Vec<Value> =
                                instruction.args_registers.
                                    iter().map(|&idx| frame.get_register(idx).clone())
                                    .collect();

                            let class =
                                runtime.get_class(instruction.method.definer.as_str())
                                    // TODO: throw ClassNotFoundException
                                    .expect(format!("Class {} not found", instruction.method.definer).as_str());

                            let method = MethodDefinition {
                                name: instruction.method.name.clone(),
                                descriptor: Rc::new(instruction.method.full_descriptor()),
                            };

                            let return_value =
                                class
                                    .borrow()
                                    .invoke_direct(
                                        &self,
                                        runtime,
                                        method,
                                        args.as_slice(),
                                    );

                            frame.last_return_value = Some(return_value);
                        }
                        // InvokeOp::INVOKE_STATIC(_) => {}
                        // InvokeOp::INVOKE_INTERFACE(_) => {}
                        instruction => panic!("Not implemented: {:?}", instruction)
                    }
                }
                Instruction::RETURN_VOID => {
                    return Value::Void;
                }
                Instruction::RETURN_WIDE(register) => {
                    return frame.get_register(*register).clone();
                }
                // Instruction::InvokeOp(_) => {}
                // Instruction::BinaryOpLit16(_) => {}
                instruction => panic!("Not implemented: {:?}", instruction)
            }

            instruction_idx += 1;
            if frame.code.instructions.len() == instruction_idx {
                break;
            }
        }

        panic!("Method did not return anything")
    }
}