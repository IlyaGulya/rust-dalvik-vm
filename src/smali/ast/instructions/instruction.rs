use crate::smali::ast::instructions::binary_instruction::BinaryInstruction;
use crate::smali::ast::instructions::registers::RegisterIdentifier;
use crate::smali::ast::instructions::ternary_instruction::TernaryInstruction;

// The Instruction enum can encompass all the different types of instructions.
#[derive(Debug)]
pub enum Instruction {
    Ternary(TernaryInstruction),
    Binary(BinaryInstruction),

    ReturnVoid,
    Nop,

    Goto(GotoInstruction),
    Goto16(GotoInstruction),
    Goto32(GotoInstruction),

    MoveResult(MoveResultInstruction),
    MoveResultWide(MoveResultInstruction),
    MoveResultObject(MoveResultInstruction),
    MoveException(MoveExceptionInstruction),

    Return(ReturnInstruction),
    ReturnWide(ReturnInstruction),
    ReturnObject(ReturnInstruction),

    MonitorEnter(MonitorEnterInstruction),
    MonitorExit(MonitorEnterInstruction),
    Throw(ReturnInstruction),
}

#[derive(Debug)]
pub struct GotoInstruction {
    pub label: String,
}

#[derive(Debug)]
pub struct MoveResultInstruction {
    pub register: RegisterIdentifier,
}

#[derive(Debug)]
pub struct MoveExceptionInstruction {
    pub register: RegisterIdentifier,
}

#[derive(Debug)]
pub struct ReturnInstruction {
    pub register: RegisterIdentifier,
}

#[derive(Debug)]
pub struct MonitorEnterInstruction {
    pub register: RegisterIdentifier,
}
