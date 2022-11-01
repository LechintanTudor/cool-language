#![allow(dead_code)]

mod scanner;

/// Symbols and symbol table for storing constans and identifiers.
mod symbols;

/// Various utility functions.
mod utils;

use crate::scanner::Program;

fn main() {
    let program = std::fs::read_to_string("programs/p1.cl").unwrap();
    let program = Program::from_source(&program);
    println!("{:#?}", program);
}
