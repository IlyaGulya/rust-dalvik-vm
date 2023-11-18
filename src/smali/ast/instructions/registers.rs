#[derive(Debug)]
pub enum RegisterList {
    Empty,
    Registers(Vec<RegisterIdentifier>),
}

#[derive(Debug)]
pub struct RegisterIdentifier {
    pub name: String,
}

#[derive(Debug)]
pub struct RegisterRange {
    pub start: RegisterIdentifier,
    pub end: RegisterIdentifier,
}
