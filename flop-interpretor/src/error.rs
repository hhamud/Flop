use flop_frontend::ast::Node;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EvalError {
    #[error("Variable is not defined: {}", .0)]
    Variable(String),
    #[error("Full array of {:?} for expected Integer operand {:?} and operand {:?} with symbol {:?}", .0, .1, .2, .3)]
    Integer(Vec<Node>, Node, Vec<Node>, String),
    #[error("Unsupported Math operation: {}", .0)]
    Math(String),
    #[error("Unsupported Binary operation: {}", .0)]
    Binary(String),
    #[error("Expected expression: {}", .0)]
    Expression(String),
    #[error("Insufficient operands")]
    Operands,
    #[error("Unsupported node type: {:?}", .0)]
    Node(Node),
    #[error("Incomplete Function Definition: {}", .0.len())]
    FunctionDefinition(Vec<Node>),
    #[error("Expected Parameter List but parameter list provided is: {:?}", .0)]
    Parameter(Vec<Node>),
    #[error("Expected a function name: {:?}", .0)]
    FunctionName(Node),
    #[error("Empty Expression: {:?}", .0)]
    EmptyExpression(Vec<Node>),
    #[error("Expected function name, operator, or expression: {:?}", .0)]
    UnexpectedExpression(Vec<Node>),
    #[error("Expected a symbol node as a variable name: {:?}", .0)]
    Symbol(Node),
}
