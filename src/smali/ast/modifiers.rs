#[derive(Debug)]
pub enum ClassModifier {
    Public,
    Private,
    Protected,
    Final,
    Annotation,
    Synthetic,
    Static,
    Abstract,
    Enum,
    Interface,
}

#[derive(Debug)]
pub enum MethodModifier {
    Public,
    Private,
    Protected,
    Final,
    Synthetic,
    Static,
    Abstract,
    Constructor,
    Bridge,
    DeclaredSynchronized,
    Strictfp,
    Varargs,
    Native,
}

#[derive(Debug)]
pub enum FieldModifier {
    Public,
    Private,
    Protected,
    Final,
    Synthetic,
    Static,
    Transient,
    Volatile,
    Enum,
}
