use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

use crate::{stack::Stack, token::Token};

#[derive(Debug, Error, Diagnostic)]
#[error("LexerError")]
#[diagnostic(
    help("lmao"),
    code(oops::my::bad)
)]
pub struct LexerError {

    #[label("String to be either doc strings or within an expression")]
    pub bad: SourceSpan,
}

//#[derive(Debug, PartialEq, Error, Diagnostic)]
//#[error("LexerError")]
//pub enum LexerError {
//#[label("String to be either doc strings or within an expression")]
//IncompleteStringError(SourceSpan),
//
//#[label("Valid keyword needed")]
//KeywordError(Token),
//
//#[label("Found another word, check the stack")]
//ExtractWordError(Token),
//}

#[derive(Debug, PartialEq, Error, Diagnostic)]
#[error("ParseError")]
pub enum ParseError<K>
where
    K: std::fmt::Debug + std::fmt::Display,
{
    #[error("Failed to pop stack for {name}: {stack}")]
    StackError { name: &'static str, stack: Stack<K> },

    #[label("Variable Definition: Expected a variable name")]
    VariableDefinition(SourceSpan),

    #[label("Variable Assignment: Expected a variable value")]
    VariableAssignment(SourceSpan),

    #[label("LIST can only contain NUMBER, STRING, BOOLEAN OR another LIST")]
    ListDefinition(Token),

    #[label("Function name must be a symbol")]
    FunctionName(Token),

    #[label("Function Parameter must be a SYMBOL")]
    FunctionParameter(Token),

    #[label("Function Docstring must be a symbol")]
    FunctionDocstring(Token),

    #[label("Function body must start with a LeftRoundBracket")]
    FunctionBody(Token),

    #[label("Variable call must be a SYMBOL")]
    VariableCall(Token),

    #[label("Function Name is not of the correct type: SYMBOL")]
    FunctionCallName(Token),

    #[label("Function argument is not of the correct type: NUMBER, STRING, BOOL")]
    FunctionCallArg(Token),

    #[label("Wrong token")]
    ParseError(Token),
}
