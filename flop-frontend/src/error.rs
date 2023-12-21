use std::fmt::{Debug, Display};

use miette::Diagnostic;
use thiserror::Error;

use crate::{stack::Stack, token::Token};

#[derive(Debug, Error, Diagnostic)]
#[error(transparent)]
pub enum LexerError {
    #[error("Parse Error: Incorrect string placement")]
    #[diagnostic(help("Did you mean this {} to be a SYMBOL", .0.token))]
    IncompleteStringError(
        #[source_code]
        #[label("String has to be either a COMMENT, DOCUMENTATION STRING")]
        Token,
    ),

    KeywordError(
        #[source_code]
        #[label("Valid keyword needed")]
        Token,
    ),

    ExtractWordError(
        #[source_code]
        #[label("Found another word, check the stack")]
        Token,
    ),
}

#[derive(Debug, Error, Diagnostic)]
#[error("ParseError")]
pub enum ParseError<K>
where
    K: Debug + Display,
{
    #[error("Failed to pop stack for {name}: {stack}")]
    StackError { name: &'static str, stack: Stack<K> },

    VariableDefinition(
        #[source_code]
        #[label("Variable Definition: Expected a variable name")]
        Token,
    ),

    VariableAssignment(
        #[source_code]
        #[label("Variable Assignment: Expected a variable value")]
        Token,
    ),

    ListDefinition(
        #[source_code]
        #[label("LIST can only contain NUMBER, STRING, BOOLEAN OR another LIST")]
        Token,
    ),

    FunctionName(
        #[source_code]
        #[label("Function name must be a symbol")]
        Token,
    ),

    FunctionParameter(
        #[source_code]
        #[label("Function Parameter must be a SYMBOL")]
        Token,
    ),

    FunctionDocstring(
        #[source_code]
        #[label("Function Docstring must be a symbol")]
        Token,
    ),

    FunctionBody(
        #[label("Function body must start with a LeftRoundBracket")]
        #[source_code]
        Token,
    ),

    VariableCall(
        #[source_code]
        #[label("Variable call must be a SYMBOL")]
        Token,
    ),

    FunctionCallName(
        #[source_code]
        #[label("Function Name is not of the correct type: SYMBOL")]
        Token,
    ),

    FunctionCallArg(
        #[source_code]
        #[label("Function argument is not of the correct type: NUMBER, STRING, BOOL")]
        Token,
    ),

    ParseError(
        #[source_code]
        #[label("Wrong token")]
        Token,
    ),
}
