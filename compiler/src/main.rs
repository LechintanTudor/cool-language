#![allow(dead_code)]

/// Constans and identifiers that may be saved in a [SymbolTable](crate::SymbolTable).
mod symbol;

/// Symbol table for storing constans and identifiers.
mod symbol_table;

/// Various utility functions.
mod utils;

use crate::symbol::{Const, Symbol};
use crate::symbol_table::SymbolTable;

fn main() {
    let mut symbol_table = SymbolTable::default();
    symbol_table.insert(Symbol::Ident("counter".to_string()));
    symbol_table.insert(Symbol::Const(Const::I32(100)));
    println!("{:#?}", symbol_table);
}
