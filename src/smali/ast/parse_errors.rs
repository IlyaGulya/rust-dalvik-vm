#[derive(Debug)]
pub enum ParseError {
    WrongNode,
    WrongNodeType,
    NodeParseError(String),
}