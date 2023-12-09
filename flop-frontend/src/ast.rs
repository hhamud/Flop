use core::fmt;

use crate::{
    error::ParseError,
    stack::Stack,
    token::{Token, TokenKind},
};

use thiserror::Error;

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDefinition {
    pub name: Token,
    pub parameters: Stack<Token>,
    pub docstrings: Token,
    pub body: Stack<Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Conditional {
    pub condition: Stack<Node>,
    pub true_expression: Stack<Node>,
    pub false_expression: Stack<Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariableDefinition {
    pub name: Token,
    pub assignment: Token,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCall {
    pub name: Token,
    pub arguments: Stack<Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariableCall {
    pub name: Token,
}

#[derive(Debug, Clone, PartialEq)]
pub struct List {
    pub data: Stack<Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Documentation {
    pub commentary: Token,
    pub code: Option<Stack<Node>>,
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum Node {
    FunctionDefinition(FunctionDefinition),
    Conditional(Conditional),
    VariableDefinition(VariableDefinition),
    FunctionCall(FunctionCall),
    Literal(Token),
    VariableCall(VariableCall),
    List(List),
    Documentation(Documentation),
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::FunctionDefinition(def) => {
                write!(
                    f,
                    "Function Definition: {}, Parameters: {}, Body: [...]",
                    def.name,
                    def.parameters.data.len()
                )
            }
            Node::Conditional(cond) => {
                write!(f, "Conditional: If [...], Then [...], Else [...]")
            }
            Node::VariableDefinition(var_def) => {
                write!(
                    f,
                    "Variable Definition: {}, Assignment: {}",
                    var_def.name, var_def.assignment
                )
            }
            Node::FunctionCall(func_call) => {
                write!(f, "Function Call: {}, Arguments: [...]", func_call.name)
            }
            Node::Literal(token) => {
                write!(f, "Literal: {}", token)
            }
            Node::VariableCall(var_call) => {
                write!(f, "Variable Call: {}", var_call.name)
            }
            Node::List(list) => {
                write!(f, "List: [...]") // You might want to display elements of the list
            }
            Node::Documentation(doc) => {
                write!(f, "Documentation: {}, Code: [...]", doc.commentary)
            }
        }
    }
}
