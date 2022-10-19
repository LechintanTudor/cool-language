mod symbol;
#[allow(dead_code)]

mod symbol_table;
mod utils;

use crate::symbol::{Const, Symbol};
use crate::symbol_table::SymbolTable;

fn main() {
    let mut symbol_table = SymbolTable::default();
    symbol_table.insert(Symbol::Ident("counter".to_string()));
    symbol_table.insert(Symbol::Const(Const::I32(100)));
    println!("{:#?}", symbol_table);
}
