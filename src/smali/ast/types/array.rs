use crate::smali::ast::instructions::i::Identifier;
use crate::smali::ast::types::non_array_type::NonArrayType;

#[derive(Debug)]
pub enum ArrayType {
    NonArray(Box<NonArrayType>),
    Array(Box<ArrayType>),
    MethodParameterLiteral(String),
}

#[derive(Debug)]
pub enum MethodParameterLiteral {
    Identifier(Identifier)
}