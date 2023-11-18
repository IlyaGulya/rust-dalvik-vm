use crate::smali::ast::types::reference::ReferenceType;
use crate::smali::ast::types::primitive::PrimitiveType;

#[derive(Debug)]
pub enum NonArrayType {
    Primitive(PrimitiveType),
    Reference(ReferenceType),
}
