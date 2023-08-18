use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
pub enum Token {
    Integer(i64),
    Symbol(String),
    Bool(bool),
    LeftRoundBracket,
    RightRoundBracket,
    LeftSquareBracket,
    RightSquareBracket,
    FunctionDefinition,
    Body,
    Comment,
    DocString(String),
    EOF,
}

#[derive(Debug, PartialEq)]
pub struct Stack {
    data: VecDeque<Token>,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            data: Vec::new().into(),
        }
    }

    pub fn push(&mut self, token: Token) {
        self.data.push_back(token);
    }

    pub fn pop(&mut self) -> Option<Token> {
        self.data.pop_back()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn last(&self) -> Option<&Token> {
        self.data.back()
    }

    pub fn pop_front(&mut self) -> Option<Token> {
        self.data.pop_front()
    }
}

pub fn tokenise(code: String) -> Stack {
    let mut stack = Stack::new();

    let tokens = ["(", ")", "[", "]"];
    let replacement_tokens = ["( ", " )", "[ ", " ]"];

    let mut words = code;
    for (index, t) in tokens.iter().enumerate() {
        words = words.replace(t, replacement_tokens[index]);
    }

    // Split into an array of words using whitespace
    let mut program = words.split_whitespace();

    while let Some(word) = program.next() {
        match word {
            "(" => stack.push(Token::LeftRoundBracket),
            ")" => stack.push(Token::RightRoundBracket),
            "[" => stack.push(Token::LeftSquareBracket),
            "]" => stack.push(Token::RightSquareBracket),
            "defn" => stack.push(Token::FunctionDefinition),
            //"""" => stack.push(Token::DocString(s))
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

    #[test]
    fn test_lexor_list() {
        let code = "(+ [1 2])".to_string();
        let tokens = tokenise(code);
        assert_eq!(
            tokens.data,
            vec![
                Token::LeftRoundBracket,
                Token::Symbol("+".to_string()),
                Token::LeftSquareBracket,
                Token::Integer(1),
                Token::Integer(2),
                Token::RightSquareBracket,
                Token::RightRoundBracket,
            ]
        )
    }

    #[test]
    fn test_function_definition() {
        let code = "(defn hi [name] (+ 1 1))".to_string();
        let tokens = tokenise(code);
        assert_eq!(
            tokens.data,
            vec![
                Token::LeftRoundBracket,
                Token::FunctionDefinition,
                Token::Symbol("hi".to_string()),
                Token::LeftSquareBracket,
                Token::Symbol("name".to_string()),
                Token::RightSquareBracket,
                Token::LeftRoundBracket,
                Token::Symbol("+".to_string()),
                Token::Integer(1),
                Token::Integer(1),
                Token::RightRoundBracket,
                Token::RightRoundBracket,
            ]
        )
    }
}
