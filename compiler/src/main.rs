#![allow(dead_code)]

/// Pretty-print the tokens and symbol tables of a program.
mod output;

/// Scanner for splitting source files into tokens.
mod scanner;

/// Symbols and symbol table for storing constans and identifiers.
mod symbols;

/// Various utility functions.
mod utils;

/// State machine implementation.
mod state_machine;

use crate::scanner::Program;
use crate::state_machine::StateMachine;
use std::io::Write;
use unicode_segmentation::UnicodeSegmentation;

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
    let state_machine_path = match args.get(2) {
        Some(state_machine_path) => state_machine_path,
        None => {
            eprintln!("No state_machine file provided");
            std::process::exit(3);
        }
    };

    let state_machine_string = match std::fs::read_to_string(&state_machine_path) {
        Ok(state_machine_string) => state_machine_string,
        Err(error) => {
            eprintln!("Failed to read state_machine file: {}", error);
            std::process::exit(4);
        }
    };

    let state_machine = match serde_json::from_str::<StateMachine>(&state_machine_string) {
        Ok(state_machine) => state_machine,
        Err(error) => {
            eprintln!("Failed to deserialize state_machine: {}", error);
            std::process::exit(5);
        }
    };

    let mut input = String::new();
    let mut should_run = true;

    while should_run {
        print!(">>> ");
        std::io::stdout().flush().expect("Failed to flush stdout");

        input.clear();
        std::io::stdin().read_line(&mut input).expect("Failed to read from stdin");
        let mut words = input.split_whitespace().map(str::trim);

        if let Some(command) = words.next() {
            match command {
                "display" => {
                    display_state_machine(&state_machine);
                }
                "validate" => match words.next() {
                    Some(sequence) => {
                        let split_sequence = sequence.graphemes(true).collect::<Vec<_>>();

                        if state_machine.is_accepted(&split_sequence) {
                            println!("{} is accepted", sequence);
                        } else {
                            println!("{} is not accepted", sequence);
                        }
                    }
                    None => eprintln!("No sequence provided"),
                },
                "exit" => {
                    should_run = false;
                }
                unknown => {
                    eprintln!("Unknown command: '{}'", unknown);
                }
            }
        }

        println!();
    }
}

fn display_state_machine(state_machine: &StateMachine) {
    println!("[STATES]");
    for state in state_machine.iter_states() {
        println!("{}", state);
    }

    println!("\n[ALPHABET]");
    for symbol in state_machine.iter_symbols() {
        println!("{}", symbol);
    }

    println!("\n[TRANSITIONS]");
    for (src_state, symbol, dst_state) in state_machine.iter_transitions() {
        println!("{} ---({})--> {}", src_state, symbol, dst_state);
    }

    println!("\n[INITIAL STATE]");
    println!("{}", state_machine.initial_state());

    println!("\n[FINAL STATES]");
    for final_state in state_machine.final_states() {
        println!("{}", final_state);
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    if let Some(run_mode) = args.get(1).map(String::as_str) {
        match run_mode {
            "compile" => run_compiler(&args),
            "state_machine" => run_state_machine(&args),
            unknown => {
                eprintln!("Unknown run mode: '{}'", unknown);
                std::process::exit(2);
            }
        }
    } else {
        eprintln!("No run mode provided");
        std::process::exit(1);
    }
}
