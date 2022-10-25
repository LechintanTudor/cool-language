use crate::utils;

/// Constant that can be stored in a [SymbolTable](crate::SymbolTable).
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Const {
    /// 32-bit integer constant.
    I32(i32),
    /// Unicode scalar constant.
    Char(char),
    /// Boolean constant.
    Bool(bool),
    /// String constant.
    String(String),
}

impl Const {
    /// Returns the hash code of the constant.
    pub fn hash_code(&self) -> u64 {
        match self {
            Self::I32(value) => u64::from_ne_bytes(i64::from(*value).to_ne_bytes()),
            Self::Char(value) => u64::from(*value),
            Self::Bool(value) => u64::from(*value),
            Self::String(value) => utils::hash_str(value),
        }
    }
}

/// Identifier or constant that can be stored in a [SymbolTable](crate::SymbolTable).
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Symbol {
    /// Identifier.
    Ident(String),
    /// Constant.
    Const(Const),
}

impl Symbol {
    /// Returns the hash code of the symbol.
    pub fn hash_code(&self) -> u64 {
        match self {
            Self::Ident(value) => utils::hash_str(value),
            Self::Const(value) => value.hash_code(),
        }
    }
}
