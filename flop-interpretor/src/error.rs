use crate::lexer::Token;
use crate::parser::Node;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EvalError<'a> {
    #[error("Variable is not defined: {}", .0)]
    Variable(String),
    #[error("Expected Integer operand: {}", .0)]
    Integer(String),
    #[error("Unsupported Math operation: {}", .0)]
    Math(String),
    #[error("Unsupported Binary operation: {}", .0)]
    Binary(String),
    #[error("Expected expression: {}", .0)]
    Expression(String),
    #[error("Insufficient operands")]
    Operands,
    #[error("Unsupported node type: {:?}", .0)]
    Node(&'a Node),
    #[error("Incomplete Function Definition: {}", .0.len())]
    FunctionDefinition(&'a Vec<Node>),
    #[error("Expected Parameter List but parameter list provided is: {:?}", .0)]
    Parameter(&'a Vec<Node>),
    #[error("Expected a function name: {:?}", .0)]
    FunctionName(&'a Node),
    #[error("Empty Expression: {:?}", .0)]
    EmptyExpression(&'a Vec<Node>),
    #[error("Expected function name, operator, or expression: {:?}", .0)]
    UnexpectedExpression(&'a Vec<Node>),
    #[error("Expected a symbol node as a variable name: {:?}", .0)]
    Symbol(&'a Node),
}

#[derive(Debug)]
pub enum ParseError {
    InputError(&'static str),
    TokenError(TokenError),
}

#[derive(Debug)]
pub struct TokenError {
    pub message: &'static str,
    pub token: Token,
}
