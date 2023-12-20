use std::num::ParseIntError;

use flop_frontend::{ast::Node, stack::Stack, token::Token};
use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
#[error("evalerror")]
pub enum EvalError {
    SymbolError(
        #[source_code]
        #[label("Incorrect operation symbol")]
        Token,
    ),

    OperationError(
        #[source_code]
        #[label("Incorrect operation Token")]
        Node,
    ),

    LiteralError(
        #[source_code]
        #[label("Node is not a literal token")]
        Node,
    ),

    #[label("Stack failed to pop off ")]
    StackError(Stack<Node>),

    #[label("Parsing error: {.0}")]
    ParsingError(ParseIntError),
}

impl From<ParseIntError> for EvalError {
    fn from(error: ParseIntError) -> Self {
        EvalError::ParsingError(error)
    }
}
