/// Operator defined by the "cool language" specification.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Operator {
    /// Assignment operator.
    Assignment(AssignmentOperator),
    /// Arithmetic operator.
    Arithmetic(ArithmeticOperator),
    /// Relational operator.
    Relational(RelationalOperator),
    /// Logical operator.
    Logical(LogicalOperator),
    /// Bitwiser operator.
    Bitwise(BitwiseOperator),
}

impl Operator {
    /// Tries to parse the `input` into an operator.
    pub fn try_parse(input: &str) -> Option<Self> {
        if let Some(operator) = AssignmentOperator::try_parse(input) {
            return Some(Self::Assignment(operator));
        }

        if let Some(operator) = ArithmeticOperator::try_parse(input) {
            return Some(Self::Arithmetic(operator));
        }

        if let Some(operator) = RelationalOperator::try_parse(input) {
            return Some(Self::Relational(operator));
        }

        if let Some(operator) = LogicalOperator::try_parse(input) {
            return Some(Self::Logical(operator));
        }

        if let Some(operator) = BitwiseOperator::try_parse(input) {
            return Some(Self::Bitwise(operator));
        }

        None
    }

    /// Returns `true` if the scanner needs to look ahead to determine the correct operator type.
    pub fn needs_lookahead(&self) -> bool {
        match self {
            Self::Assignment(operator) => operator.needs_lookahead(),
            Self::Arithmetic(operator) => operator.needs_lookahead(),
            Self::Relational(operator) => operator.needs_lookahead(),
            Self::Logical(operator) => operator.needs_lookahead(),
            Self::Bitwise(operator) => operator.needs_lookahead(),
        }
    }
}

impl From<AssignmentOperator> for Operator {
    fn from(operator: AssignmentOperator) -> Self {
        Self::Assignment(operator)
    }
}

impl From<ArithmeticOperator> for Operator {
    fn from(operator: ArithmeticOperator) -> Self {
        Self::Arithmetic(operator)
    }
}

impl From<RelationalOperator> for Operator {
    fn from(operator: RelationalOperator) -> Self {
        Self::Relational(operator)
    }
}

impl From<LogicalOperator> for Operator {
    fn from(operator: LogicalOperator) -> Self {
        Self::Logical(operator)
    }
}

impl From<BitwiseOperator> for Operator {
    fn from(operator: BitwiseOperator) -> Self {
        Self::Bitwise(operator)
    }
}

/// Assignment operator defined by the "cool language" specification.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct AssignmentOperator;

impl AssignmentOperator {
    /// Tries to parse the `input` into an assignment operator.
    pub fn try_parse(input: &str) -> Option<Self> {
        Some(match input {
            "=" => Self,
            _ => return None,
        })
    }

    /// Returns `true` if the scanner needs to look ahead to determine the correct operator type.
    pub fn needs_lookahead(&self) -> bool {
        // = -> ==
        true
    }
}

/// Arithmetic operator defined by the "cool language" specification.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ArithmeticOperator {
    /// Addition operator.
    Addition,
    /// Subtraction operator.
    Subtraction,
    /// Multiplication operator.
    Multiplication,
    /// Division operator.
    Division,
    /// Remainder operator.
    Remainder,
}

impl ArithmeticOperator {
    /// Tries to parse the `input` into an arithmetic operator.
    pub fn try_parse(input: &str) -> Option<Self> {
        Some(match input {
            "+" => Self::Addition,
            "-" => Self::Subtraction,
            "*" => Self::Multiplication,
            "/" => Self::Division,
            "%" => Self::Remainder,
            _ => return None,
        })
    }

    /// Returns `true` if the scanner needs to look ahead to determine the correct operator type.
    pub fn needs_lookahead(&self) -> bool {
        false
    }
}

/// Relational operator defined by the "cool language" specification.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RelationalOperator {
    /// Equal operator.
    Equal,
    /// Not equal operator.
    NotEqual,
    /// Less than operator.
    Less,
    /// Less than or equal operator.
    LessOrEqual,
    /// Greater than operator.
    Greater,
    /// Greater than or equal operator.
    GreaterOrEqual,
}

impl RelationalOperator {
    /// Tries to parse the `input` into a relational operator.
    pub fn try_parse(input: &str) -> Option<Self> {
        Some(match input {
            "==" => Self::Equal,
            "!=" => Self::NotEqual,
            "<" => Self::Less,
            "<=" => Self::LessOrEqual,
            ">" => Self::Greater,
            ">=" => Self::GreaterOrEqual,
            _ => return None,
        })
    }

    /// Returns `true` if the scanner needs to look ahead to determine the correct operator type.
    pub fn needs_lookahead(&self) -> bool {
        // < -> <=, > -> >=
        [Self::Less, Self::Greater].contains(self)
    }
}

/// Logical operator defined by the "cool language" specification.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LogicalOperator {
    /// And operator.
    And,
    /// Or operator.
    Or,
    /// Not operator.
    Not,
}

impl LogicalOperator {
    /// Tries to parse the `input` into a logical operator.
    pub fn try_parse(input: &str) -> Option<Self> {
        Some(match input {
            "&&" => Self::And,
            "||" => Self::Or,
            "!" => Self::Not,
            _ => return None,
        })
    }

    /// Returns `true` if the scanner needs to look ahead to determine the correct operator type.
    pub fn needs_lookahead(&self) -> bool {
        // ! -> !=
        [Self::Not].contains(self)
    }
}

/// Bitwise operator defined by the "cool language" specification.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BitwiseOperator {
    /// Bitwise and operator.
    And,
    /// Bitwise or operator.
    Or,
    /// Bitwise not operator.
    Not,
}

impl BitwiseOperator {
    /// Tries to parse the `input` into a bitwise operator.
    pub fn try_parse(input: &str) -> Option<Self> {
        Some(match input {
            "&" => Self::And,
            "|" => Self::Or,
            "^" => Self::Not,
            _ => return None,
        })
    }

    /// Returns `true` if the scanner needs to look ahead to determine the correct operator type.
    pub fn needs_lookahead(&self) -> bool {
        // & -> &&, | -> ||
        [Self::And, Self::Or].contains(self)
    }
}
