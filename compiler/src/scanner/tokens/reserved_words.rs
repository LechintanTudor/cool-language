/// Reserved words defined by the "cool language" specification.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ReservedWord {
    /// Primitive types.
    Primitive(Primitive),
    /// Control flow.
    ControlFlow(ControlFlow),
    /// `true` and `false`.
    BoolLiteral(BoolLiteral),
}

impl ReservedWord {
    /// Tries to parse the `input` into a reserved word.
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

/// Primitive type defined by the "cool language" specification.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Primitive {
    /// `i32`.
    I32,
    /// `bool`.
    Bool,
    /// `char`.
    Char,
    /// `str`.
    Str,
}

impl Primitive {
    /// Tries to parse the `input` into a primitive.
    pub fn try_parse(input: &str) -> Option<Self> {
        Some(match input {
            "i32" => Self::I32,
            "bool" => Self::Bool,
            "char" => Self::Char,
            "str" => Self::Str,
            _ => return None,
        })
    }
}

/// Control flow word defined by the "cool language" specification.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ControlFlow {
    /// `if`.
    If,
    /// `else`.
    Else,
    /// `for`.
    For,
    /// `in`.
    In,
    /// `while`.
    While,
}

impl ControlFlow {
    /// Tries to parse the `input` into a control flow word.
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

/// Boolean literal defined by the "cool language" specification.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BoolLiteral {
    /// `true`.
    True,
    /// `false`.
    False,
}

impl BoolLiteral {
    /// Tries to parse the `input` into a boolean literal.
    pub fn try_parse(input: &str) -> Option<Self> {
        Some(match input {
            "true" => Self::True,
            "false" => Self::False,
            _ => return None,
        })
    }
}
