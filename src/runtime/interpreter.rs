use std::sync::Arc;

use tokio::sync::RwLock;

use crate::bytecode::instructions::{ConstOp, IfTestZOp, InstanceFieldOp, Instruction, InvokeOp, StaticFieldOp};
use crate::runtime::class::MethodDefinition;
use crate::runtime::frame::Frame;
use crate::runtime::value::{ArrayInstance, Value};
use crate::vm::main::VmState;

pub struct Interpreter;

pub enum InterpreterSuspendReason {
    EndOfFrame,
    ReturnVoid,
    ReturnWide(Value),
    Throw,
    NewInstance {
        register: u8,
        class: Arc<String>,
    },
    Invoke {
        op: InvokeOp,
        definer: Arc<String>,
        method: MethodDefinition,
        args: Vec<Value>,
    },
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {}
    }

    pub async fn interpret_current_frame(&self, vm_state: &mut VmState, current_frame: &mut Frame) -> Result<InterpreterSuspendReason, String> {
        let frame = current_frame;
            // if let Some(frame) = current_frame {
            //     frame.blocking_write()
            // } else {
            //     return Err("No current frame".to_string());
            // };
        loop {
            // vm_state.maybe_wait_for_debugger_command();
            let current_instruction = frame.current_instruction().await.clone();
            println!("Current instruction: {:?}", current_instruction);
            let instruction = &current_instruction;
            match instruction {
                Instruction::NOP => {
                    // do nothing
                }
                Instruction::MONITOR_ENTER(register) => {
                    let object = frame.get_register(*register);
                    // no-op for now, since we are single-threaded yet
                    // TODO: implement when we will get threads
                }
                Instruction::NEW_INSTANCE { register, class } => {
                    return Ok(InterpreterSuspendReason::NewInstance {
                        register: *register,
                        class: class.clone(),
                    });
                }
                Instruction::NEW_ARRAY {
                    dest_register,
                    size_register,
                    type_
                } => {
                    let size =
                        frame
                            .get_int_register(*size_register).await
                            .expect("Attempt to create array with non-int size");
                    let array = vec![Value::Null; size as usize];
                    frame.set_register(*dest_register, Value::Array(Arc::new(RwLock::new(
                        ArrayInstance {
                            type_: type_.clone(),
                            array,
                        }
                    )))).await;
                }
                Instruction::IfTestZ(instruction) => {
                    match instruction {
                        IfTestZOp::IF_EQZ(data) => {
                            let register = data.register_a;
                            let value = frame.get_register(register).await;

                            println!("Comparing {:?} with zero", value);

                            let is_zero =
                                match value {
                                    Value::Null => true,
                                    Value::Int(0) => true,
                                    Value::Long(0) => true,
                                    Value::Float(0.0) => Err("Attempt to compare float with zero".to_string())?,
                                    Value::Double(0.0) => Err("Attempt to compare double with zero".to_string())?,
                                    _ => false,
                                };

                            if is_zero {
                                frame.offset_instruction(data.offset as isize).await;
                                continue;
                            }
                        }
                        IfTestZOp::IF_NEZ(data) => {
                            let register = data.register_a;
                            let value = frame.get_register(register).await;

                            println!("Comparing {:?} with zero", value);

                            let is_zero =
                                match value {
                                    Value::Null => true,
                                    Value::Int(0) => true,
                                    Value::Long(0) => true,
                                    Value::Float(0.0) => Err("Attempt to compare float with zero".to_string())?,
                                    Value::Double(0.0) => Err("Attempt to compare double with zero".to_string())?,
                                    _ => false,
                                };

                            if !is_zero {
                                frame.offset_instruction(data.offset as isize).await;
                                continue;
                            }
                        }
                        // IfTestZOp::IF_LTZ(data) => {}
                        // IfTestZOp::IF_GEZ(data) => {}
                        // IfTestZOp::IF_GTZ(data) => {}
                        // IfTestZOp::IF_LEZ(data) => {}
                        _ => Err(format!("Not implemented: {:?}", instruction))?,
                    }
                }
                Instruction::InstanceField(instruction) => {
                    match instruction {
                        // InstanceFieldOp::IGET(data) => {}
                        // InstanceFieldOp::IGET_WIDE(data) => {}
                        InstanceFieldOp::IGET_OBJECT(data) => {
                            let register = frame.get_register(data.object_register).await;

                            let value =
                                match register {
                                    // TODO: Throw NullPointerException
                                    Value::Null => Err("Attempt to get instance field from a null pointer")?,
                                    Value::Object(object) => {
                                        object.read().await.get_field(data.field.name.clone())
                                    }
                                    _ => Err("Attempt to get instance field from a non-object register")?,
                                };
                            frame.set_register(data.register_or_pair, value).await
                        }
                        // InstanceFieldOp::IGET_BOOLEAN(data) => {}
                        // InstanceFieldOp::IGET_BYTE(data) => {}
                        // InstanceFieldOp::IGET_CHAR(data) => {}
                        // InstanceFieldOp::IGET_SHORT(_) => {}
                        // InstanceFieldOp::IPUT(_) => {}
                        // InstanceFieldOp::IPUT_WIDE(_) => {}
                        // InstanceFieldOp::IPUT_OBJECT(_) => {}
                        // InstanceFieldOp::IPUT_BOOLEAN(_) => {}
                        // InstanceFieldOp::IPUT_BYTE(_) => {}
                        // InstanceFieldOp::IPUT_CHAR(_) => {}
                        // InstanceFieldOp::IPUT_SHORT(_) => {}
                        _ => Err(format!("Not implemented: {:?}", instruction))?,
                    }
                }
                Instruction::Const(instruction) => {
                    match instruction {
                        ConstOp::CONST_4 { dest_register, literal } => {
                            frame.set_register(*dest_register, Value::Int(*literal as i32)).await;
                        }
                        ConstOp::CONST_STRING { dest_register: register, string } => {
                            frame.set_register(*register, Value::String(string.clone())).await;
                        }
                        instruction => panic!("Not implemented: {:?}", instruction)
                    }
                }
                Instruction::Static(instruction) => {
                    match instruction {
                        // StaticFieldOp::SGET(_) => {}
                        // StaticFieldOp::SPUT(_) => {}
                        StaticFieldOp::SGET_OBJECT(_) => {}
                        _ => Err(format!("Not implemented: {:?}", instruction))?,
                    }
                }
                Instruction::Invoke(data, instruction) => {
                    // return InterpreterSuspendReason::Invoke {
                    //     new_frame: Frame::new(data.method.code.clone())
                    // };

                    let mut args: Vec<Value> = vec![];
                    for register_idx in &data.args_registers {
                        let value = frame.get_register(*register_idx).await.clone();
                        args.push(value);
                    }

                    let method = MethodDefinition {
                        name: data.method.name.clone(),
                        descriptor: Arc::new(data.method.full_descriptor()),
                    };

                    return Ok(
                        InterpreterSuspendReason::Invoke {
                            op: instruction.clone(),
                            definer: data.method.definer.clone(),
                            args,
                            method,
                        }
                    );
                }
                Instruction::RETURN_VOID => {
                    return Ok(InterpreterSuspendReason::ReturnVoid);
                }
                Instruction::RETURN_WIDE(register) => {
                    let value = frame.get_register(*register).await.clone();
                    return Ok(InterpreterSuspendReason::ReturnWide(value));
                }
                // Instruction::InvokeOp(_) => {}
                // Instruction::BinaryOpLit16(_) => {}
                instruction => panic!("Not implemented: {:?}", instruction)
            }

            if frame.is_completed().await {
                return Ok(InterpreterSuspendReason::EndOfFrame);
            }
            frame.move_to_next_instruction().await;
            vm_state.maybe_notify_debugger();
        }

        panic!("Method did not return anything")
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::testing::interpreter::get_method_frame;

    #[test]
    fn internal() {
        let bytecode = get_method_frame(indoc! {"
            .method public static main([Ljava/lang/String;)V

                .registers 4

                .line 151
                const-string v0, \"Null output stream\"

                invoke-static {p1, v0}, Ljava/io/PrintStream;->requireNonNull(Ljava/lang/Object;Ljava/lang/String;)Ljava/lang/Object;

                move-result-object p1

                check-cast p1, Ljava/io/OutputStream;

                invoke-direct {p0, p2, p1}, Ljava/io/PrintStream;-><init>(ZLjava/io/OutputStream;)V

                .line 152
                return-void

            .end method
        "});

        println!()
    }
}