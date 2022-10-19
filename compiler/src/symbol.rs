use crate::utils;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Const {
    I32(i32),
    Char(char),
    Bool(bool),
    String(String),
}

impl Const {
    pub fn hash_code(&self) -> u64 {
        match self {
            Self::I32(value) => u64::from_ne_bytes(i64::from(*value).to_ne_bytes()),
            Self::Char(value) => u64::from(*value),
            Self::Bool(value) => u64::from(*value),
            Self::String(value) => utils::hash_string(value),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Symbol {
    Ident(String),
    Const(Const),
}

impl Symbol {
    pub fn hash_code(&self) -> u64 {
        match self {
            Self::Ident(value) => utils::hash_string(value),
            Self::Const(value) => value.hash_code(),
        }
    }
}
