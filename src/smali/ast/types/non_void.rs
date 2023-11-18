use crate::smali::ast::types::array::ArrayType;
use crate::smali::ast::types::non_array_type::NonArrayType;

#[derive(Debug)]
pub enum NonVoidType {
    NonArrayType(NonArrayType),
    ArrayType(ArrayType),
}
