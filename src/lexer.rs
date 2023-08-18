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
    let mut chars = code.chars().peekable();
    let mut after_function_parameters = false; // Flag to check if we're after function parameters

    while let Some(ch) = chars.next() {
        match ch {
            '(' => {
                stack.push(Token::LeftRoundBracket);
                after_function_parameters = false;
            }
            ')' => {
                stack.push(Token::RightRoundBracket);
                after_function_parameters = false;
            }
            '[' => stack.push(Token::LeftSquareBracket),
            ']' => {
                stack.push(Token::RightSquareBracket);
                after_function_parameters = true; // We're potentially after function parameters now
            }
            'd' if chars.peek() == Some(&'e') => {
                let defn: String = chars.by_ref().take(4).collect();
                if defn == "efn " {
                    stack.push(Token::FunctionDefinition);
                } else {
                    stack.push(Token::Symbol(format!("d{}", defn)));
                }
                after_function_parameters = false;
            }
            '\"' => {
                let mut res = String::new();
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
            ch if ch.is_whitespace() => continue,
            _ => {
                let mut word = ch.to_string();
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
