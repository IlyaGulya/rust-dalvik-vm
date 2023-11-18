use crate::smali::ast::literals::StringLiteral;
use crate::smali::ast::method_body::{MethodBody, MethodDeclaration};
use crate::smali::ast::modifiers::{ClassModifier, FieldModifier};
use crate::smali::ast::types::assignable_value::AssignableValue;
use crate::smali::ast::types::field_name_and_type::FieldNameAndType;
use crate::smali::ast::types::reference::ReferenceType;

#[derive(Debug)]
pub struct ClassDirective {
    pub modifiers: Vec<ClassModifier>,
    pub name: StringLiteral,
}

#[derive(Debug)]
pub struct SuperDirective {
    pub name: ReferenceType,
}

#[derive(Debug)]
pub struct SourceDirective {
    pub name: StringLiteral,
}

#[derive(Debug)]
pub struct FieldDirective {
    pub modifiers: Vec<FieldModifier>,
    pub name_and_type: FieldNameAndType,
    pub value: Option<AssignableValue>,
}

#[derive(Debug)]
pub struct MethodDirective {
    pub declaration: MethodDeclaration,
    pub body: Option<MethodBody>,
}