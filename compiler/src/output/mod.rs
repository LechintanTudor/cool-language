use crate::scanner::Token;
use crate::symbols::SymbolTable;

use std::fmt::Write;

pub fn output_symbol_table(symbols: &SymbolTable) -> String {
    let mut output = String::default();

    for (id, ident) in symbols.to_sorted_vec() {
        writeln!(&mut output, "{:>3}: {:?}", ident, id).unwrap();
    }

    output
}

pub fn output_tokens(tokens: &[Token]) -> String {
    let mut output = String::default();

    for (i, token) in tokens.iter().enumerate() {
        writeln!(&mut output, "{:>3}: {:?}", i, token).unwrap();
    }

    output
}
