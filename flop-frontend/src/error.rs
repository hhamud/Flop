use crate::token::Token;
use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

/// Lexer error for tokens unable to be transformed into tokens
#[derive(Debug, PartialEq, Error)]
pub enum LexerError {
    /// Incomplete String error
    #[error("")]
    IncompleteStringError(ExpectedError),

    /// syntax keyword error
    #[error("")]
    KeywordError(ExpectedError),

    /// Incomplete String error
    #[error("")]
    ExtractWordError(ExpectedError),
}

#[derive(Debug, PartialEq, Error, Diagnostic)]
pub enum ParseError {
    /// Parsing error for tokens left unprocessed in stack
    #[error("Stack Error occured")]
    StackError(ExpectedError),

    #[error("Stack Error occured")]
    /// Parsing error for tokens unable to be transformed into nodes
    TokenError(ExpectedError),
}

#[derive(Debug, PartialEq)]
pub struct ExpectedError {
    pub expected: String,
    pub found: String,
    pub token: Token,
}
