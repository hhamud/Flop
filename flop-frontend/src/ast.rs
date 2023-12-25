use crate::{stack::Stack, token::Token};

#[derive(Debug, Clone)]
pub struct FunctionDefinition {
    pub name: Token,
    pub parameters: Stack<Token>,
    pub docstrings: Token,
    pub body: Stack<Node>,
}

#[derive(Debug, Clone)]
pub struct Conditional {
    pub condition: Stack<FunctionCall>,
    pub true_expression: Stack<Node>,
    pub false_expression: Stack<Node>,
}

#[derive(Debug, Clone)]
pub struct VariableDefinition {
    pub name: Token,
    pub assignment: Token,
}

#[derive(Debug, Clone)]
pub struct FunctionCall {
    pub name: Token,
    pub arguments: Stack<Node>,
}

#[derive(Debug, Clone)]
pub struct VariableCall {
    pub name: Token,
}

#[derive(Debug, Clone)]
pub struct List {
    pub data: Stack<Node>,
}

#[derive(Debug, Clone)]
pub struct Documentation {
    pub commentary: Token,
    pub code: Option<Stack<Node>>,
}

#[derive(Debug, Clone)]
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
