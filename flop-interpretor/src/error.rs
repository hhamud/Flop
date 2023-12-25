use flop_frontend::token::Token;
use miette::Diagnostic;
use std::num::ParseIntError;
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

    #[error("Parsing error: {0}")]
    ParsingError(ParseIntError),
}

impl From<ParseIntError> for EvalError {
    fn from(error: ParseIntError) -> Self {
        EvalError::ParsingError(error)
    }
}
