use crate::scanner::{LexicalError, Operator, ReservedWord, Separator, Token};
use crate::state_machine::StateMachine;
use crate::symbols::{Const, Symbol, SymbolTable};
use crate::utils;
use lazy_static::lazy_static;
use regex::Regex;
use unicode_segmentation::UnicodeSegmentation;

lazy_static! {
    /// Matches zero and non-zero signless numbers.
    static ref NUMBER_LITERAL_REGEX: Regex = Regex::new(r"(^0$)|(^([1-9][0-9]*)$)").unwrap();

    /// Matches string literals.
    static ref STR_LITERAL_REGEX: Regex = Regex::new(r#"^"([_a-zA-Z0-9]*)"$"#).unwrap();

    /// Matches character literals.
    static ref CHAR_LITERAL_REGEX: Regex = Regex::new(r"^'([_a-zA-Z0-9])'$").unwrap();

    /// Matches identifiers that start with underscores or ascii letters.
    static ref IDENT_REGEX: Regex = Regex::new(r"(^(_[_a-zA-Z0-9]+)$|^(([a-zA-Z])[_a-zA-Z0-9]*)$)").unwrap();

    /// State machine for validating numeric literals
    static ref NUMBER_STATE_MACHINE: StateMachine = {
        let number_state_machine = std::fs::read_to_string("state-machine/number.json")
            .expect("Failed to read number state machine file");

        serde_json::from_str::<StateMachine>(&number_state_machine)
            .expect("Failed to parse number state machine file")
    };

    /// State machine for validating identifiers.
    static ref IDENT_STATE_MACHINE: StateMachine = {
        let ident_string = std::fs::read_to_string("state-machine/identifier.json")
            .expect("Failed to read identifier state machine file");

        serde_json::from_str::<StateMachine>(&ident_string)
            .expect("Failed to parse identifier state machine file")
    };
}

/// Source file split into its tokens, identifiers and constants.
#[derive(Default, Debug)]
pub struct Program {
    /// List containing all tokens that make up the source file.
    tokens: Vec<Token>,
    /// Symbol table containing all identifiers.
    idents: SymbolTable,
    /// Symbol table containing all constants.
    consts: SymbolTable,
}

impl Program {
    /// Creates a new program from the given source code.
    pub fn from_source(source: &str) -> Result<Program, LexicalError> {
        let mut program = Program::default();
        let mut grapheme_iter = source.graphemes(true).peekable();
        let mut line = 1_usize;
        let mut word = String::new();
        let mut operator_string = String::new();

        fn consume_word(
            program: &mut Program,
            word: &mut String,
            line: usize,
        ) -> Result<(), LexicalError> {
            if let Some(reserved_word) = ReservedWord::try_parse(word) {
                program.tokens.push(reserved_word.into());
            } else if !word.is_empty() {
                let sequence = utils::str_to_grapheme_clusters(word);

                if NUMBER_STATE_MACHINE.is_accepted(&sequence) {
                    let number = parse_i32(word);
                    let number_id = program.consts.insert(Const::I32(number).into());
                    program.tokens.push(Token::Literal(number_id));
                } else if let Some(str_literal) =
                    STR_LITERAL_REGEX.captures(word).and_then(|c| c.get(1))
                {
                    let str_literal = str_literal.as_str().to_string();
                    let str_literal_id = program.consts.insert(Const::Str(str_literal).into());
                    program.tokens.push(Token::Literal(str_literal_id));
                } else if let Some(char_literal) =
                    CHAR_LITERAL_REGEX.captures(word).and_then(|c| c.get(1))
                {
                    let char_literal = char_literal.as_str().chars().next().unwrap_or('\0');
                    let char_literal_id = program.consts.insert(Const::Char(char_literal).into());
                    program.tokens.push(Token::Literal(char_literal_id));
                } else if IDENT_STATE_MACHINE.is_accepted(&sequence) {
                    let ident_id = program.idents.insert(Symbol::Ident(word.clone()));
                    program.tokens.push(Token::Ident(ident_id));
                } else {
                    return Err(LexicalError::new(format!("Failed to parse \"{}\"", word), line));
                }
            }

            word.clear();
            Ok(())
        }

        while let Some(grapheme) = grapheme_iter.next() {
            if let Some(separator) = Separator::try_parse(grapheme) {
                consume_word(&mut program, &mut word, line)?;

                if !separator.is_whitespace() {
                    program.tokens.push(separator.into());
                }
            } else if let Some(operator) = Operator::try_parse(grapheme) {
                consume_word(&mut program, &mut word, line)?;

                if operator.needs_lookahead() {
                    operator_string.clear();
                    operator_string.push_str(grapheme);
                    operator_string.push_str(grapheme_iter.peek().copied().unwrap_or(""));

                    if let Some(extended_operator) = Operator::try_parse(&operator_string) {
                        program.tokens.push(extended_operator.into());
                        grapheme_iter.next();
                    } else {
                        program.tokens.push(operator.into());
                    }
                } else {
                    program.tokens.push(operator.into());
                }
            } else {
                word.push_str(grapheme);
            }

            if grapheme == "\n" {
                line += 1;
            }
        }

        Ok(program)
    }

    /// Returns a slice of all the tokens that make up the source file.
    #[inline]
    pub fn tokens(&self) -> &[Token] {
        self.tokens.as_slice()
    }

    /// Returns the symbol table containing all identifiers from the source file.
    #[inline]
    pub fn idents(&self) -> &SymbolTable {
        &self.idents
    }

    /// Returns the symbo table containing all constants from the source file.
    #[inline]
    pub fn consts(&self) -> &SymbolTable {
        &self.consts
    }
}

/// Parses the `input` into a 32-bit signed integer.
pub fn parse_i32(input: &str) -> i32 {
    let mut accumulator = 0;

    for digit in input.chars() {
        if !('0'..='9').contains(&digit) {
            panic!("Failed to parse i32");
        }

        let digit = (u32::from(digit) - u32::from('0')) as i32;

        accumulator *= 10;
        accumulator += digit;
    }

    accumulator
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_i32() {
        assert_eq!(parse_i32("0"), 0);
        assert_eq!(parse_i32("123"), 123);
        assert_eq!(parse_i32("1000"), 1000);
    }
}
