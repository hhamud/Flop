pub enum Operation {
    Add,
    Subtract,
    Divide,
    Multiply,
}

pub enum Comparison {
    Equal,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
}

impl Comparison {
    pub fn apply(&self, a: i64, b: i64) -> bool {
        match self {
            Comparison::Equal => a == b,
            Comparison::GreaterThan => a > b,
            Comparison::GreaterThanOrEqual => a >= b,
            Comparison::LessThan => a < b,
            Comparison::LessThanOrEqual => a <= b,
        }
    }
}

impl Operation {
    pub fn apply(&self, a: i64, b: i64) -> i64 {
        match self {
            Operation::Add => a + b,
            Operation::Subtract => a - b,
            Operation::Divide => a / b,
            Operation::Multiply => a * b,
        }
    }
}

impl TryFrom<&str> for Operation {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "+" => Ok(Operation::Add),
            "-" => Ok(Operation::Subtract),
            "/" => Ok(Operation::Divide),
            "*" => Ok(Operation::Multiply),
            _ => Err("Unsupported operation"),
        }
    }
}

impl TryFrom<&str> for Comparison {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "==" => Ok(Comparison::Equal),
            ">=" => Ok(Comparison::GreaterThanOrEqual),
            "<=" => Ok(Comparison::LessThanOrEqual),
            ">" => Ok(Comparison::GreaterThan),
            "<" => Ok(Operation::LessThan),
            _ => Err("Unsupported comparison"),
        }
    }
}
