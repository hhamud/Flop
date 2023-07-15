#[derive(Debug, PartialEq)]
pub enum Token {
    Integer(i64),
    Symbol(String),
    LeftRoundBracket,
    RightRoundBracket,
    LeftSquareBracket,
    RightSquareBracket,
}

#[derive(Debug)]
struct Stack {
    data: Vec<Token>,
}

impl Stack {
    fn new() -> Self {
        Self { data: Vec::new() }
    }

    fn push(&mut self, token: Token) {
        self.data.push(token);
    }

    fn pop(&mut self) -> Token {
        self.data.pop().unwrap()
    }
}

fn tokenise(code: String) -> Stack {
    let mut stack = Stack::new();

    let mut current = 0;

    let tokens = ["(", ")", "[", "]"];
    let replacement_tokens = ["( ", " )", "[ ", " ]"];

    let mut words = code;
    for (index, t) in tokens.iter().enumerate() {
        words = words.replace(t, replacement_tokens[index]);
    }

    // Split into an array of words using whitespace
    let program = words.split_whitespace();

    for word in program {
        match word {
            "(" => stack.push(Token::LeftRoundBracket),
            ")" => stack.push(Token::RightRoundBracket),
            "[" => stack.push(Token::LeftSquareBracket),
            "]" => stack.push(Token::RightSquareBracket),
            _ => {
                let i = word.parse::<i64>();
                if i.is_ok() {
                    stack.push(Token::Integer(i.unwrap()));
                } else {
                    stack.push(Token::Symbol(word.to_string()));
                }
            }
        }
    }

    stack
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexor() {
        let code = "(+ 1 2)".to_string();
        let tokens = tokenise(code);
        assert_eq!(
            tokens.data,
            vec![
                Token::LeftRoundBracket,
                Token::Symbol("+".to_string()),
                Token::Integer(1),
                Token::Integer(2),
                Token::RightRoundBracket,
            ]
        )
    }
}
