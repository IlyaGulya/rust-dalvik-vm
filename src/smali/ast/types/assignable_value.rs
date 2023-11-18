use crate::smali::ast::literals::BooleanLiteral;
use crate::smali::ast::literals::NullLiteral;
use crate::smali::ast::literals::NumericLiteral;
use crate::smali::ast::literals::StringLiteral;
use crate::smali::ast::types::any::AnyType;

#[derive(Debug)]
pub enum AssignableValue {
    Type(AnyType),
    StringLiteral(StringLiteral),
    NumericLiteral(NumericLiteral),
    NullLiteral(NullLiteral),
    BooleanLiteral(BooleanLiteral),
}
