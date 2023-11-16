use crate::lexer::Token;

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

pub enum LexerError {
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
            ParseError::StackError(msg) => write!(f, "ParseError: {}", msg),
            ParseError::TokenError { message, token, .. } => {
                write!(f, "TokenError: {} - {:?}", message, token)
            }
        }
    }
}
