use crate::scanner::{Literal, LiteralKind, Operator, ReservedWord, Separator, Token};
use crate::symbols::{Const, Symbol, SymbolTable};
use lazy_static::lazy_static;
use regex::Regex;
use unicode_segmentation::UnicodeSegmentation;

lazy_static! {
    /// Matches zero and non-zero signless numbers.
    static ref SIGNLESS_NUMBER_REGEX: Regex = Regex::new(r"(^0$)|(^([1-9][0-9]*)$)").unwrap();

    /// Matches identifiers that start with underscores or ascii letters.
    static ref IDENT_REGEX: Regex = Regex::new(r"(^(_[_a-zA-Z0-9]+)$|^(([a-zA-Z])[_a-zA-Z0-9]*)$)").unwrap();
}

#[derive(Default, Debug)]
pub struct Program {
    pub tokens: Vec<Token>,
    pub idents: SymbolTable,
    pub consts: SymbolTable,
}

impl Program {
    pub fn from_source(source: &str) -> Program {
        let mut program = Program::default();
        let mut grapheme_iter = source.graphemes(true).peekable();
        let mut word = String::new();
        let mut operator_string = String::new();

        fn consume_word(program: &mut Program, word: &mut String) {
            if let Some(reserved_word) = ReservedWord::try_parse(word) {
                program.tokens.push(reserved_word.into());
            } else if !word.is_empty() {
                if SIGNLESS_NUMBER_REGEX.is_match(word) {
                    let number = parse_i32(word);
                    let number_id = program.consts.insert(Const::I32(number).into());
                    program.tokens.push(Literal::new(number_id, LiteralKind::I32).into());
                } else if IDENT_REGEX.is_match(word) {
                    let ident_id = program.idents.insert(Symbol::Ident(word.clone()));
                    program.tokens.push(Token::Ident(ident_id));
                } else {
                    panic!("Lexical error: {}", word);
                }

                word.clear();
            }
        }

        while let Some(grapheme) = grapheme_iter.next() {
            if let Some(separator) = Separator::try_parse(grapheme) {
                consume_word(&mut program, &mut word);

                if !separator.is_whitespace() {
                    program.tokens.push(separator.into());
                }
            } else if let Some(operator) = Operator::try_parse(grapheme) {
                consume_word(&mut program, &mut word);

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
        }

        program
    }
}

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
