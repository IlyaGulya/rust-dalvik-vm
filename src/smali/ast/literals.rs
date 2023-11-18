#[derive(Debug)]
pub struct BooleanLiteral {
    pub value: bool,
}

#[derive(Debug)]
pub struct NullLiteral;

#[derive(Debug)]
pub enum NumericLiteral {
    Negative(Box<PositiveNumericLiteral>),
    Positive(PositiveNumericLiteral),
}

#[derive(Debug)]
pub enum PositiveNumericLiteral {
    Decimal(String),
    Hex(String),
    Octal(String),
    Binary(String),
    Float(String),
    HexFloat(String),
}

#[derive(Debug)]
pub struct StringLiteral {
    pub value: String,
}
