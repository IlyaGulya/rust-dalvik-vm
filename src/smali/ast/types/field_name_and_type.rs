use crate::smali::ast::instructions::i::Identifier;
use crate::smali::ast::types::any::AnyType;

#[derive(Debug)]
pub struct FieldNameAndType {
    pub name: Identifier,
    pub field_type: AnyType,
}
