#![allow(dead_code)]

/// Pretty-print the tokens and symbol tables of a program.
mod output;

/// Scanner for splitting source files into tokens.
mod scanner;

/// Symbols and symbol table for storing constans and identifiers.
mod symbols;

/// Various utility functions.
mod utils;

use crate::scanner::Program;

fn main() {
    let source_path = match std::env::args().nth(1) {
        Some(source_path) => source_path,
        None => {
            eprintln!("No source file provied");
            std::process::exit(1);
        }
    };

    let source = match std::fs::read_to_string(&source_path) {
        Ok(source) => source,
        Err(error) => {
            eprintln!("Failed to read source file: {}", error);
            std::process::exit(2);
        }
    };

    let program = match Program::from_source(&source) {
        Ok(program) => program,
        Err(error) => {
            eprintln!("{}", error);
            std::process::exit(3);
        }
    };

    println!("[Identifiers]\n{}\n", output::output_symbol_table(&program.idents));
    println!("[Constants]\n{}\n", output::output_symbol_table(&program.consts));
    println!("[Tokens]\n{}\n", output::output_tokens(&program.tokens));
}
