use crate::token::Token;

#[derive(Debug, PartialEq)]
pub enum Error {
    /// Parsing error for tokens left unprocessed in stack
    StackError(ExpectedError),

    /// Parsing error for tokens unable to be transformed into nodes
    TokenError(ExpectedError),

    /// Lexer error for tokens unable to be transformed into tokens
    LexerError(ExpectedError),

    /// Evaluation errors for nodes that are unable to be executed
    EvalError(ExpectedError),

    /// Operation and comparison error
    OperationError(ExpectedError),
}

#[derive(Debug, PartialEq)]
pub struct ExpectedError {
    pub expected: &'static str,
    pub found: &'static str,
    pub token: Token,
}
