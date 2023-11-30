use std::rc::Rc;

use crate::dex::dex_file::{Field, Method, Prototype};

#[derive(Debug, PartialEq)]
pub struct StaticFieldOpData {
    pub register: u8,
    pub field: Rc<Field>,
}

#[derive(Debug, PartialEq)]
pub struct IfTestOpData {
    pub register_a: u8,
    pub register_b: u8,
    pub offset: i16,
}

#[derive(Debug, PartialEq)]
pub struct IfTestZOpData {
    pub register_a: u8,
    pub offset: i16,
}

#[derive(Debug, PartialEq)]
pub struct CmpOpData {
    pub destination_register: u8,
    pub register_a: u8,
    pub register_b: u8,
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum CmpOp {
    CMPL_FLOAT(CmpOpData),
    CMPG_FLOAT(CmpOpData),
    CMPL_DOUBLE(CmpOpData),
    CMPG_DOUBLE(CmpOpData),
    CMP_LONG(CmpOpData),
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum IfTestOp {
    IF_EQ(IfTestOpData),
    IF_NE(IfTestOpData),
    IF_LT(IfTestOpData),
    IF_GE(IfTestOpData),
    IF_GT(IfTestOpData),
    IF_LE(IfTestOpData),
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum IfTestZOp {
    IF_EQZ(IfTestZOpData),
    IF_NEZ(IfTestZOpData),
    IF_LTZ(IfTestZOpData),
    IF_GEZ(IfTestZOpData),
    IF_GTZ(IfTestZOpData),
    IF_LEZ(IfTestZOpData),
}


#[derive(Debug, PartialEq)]
pub struct ArrayOpData {
    pub register_or_pair: u8,
    pub array_register: u8,
    pub index_register: u8,
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum ArrayOp {
    AGET(ArrayOpData),
    AGET_WIDE(ArrayOpData),
    AGET_OBJECT(ArrayOpData),
    AGET_BOOLEAN(ArrayOpData),
    AGET_BYTE(ArrayOpData),
    AGET_CHAR(ArrayOpData),
    AGET_SHORT(ArrayOpData),
    APUT(ArrayOpData),
    APUT_WIDE(ArrayOpData),
    APUT_OBJECT(ArrayOpData),
    APUT_BOOLEAN(ArrayOpData),
    APUT_BYTE(ArrayOpData),
    APUT_CHAR(ArrayOpData),
    APUT_SHORT(ArrayOpData),
}

#[derive(Debug, PartialEq)]
pub struct InstanceFieldOpData {
    pub register_or_pair: u8,
    pub object_register: u8,
    pub field: Rc<Field>,
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum InstanceFieldOp {
    IGET(InstanceFieldOpData),
    IGET_WIDE(InstanceFieldOpData),
    IGET_OBJECT(InstanceFieldOpData),
    IGET_BOOLEAN(InstanceFieldOpData),
    IGET_BYTE(InstanceFieldOpData),
    IGET_CHAR(InstanceFieldOpData),
    IGET_SHORT(InstanceFieldOpData),
    IPUT(InstanceFieldOpData),
    IPUT_WIDE(InstanceFieldOpData),
    IPUT_OBJECT(InstanceFieldOpData),
    IPUT_BOOLEAN(InstanceFieldOpData),
    IPUT_BYTE(InstanceFieldOpData),
    IPUT_CHAR(InstanceFieldOpData),
    IPUT_SHORT(InstanceFieldOpData),
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub struct Const4Data {
    pub dest_register: u8,
    pub literal: i32,
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum ConstOp {
    CONST_4 { dest_register: u8, literal: i8 },
    CONST_16 { dest_register: u8, literal: i16 },
    CONST { dest_register: u8, literal: [u8; 4] },
    CONST_HIGH_16 { dest_register: u8, literal: i32 },
    CONST_WIDE_16 { dest_register: u8, literal: i16 },
    CONST_WIDE_32 { dest_register: u8, literal: i32 },
    CONST_WIDE { dest_register: u8, literal: i32 },
    CONST_WIDE_HIGH_16 { dest_register: u8, literal: i64 },
    CONST_STRING { dest_register: u8, string: Rc<String> },
    CONST_STRING_JUMBO { dest_register: u8, string: Rc<String> },
    CONST_CLASS { dest_register: u8, class: Rc<String> },
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum StaticFieldOp {
    SGET(StaticFieldOpData),
    SGET_WIDE(StaticFieldOpData),
    SGET_OBJECT(StaticFieldOpData),
    SGET_BOOLEAN(StaticFieldOpData),
    SGET_BYTE(StaticFieldOpData),
    SGET_CHAR(StaticFieldOpData),
    SGET_SHORT(StaticFieldOpData),
    SPUT(StaticFieldOpData),
    SPUT_WIDE(StaticFieldOpData),
    SPUT_OBJECT(StaticFieldOpData),
    SPUT_BOOLEAN(StaticFieldOpData),
    SPUT_BYTE(StaticFieldOpData),
    SPUT_CHAR(StaticFieldOpData),
    SPUT_SHORT(StaticFieldOpData),
}

#[derive(Debug, PartialEq)]
pub struct InvokeOpData {
    pub method: Rc<Method>,
    pub args_registers: Vec<u8>,
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum InvokeOp {
    INVOKE_VIRTUAL,
    INVOKE_SUPER,
    INVOKE_DIRECT,
    INVOKE_STATIC,
    INVOKE_INTERFACE,
}

#[derive(Debug, PartialEq)]
pub struct InvokeRangeOpData {
    pub method: Rc<Method>,
    pub start_register: u16,
    pub register_count: u8,
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum InvokeRangeOp {
    INVOKE_VIRTUAL(InvokeRangeOpData),
    INVOKE_SUPER(InvokeRangeOpData),
    INVOKE_DIRECT(InvokeRangeOpData),
    INVOKE_STATIC(InvokeRangeOpData),
    INVOKE_INTERFACE(InvokeRangeOpData),
}

#[derive(Debug, PartialEq)]
pub struct BinaryOpData {
    pub dest: u8,
    pub src_a: u8,
    pub src_b: u8,
}

#[derive(Debug, PartialEq)]
pub struct BinaryOp2AddrData {
    pub dest: u8,
    pub src: u8,
}


#[derive(Debug, PartialEq)]
pub struct UnaryOpData {
    pub dest: u8,
    pub src: u8,
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum UnaryOp {
    NEG_INT(UnaryOpData),
    NOT_INT(UnaryOpData),
    NEG_LONG(UnaryOpData),
    NOT_LONG(UnaryOpData),
    NEG_FLOAT(UnaryOpData),
    NEG_DOUBLE(UnaryOpData),
    INT_TO_LONG(UnaryOpData),
    INT_TO_FLOAT(UnaryOpData),
    INT_TO_DOUBLE(UnaryOpData),
    LONG_TO_INT(UnaryOpData),
    LONG_TO_FLOAT(UnaryOpData),
    LONG_TO_DOUBLE(UnaryOpData),
    FLOAT_TO_INT(UnaryOpData),
    FLOAT_TO_LONG(UnaryOpData),
    FLOAT_TO_DOUBLE(UnaryOpData),
    DOUBLE_TO_INT(UnaryOpData),
    DOUBLE_TO_LONG(UnaryOpData),
    DOUBLE_TO_FLOAT(UnaryOpData),
    INT_TO_BYTE(UnaryOpData),
    INT_TO_CHAR(UnaryOpData),
    INT_TO_SHORT(UnaryOpData),
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum BinaryOp {
    ADD_INT(BinaryOpData),
    SUB_INT(BinaryOpData),
    MUL_INT(BinaryOpData),
    DIV_INT(BinaryOpData),
    REM_INT(BinaryOpData),
    AND_INT(BinaryOpData),
    OR_INT(BinaryOpData),
    XOR_INT(BinaryOpData),
    SHL_INT(BinaryOpData),
    SHR_INT(BinaryOpData),
    USHR_INT(BinaryOpData),
    ADD_LONG(BinaryOpData),
    SUB_LONG(BinaryOpData),
    MUL_LONG(BinaryOpData),
    DIV_LONG(BinaryOpData),
    REM_LONG(BinaryOpData),
    AND_LONG(BinaryOpData),
    OR_LONG(BinaryOpData),
    XOR_LONG(BinaryOpData),
    SHL_LONG(BinaryOpData),
    SHR_LONG(BinaryOpData),
    USHR_LONG(BinaryOpData),
    ADD_FLOAT(BinaryOpData),
    SUB_FLOAT(BinaryOpData),
    MUL_FLOAT(BinaryOpData),
    DIV_FLOAT(BinaryOpData),
    REM_FLOAT(BinaryOpData),
    ADD_DOUBLE(BinaryOpData),
    SUB_DOUBLE(BinaryOpData),
    MUL_DOUBLE(BinaryOpData),
    DIV_DOUBLE(BinaryOpData),
    REM_DOUBLE(BinaryOpData),
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum BinaryOp2Addr {
    ADD_INT_2ADDR(BinaryOp2AddrData),
    SUB_INT_2ADDR(BinaryOp2AddrData),
    MUL_INT_2ADDR(BinaryOp2AddrData),
    DIV_INT_2ADDR(BinaryOp2AddrData),
    REM_INT_2ADDR(BinaryOp2AddrData),
    AND_INT_2ADDR(BinaryOp2AddrData),
    OR_INT_2ADDR(BinaryOp2AddrData),
    XOR_INT_2ADDR(BinaryOp2AddrData),
    SHL_INT_2ADDR(BinaryOp2AddrData),
    SHR_INT_2ADDR(BinaryOp2AddrData),
    USHR_INT_2ADDR(BinaryOp2AddrData),
    ADD_LONG_2ADDR(BinaryOp2AddrData),
    SUB_LONG_2ADDR(BinaryOp2AddrData),
    MUL_LONG_2ADDR(BinaryOp2AddrData),
    DIV_LONG_2ADDR(BinaryOp2AddrData),
    REM_LONG_2ADDR(BinaryOp2AddrData),
    AND_LONG_2ADDR(BinaryOp2AddrData),
    OR_LONG_2ADDR(BinaryOp2AddrData),
    XOR_LONG_2ADDR(BinaryOp2AddrData),
    SHL_LONG_2ADDR(BinaryOp2AddrData),
    SHR_LONG_2ADDR(BinaryOp2AddrData),
    USHR_LONG_2ADDR(BinaryOp2AddrData),
    ADD_FLOAT_2ADDR(BinaryOp2AddrData),
    SUB_FLOAT_2ADDR(BinaryOp2AddrData),
    MUL_FLOAT_2ADDR(BinaryOp2AddrData),
    DIV_FLOAT_2ADDR(BinaryOp2AddrData),
    REM_FLOAT_2ADDR(BinaryOp2AddrData),
    ADD_DOUBLE_2ADDR(BinaryOp2AddrData),
    SUB_DOUBLE_2ADDR(BinaryOp2AddrData),
    MUL_DOUBLE_2ADDR(BinaryOp2AddrData),
    DIV_DOUBLE_2ADDR(BinaryOp2AddrData),
    REM_DOUBLE_2ADDR(BinaryOp2AddrData),
}


#[derive(Debug, PartialEq)]
pub struct BinaryOpLit16Data {
    pub dest: u8,
    pub src: u8,
    pub literal: i16,
}

#[derive(Debug, PartialEq)]
pub struct BinaryOpLit8Data {
    pub dest: u8,
    pub src: u8,
    pub literal: u8,
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum BinaryOpLit16 {
    ADD_INT_LIT16(BinaryOpLit16Data),
    RSUB_INT(BinaryOpLit16Data),
    MUL_INT_LIT16(BinaryOpLit16Data),
    DIV_INT_LIT16(BinaryOpLit16Data),
    REM_INT_LIT16(BinaryOpLit16Data),
    AND_INT_LIT16(BinaryOpLit16Data),
    OR_INT_LIT16(BinaryOpLit16Data),
    XOR_INT_LIT16(BinaryOpLit16Data),
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum BinaryOpLit8 {
    ADD_INT_LIT8(BinaryOpLit8Data),
    RSUB_INT_LIT8(BinaryOpLit8Data),
    MUL_INT_LIT8(BinaryOpLit8Data),
    DIV_INT_LIT8(BinaryOpLit8Data),
    REM_INT_LIT8(BinaryOpLit8Data),
    AND_INT_LIT8(BinaryOpLit8Data),
    OR_INT_LIT8(BinaryOpLit8Data),
    XOR_INT_LIT8(BinaryOpLit8Data),
    SHL_INT_LIT8(BinaryOpLit8Data),
    SHR_INT_LIT8(BinaryOpLit8Data),
    USHR_INT_LIT8(BinaryOpLit8Data),
}

#[derive(Debug, PartialEq)]
pub struct PackedSwitchTable {
    pub size_in_code_units: u32,
    pub first_key: i32,
    pub targets: Vec<i32>,
}

#[derive(Debug, PartialEq)]
pub struct SparseSwitchTable {
    pub size_in_code_units: u32,
    pub keys: Vec<i32>,
    pub targets: Vec<i32>,
}

#[derive(Debug, PartialEq)]
pub struct FillArrayData {
    pub size_in_code_units: u32,
    pub element_width: u16,
    pub size: u32,
    pub data: Vec<u8>,
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum Instruction {
    NOP,
    MOVE { dest: u8, src: u8 },
    MOVE_FROM16 { dest: u8, src: u16 },
    MOVE_16 { dest: u16, src: u16 },
    MOVE_WIDE { dest: u8, src: u8 },
    MOVE_WIDE_FROM16 { dest: u8, src: u16 },
    MOVE_WIDE_16 { dest: u16, src: u16 },
    MOVE_OBJECT { dest: u8, src: u8 },
    MOVE_OBJECT_FROM16 { dest: u8, src: u16 },
    MOVE_OBJECT_16 { dest: u16, src: u16 },
    MOVE_RESULT(u8),
    MOVE_RESULT_WIDE(u8),
    MOVE_RESULT_OBJECT(u8),
    MOVE_EXCEPTION(u8),
    RETURN_VOID,
    RETURN(u8),
    RETURN_WIDE(u8),
    RETURN_OBJECT(u8),
    Cmp(CmpOp),
    IfTest(IfTestOp),
    IfTestZ(IfTestZOp),
    Array(ArrayOp),
    InstanceField(InstanceFieldOp),
    Const(ConstOp),
    MONITOR_ENTER(u8),
    MONITOR_EXIT(u8),
    CHECK_CAST { register: u8, class: Rc<String> },
    INSTANCE_OF { dest_register: u8, object_register: u8, class: Rc<String> },
    ARRAY_LENGTH { dest_register: u8, array_register: u8 },
    NEW_INSTANCE { register: u8, class: Rc<String> },
    NEW_ARRAY { dest_register: u8, size_register: u8, type_: Rc<String> },
    FILLED_NEW_ARRAY { registers: Vec<u8>, type_: Rc<String> },
    FILLED_NEW_ARRAY_RANGE { type_: Rc<String>, start_register: u16, register_count: u8 },
    FILL_ARRAY_DATA { array_register: u8, data: Rc<FillArrayData> },
    THROW(u8),
    GOTO(i8),
    GOTO_16(i16),
    GOTO_32(i32),
    PACKED_SWITCH { register: u8, table: Rc<PackedSwitchTable> },
    SPARSE_SWITCH { register: u8, table: Rc<SparseSwitchTable> },
    Static(StaticFieldOp),
    Invoke(InvokeOpData, InvokeOp),
    InvokeRange(InvokeRangeOp),
    Unary(UnaryOp),
    Binary(BinaryOp),
    Binary2Addr(BinaryOp2Addr),
    BinaryLit8(BinaryOpLit8),
    BinaryLit16(BinaryOpLit16),
    CONST_METHOD_TYPE { dest: u8, method_type: Rc<Prototype> },
}