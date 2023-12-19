use crate::{
    error::ParseError,
    stack::Stack,
    token::{Token, TokenKind},
};

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

#[derive(Debug, Clone, PartialEq)]
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
