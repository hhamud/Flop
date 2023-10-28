use crate::lexer::Token;
use crate::parser::Node;
use thiserror::Error;

#[derive(Debug)]
pub struct TokenError {
    pub message: &'static str,
    pub token: Token,
}

#[derive(Error, Debug)]
pub enum EvalError {
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
    #[error("Unsupported node type: {}", .0)]
    Node(Node),
    #[error("Incomplete Function Definition", .0.len)]
    FunctionDefinition(Vec<Node>),
    #[error("Expected Parameter List but parameter list provided is: {}", .0)]
    Parameter(Vec<Node>),
    #[error("Expected a function name", .0)]
    FunctionName(Node),
    #[error("Empty Expression", .0)]
    EmptyExpression(Vec<Node>),
    #[error("Expected function name, operator, or expression", .0)]
    UnexpectedExpression(Vec<Node>),
    #[error("Expected a symbol node as a variable name", .0)]
    Symbol(Node),
}

#[derive(Debug)]
pub enum ParseError {
    InputError(&'static str),
    TokenError(TokenError),
}

#[derive(Debug)]
pub enum ErrorType {
    FunctionDefinition,
    VariableName,
    VariableValue,
    Parameter,
    List,
    Expression,
}

fn generate_error_message(error_type: ErrorType) -> &'static str {
    match error_type {
        ErrorType::VariableName => "Variable Definition: Expected a variable name",
        ErrorType::VariableValue => "Variable assignment: Expected a variable value",
        ErrorType::Parameter => "Parameter: Unexpected token",
        ErrorType::List => "List: Unexpected token",
        ErrorType::Expression => "Expression: Unexpected token",
        ErrorType::FunctionDefinition => "Function Definition: Unexpected token",
    }
}
