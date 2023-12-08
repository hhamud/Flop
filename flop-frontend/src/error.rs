use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

/// Lexer error for tokens unable to be transformed into tokens
#[derive(Debug, PartialEq, Error, Diagnostic)]
pub enum LexerError {
    /// Incomplete String error
    #[error("Incomplete String error")]
    #[label("String to be either doc strings or within an expression")]
    IncompleteStringError(SourceSpan),

    /// syntax keyword error
    #[error("Unexpected keyword")]
    #[label("Valid keyword needed")]
    KeywordError(SourceSpan),

    /// Incomplete String error
    #[error("Word ended unexpectedly")]
    #[label("Found another word, check the stack")]
    ExtractWordError(SourceSpan),
}

#[derive(Debug, PartialEq, Error, Diagnostic)]
pub enum ParseError {
    /// Parsing error for tokens left unprocessed in stack
    #[error("Stack Error occured")]
    #[label("Please ")]
    StackError(SourceSpan),

    /// Parsing error for tokens unable to be transformed into nodes
    #[error("Token Error occured")]
    #[label("Please ")]
    TokenError(SourceSpan),
}

//error
//found (x happened)
// expected (this is how x should be fixed)
// token (what x is)
// should show exactly what is happening in the stack<lexer/Node>
// should show exactly what is happening in the stack
