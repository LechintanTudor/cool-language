use crate::scanner::{Literal, LiteralKind, ReservedWord, Separator, Token};
use crate::symbols::{Const, Symbol, SymbolTable};
use regex::Regex;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct Program {
    pub tokens: Vec<Token>,
    pub idents: SymbolTable,
    pub consts: SymbolTable,
}

impl Program {
    pub fn from_str(program: &str) -> Program {
        let mut tokens = Vec::<Token>::new();
        let mut idents = SymbolTable::default();
        let mut consts = SymbolTable::default();

        // Matches zero and non-zero signless numbers
        let signless_number_regex = Regex::new(r"(^0$)|(^([1-9][0-9]*)$)").unwrap();

        // Matches identifiers that start with underscores or ascii letters
        let ident_regex = Regex::new(r"(^(_[_a-zA-Z0-9]+)$|^(([a-zA-Z])[_a-zA-Z0-9]*)$)").unwrap();

        let mut grapheme_iter = program.graphemes(true);
        let mut word = String::new();

        while let Some(grapheme) = grapheme_iter.next() {
            match Separator::try_parse(grapheme) {
                Some(separator) => {
                    if !separator.is_whitespace() {
                        tokens.push(Token::Separator(separator));
                    }

                    if !word.is_empty() {
                        match ReservedWord::try_parse(&word) {
                            Some(reserved_word) => {
                                tokens.push(Token::ReservedWord(reserved_word));
                            }
                            None => {
                                if signless_number_regex.is_match(&word) {
                                    let number = parse_i32(&word);
                                    let number_id =
                                        consts.insert(Symbol::Const(Const::I32(number)));
                                    tokens.push(Token::Literal(Literal::new(
                                        number_id,
                                        LiteralKind::I32,
                                    )));
                                } else if ident_regex.is_match(&word) {
                                    let ident_id = idents.insert(Symbol::Ident(word.to_string()));
                                    tokens.push(Token::Ident(ident_id));
                                }
                            }
                        }

                        word.clear();
                    }
                }
                None => {
                    word.push_str(grapheme);
                }
            }
        }

        Program { tokens, idents, consts }
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
