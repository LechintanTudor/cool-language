#[derive(Clone, Debug)]
pub enum Token {
    Separator(Separator),
    ReservedWord(ReservedWord),
    Literal(Literal),
    Ident(usize),
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Separator {
    Whitespace,
    Semicolon,
    Colon,
    OpenParanthesis,
    ClosedParanthesis,
    OpenSquareBracket,
    ClosedSquareBracket,
    OpenBracket,
    ClosedBracket,
    SingleQuote,
    DoubleQuote,
}

impl Separator {
    pub fn try_parse(input: &str) -> Option<Self> {
        if input.trim().is_empty() {
            Some(Self::Whitespace)
        } else {
            Some(match input {
                ";" => Self::Semicolon,
                ":" => Self::Colon,
                "(" => Self::OpenParanthesis,
                "" => Self::ClosedParanthesis,
                "[" => Self::OpenSquareBracket,
                "]" => Self::ClosedSquareBracket,
                "{" => Self::OpenBracket,
                "}" => Self::ClosedBracket,
                "'" => Self::SingleQuote,
                "\"" => Self::DoubleQuote,
                _ => return None,
            })
        }
    }

    #[inline]
    pub fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace)
    }
}

#[derive(Clone, Debug)]
pub enum ReservedWord {
    Primitive(Primitive),
    ControlFlow(ControlFlow),
    BoolLiteral(BoolLiteral),
}

impl ReservedWord {
    pub fn try_parse(input: &str) -> Option<Self> {
        if let Some(primitive) = Primitive::try_parse(input) {
            return Some(Self::Primitive(primitive));
        };

        if let Some(control_flow) = ControlFlow::try_parse(input) {
            return Some(Self::ControlFlow(control_flow));
        };

        if let Some(bool_literal) = BoolLiteral::try_parse(input) {
            return Some(Self::BoolLiteral(bool_literal));
        };

        None
    }
}

#[derive(Clone, Debug)]
pub enum Primitive {
    I32,
    Bool,
    Char,
    String,
}

impl Primitive {
    pub fn try_parse(input: &str) -> Option<Self> {
        Some(match input {
            "i32" => Self::I32,
            "bool" => Self::Bool,
            "char" => Self::Char,
            "str" => Self::String,
            _ => return None,
        })
    }
}

#[derive(Clone, Debug)]
pub enum ControlFlow {
    If,
    Else,
    For,
    In,
    While,
}

impl ControlFlow {
    pub fn try_parse(input: &str) -> Option<Self> {
        Some(match input {
            "if" => Self::If,
            "else" => Self::Else,
            "for" => Self::For,
            "in" => Self::In,
            "while" => Self::While,
            _ => return None,
        })
    }
}

#[derive(Clone, Debug)]
pub enum BoolLiteral {
    True,
    False,
}

impl BoolLiteral {
    pub fn try_parse(input: &str) -> Option<Self> {
        Some(match input {
            "true" => Self::True,
            "false" => Self::False,
            _ => return None,
        })
    }
}

#[derive(Clone, Debug)]
pub enum Literal {
    I32(usize),
    Char(usize),
    String(usize),
}
