#[derive(Debug)]
pub enum StatementParseError {
    InvalidDirective,
    ParseError(String),
    NotImplemented,
}