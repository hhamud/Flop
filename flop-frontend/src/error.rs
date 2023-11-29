use std::ops::Range;

use crate::token::{Token, TokenError};

#[derive(Debug)]
pub enum ParseError {
    /// Parsing error for tokens left unprocessed in stack
    StackError(&'static str),

    /// Parsing error for tokens unable to be transformed into nodes
    TokenError {
        /// error message
        message: &'static str,

        /// token unabled to be transformed
        token: Token,
    },
}

impl ParseError {
    pub fn span(&self) -> Token {
        match self {
            ParseError::TokenError { message, token } => token.clone(),
            ParseError::StackError(_) => todo!(),
        }
    }

    pub fn start(&self) -> usize {
        match self {
            ParseError::TokenError { message, token } => token.column.start,
            ParseError::StackError(_) => todo!(),
        }
    }

    pub fn reason(&self) -> String {
        match self {
            ParseError::TokenError { message, token } => message.to_string(),
            ParseError::StackError(_) => todo!(),
        }
    }
}

pub enum LexerError {
    /// Parsing error for tokens unable to be transformed into nodes
    TokenError {
        /// error message
        message: &'static str,

        /// token unabled to be transformed
        token: Token,
    },
}

impl LexerError {
    pub fn span(&self) -> Range<usize> {
        match self {
            LexerError::TokenError { message, token } => {
                let span = token.column.start..token.column.end;
                span
            }
        }
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParseError::StackError(msg) => write!(f, "ParseError: {}", msg),
            ParseError::TokenError { message, token, .. } => {
                write!(f, "TokenError: {} - {:?}", message, token)
            }
        }
    }
}
