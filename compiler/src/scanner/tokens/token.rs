use crate::scanner::{Literal, ReservedWord, Separator};

#[derive(Clone, Debug)]
pub enum Token {
    Separator(Separator),
    ReservedWord(ReservedWord),
    Literal(Literal),
    Ident(usize),
}
