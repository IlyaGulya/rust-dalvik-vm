use crate::smali::ast::types::array::ArrayType;

#[derive(Debug)]
pub enum ReferenceOrArrayType {
    ReferenceType(String),
    ArrayType(ArrayType),
}
