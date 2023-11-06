use crate::lexer::Token;

#[derive(Debug)]
pub enum ParseError {
    InputError(&'static str),
    TokenError(TokenError),
}

#[derive(Debug)]
pub struct TokenError {
    pub message: &'static str,
    pub token: Token,
}
