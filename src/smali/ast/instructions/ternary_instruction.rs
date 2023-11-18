use crate::smali::ast::instructions::i::FieldInvocationTarget;
use crate::smali::ast::instructions::registers::RegisterIdentifier;
use crate::smali::ast::literals::NumericLiteral;
use crate::smali::ast::types::non_void::NonVoidType;

#[derive(Debug)]
pub enum TernaryInstruction {
    InvokePolymorphic,
    InvokePolymorphicRange,
    InvokeCustom,
    InvokeCustomRange,
    InvokeConstMethodHandle,
    InvokeConstMethodType,

    InstanceOf(InstanceOfInstruction),
    NewArray(NewArrayInstruction),

    Aget(ArrayAccessInstruction),
    AgetWide(ArrayAccessInstruction),
    AgetObject(ArrayAccessInstruction),
    AgetBoolean(ArrayAccessInstruction),
    AgetByte(ArrayAccessInstruction),
    AgetChar(ArrayAccessInstruction),
    AgetShort(ArrayAccessInstruction),

    Aput(ArrayAccessInstruction),
    AputWide(ArrayAccessInstruction),
    AputObject(ArrayAccessInstruction),
    AputBoolean(ArrayAccessInstruction),
    AputByte(ArrayAccessInstruction),
    AputChar(ArrayAccessInstruction),
    AputShort(ArrayAccessInstruction),

    Iget(InstanceFieldAccessInstruction),
    IgetWide(InstanceFieldAccessInstruction),
    IgetObject(InstanceFieldAccessInstruction),
    IgetBoolean(InstanceFieldAccessInstruction),
    IgetByte(InstanceFieldAccessInstruction),
    IgetChar(InstanceFieldAccessInstruction),
    IgetShort(InstanceFieldAccessInstruction),

    Iput(InstanceFieldAccessInstruction),
    IputWide(InstanceFieldAccessInstruction),
    IputObject(InstanceFieldAccessInstruction),
    IputBoolean(InstanceFieldAccessInstruction),
    IputByte(InstanceFieldAccessInstruction),
    IputChar(InstanceFieldAccessInstruction),
    IputShort(InstanceFieldAccessInstruction),

    AddInt(BinaryOperationRegisterInstruction),
    SubInt(BinaryOperationRegisterInstruction),
    MulInt(BinaryOperationRegisterInstruction),
    DivInt(BinaryOperationRegisterInstruction),
    RemInt(BinaryOperationRegisterInstruction),
    AndInt(BinaryOperationRegisterInstruction),
    OrInt(BinaryOperationRegisterInstruction),
    XorInt(BinaryOperationRegisterInstruction),
    ShlInt(BinaryOperationRegisterInstruction),
    ShrInt(BinaryOperationRegisterInstruction),
    UshrInt(BinaryOperationRegisterInstruction),
    RsubInt(BinaryOperationRegisterInstruction),

    AddLong(BinaryOperationRegisterInstruction),
    SubLong(BinaryOperationRegisterInstruction),
    MulLong(BinaryOperationRegisterInstruction),
    DivLong(BinaryOperationRegisterInstruction),
    RemLong(BinaryOperationRegisterInstruction),
    AndLong(BinaryOperationRegisterInstruction),
    OrLong(BinaryOperationRegisterInstruction),
    XorLong(BinaryOperationRegisterInstruction),
    ShlLong(BinaryOperationRegisterInstruction),
    ShrLong(BinaryOperationRegisterInstruction),
    UshrLong(BinaryOperationRegisterInstruction),

    AddFloat(BinaryOperationRegisterInstruction),
    SubFloat(BinaryOperationRegisterInstruction),
    MulFloat(BinaryOperationRegisterInstruction),
    DivFloat(BinaryOperationRegisterInstruction),
    RemFloat(BinaryOperationRegisterInstruction),

    AddDouble(BinaryOperationRegisterInstruction),
    SubDouble(BinaryOperationRegisterInstruction),
    MulDouble(BinaryOperationRegisterInstruction),
    DivDouble(BinaryOperationRegisterInstruction),
    RemDouble(BinaryOperationRegisterInstruction),

    AddIntLit16(BinaryOperationLiteralInstruction),
    MulIntLit16(BinaryOperationLiteralInstruction),
    DivIntLit16(BinaryOperationLiteralInstruction),
    RemIntLit16(BinaryOperationLiteralInstruction),
    AndIntLit16(BinaryOperationLiteralInstruction),
    OrIntLit16(BinaryOperationLiteralInstruction),
    XorIntLit16(BinaryOperationLiteralInstruction),

    AddIntLit8(BinaryOperationLiteral8Instruction),
    RsubIntLit8(BinaryOperationLiteral8Instruction),
    MulIntLit8(BinaryOperationLiteral8Instruction),
    DivIntLit8(BinaryOperationLiteral8Instruction),
    RemIntLit8(BinaryOperationLiteral8Instruction),
    AndIntLit8(BinaryOperationLiteral8Instruction),
    OrIntLit8(BinaryOperationLiteral8Instruction),
    XorIntLit8(BinaryOperationLiteral8Instruction),
    ShlIntLit8(BinaryOperationLiteral8Instruction),
    ShrIntLit8(BinaryOperationLiteral8Instruction),
    UshrIntLit8(BinaryOperationLiteral8Instruction),
}

#[derive(Debug)]
pub struct InstanceOfInstruction {
    pub target: RegisterIdentifier,
    pub instance: RegisterIdentifier,
    pub check_type: NonVoidType,
}

#[derive(Debug)]
pub struct NewArrayInstruction {
    pub target: RegisterIdentifier,
    pub size: RegisterIdentifier,
    pub element_type: NonVoidType,
}


#[derive(Debug)]
pub struct ArrayAccessInstruction {
    pub target: RegisterIdentifier,
    pub array: RegisterIdentifier,
    pub index: RegisterIdentifier,
}

#[derive(Debug)]
pub struct InstanceFieldAccessInstruction {
    pub target: RegisterIdentifier,
    pub instance: RegisterIdentifier,
    pub field: FieldInvocationTarget, // Assuming you have a structure for this
}

#[derive(Debug)]
pub struct BinaryOperationLiteral8Instruction {
    pub target: RegisterIdentifier,
    pub left: RegisterIdentifier,
    pub literal: NumericLiteral,
}

#[derive(Debug)]
pub struct BinaryOperationLiteralInstruction {
    pub target: RegisterIdentifier,
    pub left: RegisterIdentifier,
    pub literal: NumericLiteral,
}

#[derive(Debug)]
pub struct BinaryOperationRegisterInstruction {
    pub target: RegisterIdentifier,
    pub left: RegisterIdentifier,
    pub right: RegisterIdentifier,
}
