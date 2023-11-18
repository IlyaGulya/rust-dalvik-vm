use crate::smali::ast::instructions::i::Identifier;
use crate::smali::ast::instructions::instruction::Instruction;
use crate::smali::ast::instructions::registers::RegisterIdentifier;
use crate::smali::ast::literals::{NumericLiteral, StringLiteral};
use crate::smali::ast::modifiers::MethodModifier;
use crate::smali::ast::types::any::AnyType;
use crate::smali::ast::types::assignable_value::AssignableValue;
use crate::smali::ast::types::non_void::NonVoidType;
use crate::smali::ast::types::reference::ReferenceType;

#[derive(Debug)]
pub struct MethodBody {
    pub statements: Vec<MethodBodyStatement>,
}

#[derive(Debug)]
pub enum MethodBodyStatement {
    RegistersDirective(RegistersDirective),
    // Assuming you have a structure for this
    LocalsDirective(LocalsDirective),
    // Assuming you have a structure for this
    ParamDirective(ParamDirective),
    // Assuming you have a structure for this
    LineDirective(LineDirective),
    // Assuming you have a structure for this
    Instruction(Instruction),
    // Assuming Instruction is an enum of instructions
    LineLabel(LineLabel),
    // Using the previously defined LineLabel struct
    CatchDirective(CatchDirective),
    // Assuming you have a structure for this
    CatchAllDirective(CatchAllDirective),
    // Assuming you have a structure for this
    AnnotationDirective(AnnotationDirective),
    // Using the previously defined AnnotationDirective struct
    LocalDirective(LocalDirective),
    // Using the previously defined LocalDirective struct
    LocalEndDirective(LocalEndDirective),
    // Using the previously defined LocalEndDirective struct
    LocalRestartDirective(LocalRestartDirective),
    // Using the previously defined LocalRestartDirective struct
    PackedSwitchDirective(PackedSwitchDirective),
    // Assuming you have a structure for this
    ArrayDataDirective(ArrayDataDirective),
    // Assuming you have a structure for this
    SparseSwitchDirective(SparseSwitchDirective), // Assuming you have a structure for this
}

#[derive(Debug)]
pub struct PackedSwitchDirective {
    pub ident: NumericLiteral,
    pub labels: Vec<PackedSwitchDirectiveLabel>,
}

#[derive(Debug)]
pub struct PackedSwitchDirectiveLabel {
    pub label: StringLiteral,
}


#[derive(Debug)]
pub struct RegistersDirective {
    pub count: NumericLiteral,
}

#[derive(Debug)]
pub struct LocalsDirective {
    pub count: NumericLiteral,
}

#[derive(Debug)]
pub struct ParamDirective {
    pub register: RegisterIdentifier,
    pub directive: ParamDirectiveType,
}

#[derive(Debug)]
pub enum ParamDirectiveType {
    Simple(StringLiteral),
    Extended(ExtendedParamDirective),
}

#[derive(Debug)]
pub struct ExtendedParamDirective {
    pub annotations: Vec<AnnotationDirective>,
}

#[derive(Debug)]
pub struct LineDirective {
    pub line: NumericLiteral,
}

#[derive(Debug)]
pub struct CatchDirective {
    pub exception_type: ReferenceType,
    pub from_label: StringLiteral,
    pub to_label: StringLiteral,
    pub goto_label: StringLiteral,
}

#[derive(Debug)]
pub struct CatchAllDirective {
    pub from_label: StringLiteral,
    pub to_label: StringLiteral,
    pub goto_label: StringLiteral,
}

#[derive(Debug)]
pub struct AnnotationScope {
    pub identifier: String,
}

#[derive(Debug)]
pub struct AnnotationType {
    pub reference_type: ReferenceType,
}

#[derive(Debug)]
pub enum AnnotationFieldValue {
    Value(AssignableValue),
    Type(ReferenceType),
}

#[derive(Debug)]
pub struct AnnotationValueScoped {
    pub values: Vec<AnnotationFieldValue>,
}

#[derive(Debug)]
pub struct AnnotationField {
    pub name: Identifier,
    pub value: AnnotationFieldOrScopedValue,
}

#[derive(Debug)]
pub enum AnnotationFieldOrScopedValue {
    FieldValue(AnnotationFieldValue),
    ScopedValue(AnnotationValueScoped),
}

#[derive(Debug)]
pub struct AnnotationDirective {
    pub scope: AnnotationScope,
    pub annotation_type: AnnotationType,
    pub fields: Vec<AnnotationField>,
}


#[derive(Debug)]
pub struct ArrayDataDirective {
    pub size: NumericLiteral,
    pub entries: Vec<ArrayDataEntry>,
}

#[derive(Debug)]
pub struct ArrayDataEntry {
    pub value: NumericLiteral,
    pub identifier: Option<Identifier>,
}

#[derive(Debug)]
pub struct SparseSwitchDirective {
    pub values: Vec<SparseSwitchDirectiveValue>,
}

#[derive(Debug)]
pub struct SparseSwitchDirectiveValue {
    pub value: NumericLiteral,
    pub label: StringLiteral,
}

#[derive(Debug)]
pub struct LocalDirectiveVariableName {
    pub name: StringLiteral,
}

#[derive(Debug)]
pub struct LocalDirectiveType {
    pub non_void_type: NonVoidType,
}

#[derive(Debug)]
pub struct LocalDirectiveGenericHint {
    pub hint: StringLiteral,
}

#[derive(Debug)]
pub struct LocalDirectiveRegister {
    pub register: RegisterIdentifier,
}

#[derive(Debug)]
pub struct LocalDirective {
    pub register: LocalDirectiveRegister,
    pub variable_name: LocalDirectiveVariableName,
    pub type_hint: Option<LocalDirectiveType>,
    pub generic_hint: Option<LocalDirectiveGenericHint>,
}

#[derive(Debug)]
pub struct LocalEndDirective {
    pub register: RegisterIdentifier,
}

#[derive(Debug)]
pub struct LocalRestartDirective {
    pub register: RegisterIdentifier,
}

#[derive(Debug)]
pub struct LineLabel {
    pub label: StringLiteral,
}

#[derive(Debug)]
pub struct MethodDeclaration {
    pub modifiers: Vec<MethodModifier>,
    pub signature: MethodSignature,
}

#[derive(Debug)]
pub struct MethodSignature {
    pub identifier: StringLiteral,
    pub arguments: Vec<MethodParameterType>,
    pub return_type: AnyType,
}

#[derive(Debug)]
pub enum MethodParameterType {
    NonVoid(NonVoidType),
    Literal(String), // for methodParameterLiteral
}
