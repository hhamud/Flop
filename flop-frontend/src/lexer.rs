use crate::{
    error::LexerError,
    push_token,
    stack::Stack,
    token::{Token, TokenKind},
};
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
    col: usize,
    namespace: &PathBuf,
) -> Result<Token, LexerError> {
    // check for docstrings
    let mut res = String::new();

    chars.next(); // skip the opening quote

    for inner_ch in chars.by_ref() {
        if inner_ch == '\"' {
            break;
        }
        res.push(inner_ch);
    }

    if let Some(token) = stack.last() {
        match token.token_kind {
            TokenKind::RightSquareBracket => Ok(Token::new(
                res.as_str(),
                TokenKind::DocString,
                row,
                col,
                res.len(),
                namespace,
            )),
            _ => Ok(Token::new(
                res.as_str(),
                TokenKind::StringLiteral,
                row,
                col,
                res.len(),
                namespace,
            )),
        }
    } else {
        Err(LexerError::IncompleteStringError(Token::new(
            res.as_str(),
            TokenKind::Error,
            row,
            col.clone(),
            0,
            namespace,
        )))
    }
}

fn extract_word(
    chars: &mut Peekable<Chars>,
    row: usize,
    col: usize,
    namespace: &PathBuf,
) -> Result<String, LexerError> {
    let mut word = String::new();

    while let Some(&next_char) = chars.peek() {
        if next_char.is_whitespace() || SPECIAL_CHARS.contains(&next_char) {
            break;
        }

        if let Some(ch) = chars.next() {
            word.push(ch);
        } else {
            return Err(LexerError::ExtractWordError(Token::new(
                word.as_str(),
                TokenKind::Error,
                row,
                col,
                word.len(),
                namespace,
            )));
        }
    }

    Ok(word)
}

pub fn tokenise(code: &String, namespace: &PathBuf) -> Result<Stack<Token>, LexerError> {
    let mut stack = Stack::new();
    let mut chars = code.chars().peekable();
    // keep track of right and left brace pairs
    let mut counter = 0;

    let mut row = 0;
    let mut col = 0;

    while let Some(&ch) = chars.peek() {
        col += 1;

        match ch {
            '\n' => {
                row += 1;
                col = 0;
                chars.next();
            }
            '(' => {
                if let Some(keyword) = peek_for_keywords(&mut chars) {
                    match keyword {
                        "defn" => push_token!(
                            keyword;
                            stack,
                            keyword,
                            TokenKind::FunctionDefinition,
                            row,
                            col,
                            namespace
                        ),
                        "setq" => push_token!(
                            keyword;
                            stack,
                            keyword,
                            TokenKind::VariableDefinition,
                            row,
                            col,
                            namespace
                        ),
                        "if" => {
                            push_token!(
                                keyword;
                                stack,
                                keyword,
                                TokenKind::Conditional,
                                row,
                                col,
                                namespace
                            )
                        }
                        _ => {
                            return Err(LexerError::KeywordError(Token::new(
                                keyword,
                                TokenKind::Error,
                                row,
                                col,
                                keyword.len(),
                                namespace,
                            )))
                        }
                    };

                    col += keyword.len();
                } else {
                    counter += 1;
                    push_token!(stack, &ch, TokenKind::LeftRoundBracket, row, col, namespace);
                    chars.next();
                }
            }

            ')' => {
                if counter >= 1 {
                    counter -= 1;
                    push_token!(
                        stack,
                        &ch,
                        TokenKind::RightRoundBracket,
                        row,
                        col,
                        namespace
                    );
                    chars.next();
                } else {
                    chars.next();
                }
            }
            '[' => {
                push_token!(
                    stack,
                    &ch,
                    TokenKind::LeftSquareBracket,
                    row,
                    col,
                    namespace
                );
                chars.next();
            }
            ']' => {
                push_token!(
                    stack,
                    &ch,
                    TokenKind::RightSquareBracket,
                    row,
                    col,
                    namespace
                );

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
                    push_token!(stack, &ch, TokenKind::Symbol, row, col, namespace);
                    chars.next();
                }
            }
            _ => {
                let word = extract_word(&mut chars, row, col, &namespace)?;
                if let Ok(_) = word.parse::<i64>() {
                    stack.push(Token::new(
                        word.as_str(),
                        TokenKind::Integer,
                        row,
                        col,
                        word.len(),
                        namespace,
                    ));
                } else {
                    stack.push(Token::new(
                        word.as_str(),
                        TokenKind::Symbol,
                        row,
                        col,
                        word.len(),
                        namespace,
                    ));
                }
                col += word.len() - 1;
            }
        }
    }

    Ok(stack)
}
