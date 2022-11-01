use crate::scanner::{Literal, Operator, ReservedWord, Separator};

#[derive(Clone, Debug)]
pub enum Token {
    Separator(Separator),
    Operator(Operator),
    ReservedWord(ReservedWord),
    Literal(Literal),
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

impl From<Literal> for Token {
    fn from(literal: Literal) -> Self {
        Self::Literal(literal)
    }
}
