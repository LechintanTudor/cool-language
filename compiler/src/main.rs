#![allow(dead_code)]

mod output;

mod scanner;

/// Symbols and symbol table for storing constans and identifiers.
mod symbols;

/// Various utility functions.
mod utils;

use crate::scanner::Program;

fn main() {
    let program = std::fs::read_to_string("programs/p1.cl").expect("Failed to read program");

    let program = match Program::from_source(&program) {
        Ok(program) => program,
        Err(error) => {
            println!("{}", error);
            std::process::exit(1);
        }
    };

    println!("{}", output::output_symbol_table(&program.idents));
    println!("{}", output::output_symbol_table(&program.consts));
    println!("{}", output::output_tokens(&program.tokens));
}
