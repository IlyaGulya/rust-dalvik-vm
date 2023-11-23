use std::rc::Rc;

use crate::dex::dex_file::{Field, Method};

#[derive(Debug, PartialEq)]
pub struct StaticFieldOpData {
    pub register: u8,
    pub field: Rc<Field>,
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum ConstOp {
    CONST_STRING {
        register: u8,
        string: Rc<String>,
    },
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum StaticFieldOp {
    SGET(StaticFieldOpData),
    SPUT(StaticFieldOpData),
    SGET_OBJECT(StaticFieldOpData),
}

#[derive(Debug, PartialEq)]
pub struct InvokeOpData {
    pub method: Rc<Method>,
    pub args_registers: Vec<u8>,
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum InvokeOp {
    INVOKE_VIRTUAL(InvokeOpData),
    INVOKE_SUPER(InvokeOpData),
    INVOKE_DIRECT(InvokeOpData),
    INVOKE_STATIC(InvokeOpData),
    INVOKE_INTERFACE(InvokeOpData),
}


#[derive(Debug, PartialEq)]
pub struct BinaryOpLit16Data {
    pub dest: u8,
    pub src: u8,
    pub literal: u16,
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
    SHL_INT_LIT16(BinaryOpLit16Data),
    SHR_INT_LIT16(BinaryOpLit16Data),
    USHR_INT_LIT16(BinaryOpLit16Data),
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum Instruction {
    NOP,
    RETURN_VOID,
    RETURN_WIDE(u8),
    ConstOp(ConstOp),
    StaticOp(StaticFieldOp),
    InvokeOp(InvokeOp),
    BinaryOpLit16(BinaryOpLit16),
}