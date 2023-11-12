use crate::lexer::Token;

use ariadne::Span;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum ParseError {
    /// Parsing error for tokens left unprocessed in stack
    InputError(&'static str),

    /// Parsing error for tokens unable to be transformed into nodes
    TokenError {
        /// error message
        message: &'static str,

        /// token unabled to be transformed
        token: Token,
    },
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParseError::InputError(msg) => write!(f, "ParseError: {}", msg),
            ParseError::TokenError { message, token, .. } => {
                write!(f, "TokenError: {} - {:?}", message, token)
            }
        }
    }
}
