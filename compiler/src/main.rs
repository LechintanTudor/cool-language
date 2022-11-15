#![allow(dead_code)]

/// Pretty-print the tokens and symbol tables of a program.
mod output;

/// Scanner for splitting source files into tokens.
mod scanner;

/// Symbols and symbol table for storing constans and identifiers.
mod symbols;

/// Various utility functions.
mod utils;

mod state_machine;

use crate::scanner::Program;
use crate::state_machine::StateMachine;

fn run_compiler(args: &[String]) {
    let source_path = match args.get(2) {
        Some(source_path) => source_path,
        None => {
            eprintln!("No source file provied");
            std::process::exit(3);
        }
    };

    let source = match std::fs::read_to_string(&source_path) {
        Ok(source) => source,
        Err(error) => {
            eprintln!("Failed to read source file: {}", error);
            std::process::exit(4);
        }
    };

    let program = match Program::from_source(&source) {
        Ok(program) => program,
        Err(error) => {
            eprintln!("{}", error);
            std::process::exit(5);
        }
    };

    println!("[Identifiers]\n{}\n", output::output_symbol_table(program.idents()));
    println!("[Constants]\n{}\n", output::output_symbol_table(program.consts()));
    println!("[Tokens]\n{}\n", output::output_tokens(program.tokens()));
}

fn run_state_machine(args: &[String]) {
    let automata_path = match args.get(2) {
        Some(automata_path) => automata_path,
        None => {
            eprintln!("No automata file provided");
            std::process::exit(3);
        }
    };

    let automata_string = match std::fs::read_to_string(&automata_path) {
        Ok(automata_string) => automata_string,
        Err(error) => {
            eprintln!("Failed to read automata file: {}", error);
            std::process::exit(4);
        }
    };

    let automata = match serde_json::from_str::<StateMachine>(&automata_string) {
        Ok(automata) => automata,
        Err(error) => {
            eprintln!("Failed to deserialize automata: {}", error);
            std::process::exit(5);
        }
    };
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    if let Some(run_mode) = args.get(1).map(String::as_str) {
        match run_mode {
            "compile" => run_compiler(&args),
            "automata" => run_state_machine(&args),
            unknown => {
                eprintln!("Unknown run mode, '{}'", unknown);
                std::process::exit(2);
            }
        }
    } else {
        eprintln!("No run mode provided");
        std::process::exit(1);
    }
}
