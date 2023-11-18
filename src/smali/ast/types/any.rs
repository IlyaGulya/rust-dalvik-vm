use crate::smali::ast::types::array::ArrayType;
use crate::smali::ast::types::non_void::NonVoidType;

#[derive(Debug)]
pub enum AnyType {
    NonVoidType(NonVoidType),
    VoidType,
    ArrayType(ArrayType),
}
