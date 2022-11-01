use crate::scanner::{Operator, ReservedWord, Separator};

/// Token defined by the "cool language" specification.
#[derive(Clone, Debug)]
pub enum Token {
    /// Separator.
    Separator(Separator),
    /// Operator.
    Operator(Operator),
    /// Reserved word.
    ReservedWord(ReservedWord),
    /// Literal.
    Literal(usize),
    /// Identifier.
    Ident(usize),
}

impl From<Separator> for Token {
    fn from(separator: Separator) -> Self {
        Self::Separator(separator)
    }
}

impl From<Operator> for Token {
    fn from(operator: Operator) -> Self {
        Self::Operator(operator)
    }
}

impl From<ReservedWord> for Token {
    fn from(reserved_word: ReservedWord) -> Self {
        Self::ReservedWord(reserved_word)
    }
}
