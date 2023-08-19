#![feature(peeking_next)]
use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
pub enum Token {
    Integer(i64),
    Symbol(String),
    StringLiteral(String),
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
    pub data: VecDeque<Token>,
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
    let mut chars = code.chars().peekable();
    let mut after_function_parameters = false;
    let mut skip_next_closing_paren = false;

    while let Some(&ch) = chars.peek() {
        match ch {
            '(' => {
                let mut next_chars = Vec::new();
                let mut next_chars_iter = chars.clone();
                for _ in 0..5 {
                    if let Some(&c) = next_chars_iter.peek() {
                        next_chars.push(c);
                        next_chars_iter.next();
                    }
                }

                let next_chars_string: String = next_chars.into_iter().collect();

                if next_chars_string == "(defn" {
                    stack.push(Token::FunctionDefinition);
                    for _ in 0..5 {
                        chars.next();
                    }
                    after_function_parameters = false;
                    skip_next_closing_paren = true;
                } else {
                    stack.push(Token::LeftRoundBracket);
                    chars.next();
                    after_function_parameters = false;
                }
            }
            ')' => {
                if skip_next_closing_paren {
                    skip_next_closing_paren = false;
                } else {
                    stack.push(Token::RightRoundBracket);
                }
                chars.next();
                after_function_parameters = false;
            }
            '[' => {
                stack.push(Token::LeftSquareBracket);
                chars.next();
            }
            ']' => {
                stack.push(Token::RightSquareBracket);
                chars.next();
                after_function_parameters = true;
            }
            '\"' => {
                let mut res = String::new();
                chars.next();
                while let Some(inner_ch) = chars.next() {
                    if inner_ch == '\"' {
                        break;
                    }
                    res.push(inner_ch);
                }
                if after_function_parameters {
                    stack.push(Token::DocString(res));
                } else {
                    stack.push(Token::StringLiteral(res));
                }
                after_function_parameters = false;
            }
            ch if ch.is_whitespace() => {
                chars.next();
                continue;
            }
            _ => {
                let mut word = ch.to_string();
                chars.next();

                while let Some(&next_char) = chars.peek() {
                    if next_char.is_whitespace() || ['(', ')', '[', ']', '\"'].contains(&next_char)
                    {
                        break;
                    }
                    word.push(chars.next().unwrap());
                }

                if let Ok(i) = word.parse::<i64>() {
                    stack.push(Token::Integer(i));
                } else {
                    stack.push(Token::Symbol(word));
                }
                after_function_parameters = false;
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
    fn test_string() {
        let code = r#"(print "hi")"#.to_string();
        let tokens = tokenise(code);
        assert_eq!(
            tokens.data,
            vec![
                Token::LeftRoundBracket,
                // change to function call later
                Token::Symbol("print".to_string()),
                Token::StringLiteral("hi".to_string()),
                Token::RightRoundBracket,
            ]
        )
    }

    #[test]
    fn test_function_definition() {
        let code = "(defn add [x y] (+ x y))".to_string();
        let tokens = tokenise(code);
        assert_eq!(
            tokens.data,
            vec![
                Token::FunctionDefinition,
                Token::Symbol("add".to_string()),
                Token::LeftSquareBracket,
                Token::Symbol("x".to_string()),
                Token::Symbol("y".to_string()),
                Token::RightSquareBracket,
                Token::LeftRoundBracket,
                Token::Symbol("+".to_string()),
                Token::Symbol("x".to_string()),
                Token::Symbol("y".to_string()),
                Token::RightRoundBracket,
            ]
        )
    }
}
