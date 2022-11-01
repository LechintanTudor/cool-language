/// Literal defined by the "cool language" specification.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Literal {
    /// Id in the constant symbol table.
    pub id: usize,
    /// Type of literal.
    pub kind: LiteralKind,
}

impl Literal {
    pub fn new(id: usize, kind: LiteralKind) -> Self {
        Self { id, kind }
    }
}

/// Type of literal.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LiteralKind {
    /// `i32` literal.
    I32,
    /// `char` literal.
    Char,
    /// `str` literal.
    Str,
}
