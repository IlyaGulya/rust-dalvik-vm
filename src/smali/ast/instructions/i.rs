use crate::smali::ast::method_body::MethodSignature;
use crate::smali::ast::instructions::registers::{RegisterIdentifier, RegisterList, RegisterRange};
use crate::smali::ast::types::field_name_and_type::FieldNameAndType;
use crate::smali::ast::types::reference_or_array::ReferenceOrArrayType;

#[derive(Debug)]
pub struct NopInstruction;


#[derive(Debug)]
pub struct FieldAccessInstruction {
    pub register: RegisterIdentifier,
    pub field: FieldInvocationTarget, // Assuming you have a structure for this
}

#[derive(Debug)]
pub struct MethodInvocationInstruction {
    pub registers: RegisterList,
    pub method: MethodInvocationTarget, // Assuming you have a structure for this
}

#[derive(Debug)]
pub struct MethodInvocationRangeInstruction {
    pub register_range: RegisterRange,
    pub method_target: MethodInvocationTarget, // Assuming you have a structure for this
}

#[derive(Debug)]
pub struct TypeConversionInstruction {
    pub from_register: RegisterIdentifier,
    pub to_register: RegisterIdentifier,
}

#[derive(Debug)]
pub struct MethodInvocationTarget {
    pub target: ReferenceOrArrayType,
    pub method_signature: MethodSignature,
}

#[derive(Debug)]
pub struct FieldInvocationTarget {
    pub target: ReferenceOrArrayType,
    pub field_name_and_type: FieldNameAndType,
}


// Enums and structs for identifiers and types would be defined based on the parts of the grammar you've shown.
// For example:
#[derive(Debug)]
pub enum Identifier {
    Simple(String),
    Generics(String, String), // For <identifier>
}


