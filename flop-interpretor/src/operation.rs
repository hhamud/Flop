use crate::error::EvalError;

pub enum Operation {
    Add,
    Subtract,
    Divide,
    Multiply,
    Equal,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
}

impl Operation {
    pub fn apply(&self, a: i64, b: i64) -> i64 {
        match self {
            Operation::Add => a + b,
            Operation::Subtract => a - b,
            Operation::Divide => a / b,
            Operation::Multiply => a * b,
            Operation::Equal => (a == b) as i64,
            Operation::GreaterThan => (a > b) as i64,
            Operation::GreaterThanOrEqual => (a >= b) as i64,
            Operation::LessThan => (a < b) as i64,
            Operation::LessThanOrEqual => (a <= b) as i64,
        }
    }
}

impl TryFrom<&str> for Operation {
    type Error = EvalError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "+" => Ok(Operation::Add),
            "-" => Ok(Operation::Subtract),
            "/" => Ok(Operation::Divide),
            "*" => Ok(Operation::Multiply),
            "==" => Ok(Operation::Equal),
            ">=" => Ok(Operation::GreaterThanOrEqual),
            "<=" => Ok(Operation::LessThanOrEqual),
            ">" => Ok(Operation::GreaterThan),
            "<" => Ok(Operation::LessThan),
            _ => Err(EvalError::SymbolError(value)),
        }
    }
}
