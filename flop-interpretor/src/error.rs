use std::num::ParseIntError;

use flop_frontend::{ast::Node, token::Token};
use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
#[error(transparent)]
pub enum EvalError {
    #[error("evalerror symbolerror")]
    SymbolError(
        #[source_code]
        #[label("Incorrect operation symbol")]
        Token,
    ),

    FunctionCallMissing(
        #[source_code]
        #[label("Missing Function Call")]
        Token,
    ),

    #[error("evalerror operationerror")]
    OperationError(
        #[source_code]
        #[label("Incorrect operation Token")]
        Node,
    ),

    #[error("evalerror literalerror")]
    LiteralError(
        #[source_code]
        #[label("Node is not a literal token")]
        Node,
    ),

    #[error("Parsing error: {0}")]
    ParsingError(ParseIntError),
}

impl From<ParseIntError> for EvalError {
    fn from(error: ParseIntError) -> Self {
        EvalError::ParsingError(error)
    }
}
