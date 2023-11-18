use crate::smali::ast::instructions::registers::{RegisterIdentifier, RegisterList, RegisterRange};
use crate::smali::ast::literals::{NumericLiteral, StringLiteral};
use crate::smali::ast::types::non_void::NonVoidType;
use crate::smali::ast::types::reference::ReferenceType;
use crate::smali::ast::types::reference_or_array::ReferenceOrArrayType;

#[derive(Debug)]
pub enum BinaryInstruction {
    FilledNewArrayRange(FilledNewArrayRangeInstruction),
    FilledNewArray(FilledNewArrayInstruction),
    FillArrayData(FillArrayDataInstruction),
    ArrayLength(ArrayLengthInstruction),

    PackedSwitch(SwitchInstruction),
    SparseSwitch(SwitchInstruction),

    NewInstance(NewInstanceInstruction),
    CheckCast(CheckCastInstruction),

    Move(MoveInstruction),
    MoveFrom16(MoveInstruction),
    Move16(MoveInstruction),
    MoveWide(MoveInstruction),
    MoveWideFrom16(MoveInstruction),
    MoveWide16(MoveInstruction),
    MoveObject(MoveInstruction),
    MoveObjectFrom16(MoveInstruction),
    MoveObject16(MoveInstruction),

    Const(ConstInstruction),
    Const4(ConstInstruction),
    Const16(ConstInstruction),
    ConstHigh16(ConstInstruction),
    ConstWide16(ConstInstruction),
    ConstWide32(ConstInstruction),
    ConstWide(ConstInstruction),
    ConstWideHigh16(ConstInstruction),
    ConstString(ConstStringInstruction),
    ConstStringJumbo(ConstStringInstruction),
    ConstClass(ConstClassInstruction),

    // Static field access
    SGet(BinaryOperationInstruction),
    SGetWide(BinaryOperationInstruction),
    SGetObject(BinaryOperationInstruction),
    SGetBoolean(BinaryOperationInstruction),
    SGetByte(BinaryOperationInstruction),
    SGetChar(BinaryOperationInstruction),
    SGetShort(BinaryOperationInstruction),

    // Static field modification
    SPut(BinaryOperationInstruction),
    SPutWide(BinaryOperationInstruction),
    SPutObject(BinaryOperationInstruction),
    SPutBoolean(BinaryOperationInstruction),
    SPutByte(BinaryOperationInstruction),
    SPutChar(BinaryOperationInstruction),
    SPutShort(BinaryOperationInstruction),

    // Method invocation
    InvokeVirtual(BinaryOperationInstruction),
    InvokeSuper(BinaryOperationInstruction),
    InvokeDirect(BinaryOperationInstruction),
    InvokeStatic(BinaryOperationInstruction),
    InvokeInterface(BinaryOperationInstruction),

    // Method invocation with register ranges
    InvokeVirtualRange(BinaryOperationInstruction),
    InvokeSuperRange(BinaryOperationInstruction),
    InvokeDirectRange(BinaryOperationInstruction),
    InvokeStaticRange(BinaryOperationInstruction),
    InvokeInterfaceRange(BinaryOperationInstruction),

    // Binary instructions involving type conversion
    IntToLong(BinaryOperationInstruction),
    IntToFloat(BinaryOperationInstruction),
    IntToDouble(BinaryOperationInstruction),
    LongToInt(BinaryOperationInstruction),
    LongToFloat(BinaryOperationInstruction),
    LongToDouble(BinaryOperationInstruction),
    FloatToInt(BinaryOperationInstruction),
    FloatToLong(BinaryOperationInstruction),
    FloatToDouble(BinaryOperationInstruction),
    DoubleToInt(BinaryOperationInstruction),
    DoubleToLong(BinaryOperationInstruction),
    DoubleToFloat(BinaryOperationInstruction),
    IntToByte(BinaryOperationInstruction),
    IntToChar(BinaryOperationInstruction),
    IntToShort(BinaryOperationInstruction),

    // Binary instructions for conditional branches based on zero comparison
    IfEqz(ConditionalBranchInstruction),
    IfNez(ConditionalBranchInstruction),
    IfLtz(ConditionalBranchInstruction),
    IfGez(ConditionalBranchInstruction),
    IfGtz(ConditionalBranchInstruction),
    IfLez(ConditionalBranchInstruction),

    // Binary instructions for negation and bitwise NOT operations
    NegInt(UnaryOperationInstruction),
    NotInt(UnaryOperationInstruction),
    NegLong(UnaryOperationInstruction),
    NotLong(UnaryOperationInstruction),
    NegFloat(UnaryOperationInstruction),
    NegDouble(UnaryOperationInstruction),

    // Binary instructions for conditional branches based on register comparison
    IfEq(ConditionalBranchBinaryInstruction),
    IfNe(ConditionalBranchBinaryInstruction),
    IfLt(ConditionalBranchBinaryInstruction),
    IfGe(ConditionalBranchBinaryInstruction),
    IfGt(ConditionalBranchBinaryInstruction),
    IfLe(ConditionalBranchBinaryInstruction),

