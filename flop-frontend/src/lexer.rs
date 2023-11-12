use crate::stack::Stack;

use ariadne::Span;
use std::{iter::Peekable, path::PathBuf, str::Chars};

#[derive(Debug, PartialEq)]
struct SourceInfo {
    column: Line,
    row: usize,
    namespace: PathBuf,
}

#[derive(Debug, PartialEq)]
struct Line {
    start: usize,
    end: usize,
}

impl Span for SourceInfo {
    type SourceId = PathBuf;

    fn source(&self) -> &PathBuf {
        &self.namespace
    }

    fn start(&self) -> usize {
        self.column.start
    }

    fn end(&self) -> usize {
        self.column.end
    }

    fn len(&self) -> usize {
        self.column.start - self.column.end
    }

    fn contains(&self, offset: usize) -> bool {
        todo!()
    }
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Space,
    Comment,
    Integer(i64),
    Symbol(String),
    StringLiteral(String),
    Bool(bool),
    Conditional,
    LeftRoundBracket,
    RightRoundBracket,
    LeftSquareBracket,
    RightSquareBracket,
    FunctionDefinition,
    VariableDefinition,
    DocString(String),
    Eof,
}

const SPECIAL_CHARS: [char; 5] = ['(', ')', '[', ']', '\"'];

const KEYWORDS: [&str; 3] = ["defn", "setq", "if"];

fn peek_for_keywords(chars: &mut Peekable<Chars>) -> Option<&'static str> {
    for &keyword in &KEYWORDS {
        let next_chars: String = chars.clone().take(keyword.len() + 1).collect();
        if &next_chars[1..] == keyword {
            for _ in 0..=keyword.len() {
                chars.next();
            }
            return Some(keyword);
        }
    }
    None
}

fn extract_string_content(chars: &mut Peekable<Chars>, stack: &Stack<Token>) -> Token {
    // check for docstrings
    let mut res = String::new();
    chars.next(); // skip the opening quote
    for inner_ch in chars.by_ref() {
        if inner_ch == '\"' {
            break;
        }
        res.push(inner_ch);
    }

    if *stack.last().unwrap() != Token::RightSquareBracket {
        Token::StringLiteral(res)
    } else {
        Token::DocString(res)
    }
}

fn extract_word(chars: &mut Peekable<Chars>) -> String {
    let mut word = String::new();
    while let Some(&next_char) = chars.peek() {
        if next_char.is_whitespace() || SPECIAL_CHARS.contains(&next_char) {
            break;
        }
        word.push(chars.next().unwrap());
    }
    word
}

pub fn tokenise(code: String) -> Stack<Token> {
    let mut stack = Stack::new();
    let mut chars = code.chars().peekable();
    // keep track of right and left brace pairs
    let mut counter = 0;

    while let Some(&ch) = chars.peek() {
        match ch {
            '(' => {
                if let Some(keyword) = peek_for_keywords(&mut chars) {
                    match keyword {
                        "defn" => stack.push(Token::FunctionDefinition),
                        "setq" => stack.push(Token::VariableDefinition),
                        "if" => stack.push(Token::Conditional),
                        _ => unreachable!(),
                    }
                } else {
                    counter += 1;
                    stack.push(Token::LeftRoundBracket);
                    chars.next();
                }
            }
            ')' => {
                if counter >= 1 {
                    counter -= 1;
                    stack.push(Token::RightRoundBracket);
                    chars.next();
                } else {
                    chars.next();
                }
            }
            '[' => {
                stack.push(Token::LeftSquareBracket);
                chars.next();
            }
            ']' => {
                stack.push(Token::RightSquareBracket);
                chars.next();
            }
            '\"' => {
                let string_content = extract_string_content(&mut chars, &stack);
                stack.push(string_content);
            }
            ch if ch.is_whitespace() => {
                chars.next();
            }
            ';' => {
                if chars.clone().take(2).collect::<String>() == ";;" {
                    // Skip the entire line
                    while let Some(next_char) = chars.next() {
                        if next_char == '\n' {
                            break;
                        }
                    }
                } else {
                    // It's a single semicolon, treat it as a normal character
                    chars.next();
                    stack.push(Token::Symbol(";".to_string()));
                }
            }
            _ => {
                let word = extract_word(&mut chars);
                if let Ok(i) = word.parse::<i64>() {
                    stack.push(Token::Integer(i));
                } else {
                    stack.push(Token::Symbol(word));
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
    fn test_string() {
        let code = r#"(print "hi")"#.to_string();
        let tokens = tokenise(code);
        assert_eq!(
            tokens.data,
            vec![
                Token::LeftRoundBracket,
                Token::Symbol("print".to_string()),
                Token::StringLiteral("hi".to_string()),
                Token::RightRoundBracket,
            ]
        )
    }

    #[test]
    fn test_variable() {
        let code = r#"(setq lmao "hi")"#.to_string();
        let tokens = tokenise(code);
        assert_eq!(
            tokens.data,
            vec![
                Token::VariableDefinition,
                Token::Symbol("lmao".to_string()),
                Token::StringLiteral("hi".to_string()),
            ]
        )
    }

    #[test]
    fn test_if() {
        let code = r#"(if (> 1 2) (print "1") (print 2))"#.to_string();
        let tokens = tokenise(code);
        assert_eq!(
            tokens.data,
            vec![
                Token::Conditional,
                Token::LeftRoundBracket,
                Token::Symbol(">".to_string()),
                Token::Integer(1),
                Token::Integer(2),
                Token::RightRoundBracket,
                Token::LeftRoundBracket,
                Token::Symbol("print".to_string()),
                Token::StringLiteral("1".to_string()),
                Token::RightRoundBracket,
                Token::LeftRoundBracket,
                Token::Symbol("print".to_string()),
                Token::Integer(2),
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
