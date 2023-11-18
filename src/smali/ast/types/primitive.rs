use crate::smali::ast::types::non_array_type::NonArrayType;

#[derive(Debug)]
pub enum PrimitiveType {
    Boolean,
    Byte,
    Short,
    Char,
    Int,
    Long,
    Float,
    Double,
}

