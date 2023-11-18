use crate::smali::ast::directives::directives::{ClassDirective, FieldDirective, MethodDirective, SourceDirective, SuperDirective};

#[derive(Debug)]
pub enum Statement {
    Class(ClassDirective),
    Super(SuperDirective),
    Source(SourceDirective),
    Field(FieldDirective),
    Method(MethodDirective),
}