    // Binary instructions that operate on two address registers
    AddInt2Addr(BinaryOperationInstruction),
    SubInt2Addr(BinaryOperationInstruction),
    MulInt2Addr(BinaryOperationInstruction),
    DivInt2Addr(BinaryOperationInstruction),
    RemInt2Addr(BinaryOperationInstruction),
    AndInt2Addr(BinaryOperationInstruction),
    OrInt2Addr(BinaryOperationInstruction),
    XorInt2Addr(BinaryOperationInstruction),
    ShlInt2Addr(BinaryOperationInstruction),
    ShrInt2Addr(BinaryOperationInstruction),
    UshrInt2Addr(BinaryOperationInstruction),
    AddLong2Addr(BinaryOperationInstruction),
    SubLong2Addr(BinaryOperationInstruction),
    MulLong2Addr(BinaryOperationInstruction),
    DivLong2Addr(BinaryOperationInstruction),
    RemLong2Addr(BinaryOperationInstruction),
    AndLong2Addr(BinaryOperationInstruction),
    OrLong2Addr(BinaryOperationInstruction),
    XorLong2Addr(BinaryOperationInstruction),
    ShlLong2Addr(BinaryOperationInstruction),
    ShrLong2Addr(BinaryOperationInstruction),
    UshrLong2Addr(BinaryOperationInstruction),
    AddFloat2Addr(BinaryOperationInstruction),
    SubFloat2Addr(BinaryOperationInstruction),
    MulFloat2Addr(BinaryOperationInstruction),
    DivFloat2Addr(BinaryOperationInstruction),
    RemFloat2Addr(BinaryOperationInstruction),
    AddDouble2Addr(BinaryOperationInstruction),
    SubDouble2Addr(BinaryOperationInstruction),
    MulDouble2Addr(BinaryOperationInstruction),
    DivDouble2Addr(BinaryOperationInstruction),
    RemDouble2Addr(BinaryOperationInstruction),

    // Comparison instructions
    CmplFloat(ComparisonInstruction),
    CmpgFloat(ComparisonInstruction),
    CmplDouble(ComparisonInstruction),
    CmpgDouble(ComparisonInstruction),
    CmpLong(ComparisonInstruction),

}

#[derive(Debug)]
pub struct FilledNewArrayRangeInstruction {
    pub register_range: RegisterRange,
    pub element_type: NonVoidType,
}

#[derive(Debug)]
pub struct FilledNewArrayInstruction {
    pub registers: RegisterList,
    pub element_type: NonVoidType,
}

#[derive(Debug)]
pub struct FillArrayDataInstruction {
    pub target: RegisterIdentifier,
    pub data_label: String,
}

#[derive(Debug)]
pub struct ArrayLengthInstruction {
    pub target: RegisterIdentifier,
    pub array: RegisterIdentifier,
}

#[derive(Debug)]
pub struct SwitchInstruction {
    pub register: RegisterIdentifier,
    pub label: String,
}

#[derive(Debug)]
pub struct NewInstanceInstruction {
    pub target: RegisterIdentifier,
    pub type_reference: ReferenceType,
}

#[derive(Debug)]
pub struct CheckCastInstruction {
    pub target: RegisterIdentifier,
    pub type_reference: ReferenceOrArrayType,
}

#[derive(Debug)]
pub struct MoveInstruction {
    pub left: RegisterIdentifier,
    pub right: RegisterIdentifier,
}

#[derive(Debug)]
pub struct ConstInstruction {
    pub register: RegisterIdentifier,
    pub value: NumericLiteral,
}

#[derive(Debug)]
pub struct ConstStringInstruction {
    pub register: RegisterIdentifier,
    pub value: StringLiteral,
}

#[derive(Debug)]
pub struct ConstClassInstruction {
    pub register: RegisterIdentifier,
    pub type_reference: ReferenceOrArrayType,
}

#[derive(Debug)]
pub struct ConditionalBranchInstruction {
    pub register: RegisterIdentifier,
    pub label: String,
}

#[derive(Debug)]
pub struct UnaryOperationInstruction {
    pub source: RegisterIdentifier,
    pub destination: RegisterIdentifier,
}

#[derive(Debug)]
pub struct ConditionalBranchBinaryInstruction {
    pub left: RegisterIdentifier,
    pub right: RegisterIdentifier,
    pub label: String,
}

#[derive(Debug)]
pub struct BinaryOperationInstruction {
    pub left: RegisterIdentifier,
    // Acts as both source and destination
    pub right: RegisterIdentifier, // Acts as the second operand
}

#[derive(Debug)]
pub struct ComparisonInstruction {
    pub target: RegisterIdentifier,
    pub left: RegisterIdentifier,
    pub right: RegisterIdentifier,
}
