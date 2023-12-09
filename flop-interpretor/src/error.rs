use std::num::ParseIntError;

use flop_frontend::{ast::Node, stack::Stack, token::Token};
use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, PartialEq, Error, Diagnostic)]
#[error("EvalError")]
pub enum EvalError {
    #[label("Incorrect operation symbol")]
    SymbolError(Token),

    #[label("Incorrect operation Token")]
    OperationError(Node),

    #[label("Node is not a literal token")]
    LiteralError(Node),

    #[label("Stack failed to pop off")]
    StackError(Stack<Node>),

    #[label("Parsing error")]
    ParsingError(ParseIntError),
}

impl From<ParseIntError> for EvalError {
    fn from(error: ParseIntError) -> Self {
        EvalError::ParsingError(error)
    }
}
