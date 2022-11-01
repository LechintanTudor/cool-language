/// Separator defined by the "cool language" specification.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Separator {
    /// Any kind of whitespace.
    Whitespace,
    /// ;
    Semicolon,
    /// :
    Colon,
    /// (
    OpenParanthesis,
    /// )
    ClosedParanthesis,
    /// [
    OpenSquareBracket,
    /// ]
    ClosedSquareBracket,
    /// {
    OpenBracket,
    /// }
    ClosedBracket,
}

impl Separator {
    /// Tries to parse the `input` into a separator.
    pub fn try_parse(input: &str) -> Option<Self> {
        if input.trim().is_empty() {
            Some(Self::Whitespace)
        } else {
            Some(match input {
                ";" => Self::Semicolon,
                ":" => Self::Colon,
                "(" => Self::OpenParanthesis,
                ")" => Self::ClosedParanthesis,
                "[" => Self::OpenSquareBracket,
                "]" => Self::ClosedSquareBracket,
                "{" => Self::OpenBracket,
                "}" => Self::ClosedBracket,
                _ => return None,
            })
        }
    }

    /// Returns whether the separator is whitespace.
    pub fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace)
    }
}
