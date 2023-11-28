use crate::token::Token;
use crate::token::{Line, TokenKind};
use crate::{stack::Stack, token::TokenError};
use std::{iter::Peekable, path::PathBuf, str::Chars};

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

fn extract_string_content(
    chars: &mut Peekable<Chars>,
    stack: &Stack<Token>,
    row: usize,
    start: usize,
    namespace: &PathBuf,
) -> Result<Token, TokenError> {
    // check for docstrings
    let mut res = String::new();

    chars.next(); // skip the opening quote

    for inner_ch in chars.by_ref() {
        if inner_ch == '\"' {
            break;
        }
        res.push(inner_ch);
    }

    let col = Line::new(start, start + res.len());

    // change this to a token to bubble up
    if let Some(token) = stack.last() {
        match token.token_kind {
            TokenKind::RightSquareBracket => Ok(Token::new(
                res.as_str(),
                TokenKind::DocString,
                row,
                col,
                namespace,
            )),
            _ => Ok(Token::new(
                res.as_str(),
                TokenKind::StringLiteral,
                row,
                col,
                namespace,
            )),
        }
    } else {
        // if no stack.last, stack is empty, (defn )
        Err(TokenError {
            expected: "String to be either doc strings or within an expression",
            found: "Incomplete string definition",
            token: Token::new(res.as_str(), TokenKind::Error, row, col, namespace),
        })
    }
}

fn extract_word(
    chars: &mut Peekable<Chars>,
    row: usize,
    col: usize,
    namespace: &PathBuf,
) -> Result<String, TokenError> {
    let mut word = String::new();
    while let Some(&next_char) = chars.peek() {
        if next_char.is_whitespace() || SPECIAL_CHARS.contains(&next_char) {
            break;
        }

        if let Some(ch) = chars.next() {
            word.push(ch);
        } else {
            return Err(TokenError {
                expected: "another word, check the stack",
                found: "Word ended unexpectedly",
                token: Token::new(
                    word.as_str(),
                    TokenKind::Error,
                    row,
                    Line::new(col, col + word.len()),
                    namespace,
                ),
            });
        }
    }
    Ok(word)
}

pub fn tokenise(code: String, namespace: &PathBuf) -> Result<Stack<Token>, TokenError> {
    let mut stack = Stack::new();
    let mut chars = code.chars().peekable();
    // keep track of right and left brace pairs
    let mut counter = 0;

    let mut row = 1;
    let mut col = 0;

    while let Some(&ch) = chars.peek() {
        col += 1;

        match ch {
            '\n' => {
                row += 1;
                col = 0;
                continue;
            }
            '(' => {
                if let Some(keyword) = peek_for_keywords(&mut chars) {
                    col += keyword.len();
                    match keyword {
                        "defn" => stack.push(Token::new(
                            keyword,
                            TokenKind::FunctionDefinition,
                            row,
                            Line::new(col - keyword.len(), col),
                            namespace,
                        )),
                        "setq" => stack.push(Token::new(
                            keyword,
                            TokenKind::VariableDefinition,
                            row,
                            Line::new(col - keyword.len(), col),
                            namespace,
                        )),
                        "if" => stack.push(Token::new(
                            keyword,
                            TokenKind::Conditional,
                            row,
                            Line::new(col - keyword.len(), col),
                            namespace,
                        )),
                        _ => {
                            return Err(TokenError {
                                expected: "Valid keyword",
                                found: "Unexpected keyword",
                                token: Token::new(
                                    keyword,
                                    TokenKind::Error,
                                    row,
                                    Line::new(col - keyword.len(), col),
                                    namespace,
                                ),
                            });
                        }
                    }
                } else {
                    counter += 1;
                    stack.push(Token::new(
                        &ch.to_string(),
                        TokenKind::LeftRoundBracket,
                        row,
                        Line::new(col - 1, col),
                        namespace,
                    ));
                    chars.next();
                }
            }

            ')' => {
                if counter >= 1 {
                    counter -= 1;
                    stack.push(Token::new(
                        &ch.to_string(),
                        TokenKind::RightRoundBracket,
                        row,
                        Line::new(col - 1, col),
                        namespace,
                    ));
                    chars.next();
                } else {
                    chars.next();
                }
            }
            '[' => {
                stack.push(Token::new(
                    &ch.to_string(),
                    TokenKind::LeftSquareBracket,
                    row,
                    Line::new(col - 1, col),
                    namespace,
                ));
                chars.next();
            }
            ']' => {
                stack.push(Token::new(
                    &ch.to_string(),
                    TokenKind::RightSquareBracket,
                    row,
                    Line::new(col - 1, col),
                    namespace,
                ));
                chars.next();
            }
            '\"' => {
                let string_content =
                    extract_string_content(&mut chars, &stack, row, col, &namespace)?;
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
                            row += 1;
                            col = 0;
                            break;
                        }
                    }
                } else {
                    // It's a single semicolon, treat it as a normal character
                    chars.next();
                    stack.push(Token::new(
                        &ch.to_string(),
                        TokenKind::Symbol,
                        row,
                        Line::new(col - 1, col),
                        namespace,
                    ));
                }
            }
            _ => {
                let word = extract_word(&mut chars, row, col, &namespace)?;
                if let Ok(i) = word.parse::<i64>() {
                    stack.push(Token::new(
                        word.as_str(),
                        TokenKind::Integer,
                        row,
                        Line::new(col, col + word.len()),
                        namespace,
                    ));
                } else {
                    stack.push(Token::new(
                        word.as_str(),
                        TokenKind::Symbol,
                        row,
                        Line::new(col, col + word.len()),
                        namespace,
                    ));
                }
            }
        }
    }

    Ok(stack)
}

