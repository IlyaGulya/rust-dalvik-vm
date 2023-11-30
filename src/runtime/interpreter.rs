use std::cell::RefCell;
use std::rc::Rc;

use crate::bytecode::instructions::{ConstOp, IfTestZOp, InstanceFieldOp, Instruction, InvokeOp, StaticFieldOp};
use crate::runtime::class::MethodDefinition;
use crate::runtime::frame::Frame;
use crate::runtime::runtime::Runtime;
use crate::runtime::value::{ArrayInstance, Value};

pub struct Interpreter;

pub enum InterpreterSuspendReason {
    EndOfFrame,
    ReturnVoid,
    ReturnWide(Value),
    Throw,
    NewInstance {
        register: u8,
        class: Rc<String>,
    },
    Invoke {
        op: InvokeOp,
        definer: Rc<String>,
        method: MethodDefinition,
        args: Vec<Value>,
    },
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {}
    }

    pub fn interpret_frame(&self, runtime: &mut Runtime, frame: &mut Frame) -> Result<InterpreterSuspendReason, String> {
        loop {
            println!("Current instruction: {:?}", frame.code.instructions[frame.current_instruction]);
            let instruction = &frame.code.instructions[frame.current_instruction];
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
                            .get_int_register(*size_register)
                            .expect("Attempt to create array with non-int size");
                    let array = vec![Value::Null; size as usize];
                    frame.set_register(*dest_register, Value::Array(Rc::new(RefCell::new(
                        ArrayInstance {
                            type_: type_.clone(),
                            array,
                        }
                    ))));
                }
                Instruction::IfTestZ(instruction) => {
                    match instruction {
                        IfTestZOp::IF_EQZ(data) => {
                            let register = data.register_a;
                            let value = frame.get_register(register);

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
                                frame.current_instruction = (frame.current_instruction as isize + data.offset as isize) as usize;
                                continue;
                            }
                        }
                        IfTestZOp::IF_NEZ(data) => {
                            let register = data.register_a;
                            let value = frame.get_register(register);

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
                                frame.current_instruction = (frame.current_instruction as isize + data.offset as isize) as usize;
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
                            let register = frame.get_register(data.object_register);

                            let value =
                                match register {
                                    // TODO: Throw NullPointerException
                                    Value::Null => Err("Attempt to get instance field from a null pointer")?,
                                    Value::Object(object) => {
                                        object.borrow().get_field(data.field.name.clone())
                                    }
                                    _ => Err("Attempt to get instance field from a non-object register")?,
                                };
                            frame.set_register(data.register_or_pair, value)
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
                            frame.set_register(*dest_register, Value::Int(*literal as i32));
                        }
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
                        _ => Err(format!("Not implemented: {:?}", instruction))?,
                    }
                }
                Instruction::Invoke(data, instruction) => {
                    // return InterpreterSuspendReason::Invoke {
                    //     new_frame: Frame::new(data.method.code.clone())
                    // };
                    let args: Vec<Value> =
                        data.args_registers.
                            iter().map(|&idx| frame.get_register(idx).clone())
                            .collect();

                    let method = MethodDefinition {
                        name: data.method.name.clone(),
                        descriptor: Rc::new(data.method.full_descriptor()),
                    };

                    frame.current_instruction += 1; // TODO not sure if this is correct place to go to next instruction.
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
                    let value = frame.get_register(*register).clone();
                    return Ok(InterpreterSuspendReason::ReturnWide(value));
                }
                // Instruction::InvokeOp(_) => {}
                // Instruction::BinaryOpLit16(_) => {}
                instruction => panic!("Not implemented: {:?}", instruction)
            }

            if frame.is_completed() {
                return Ok(InterpreterSuspendReason::EndOfFrame);
            }
            frame.current_instruction += 1;
        }

        panic!("Method did not return anything")
    }
}