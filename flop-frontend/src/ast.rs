use miette::{SourceCode, SourceSpan};

use crate::{
    error::ParseError,
    stack::Stack,
    token::{Token, TokenKind},
};

#[derive(Debug, Clone)]
pub struct FunctionDefinition {
    pub name: Token,
    pub parameters: Stack<Token>,
    pub docstrings: Token,
    pub body: Stack<Node>,
}

#[derive(Debug, Clone)]
pub struct Conditional {
    pub condition: Stack<Node>,
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

impl SourceCode for Node {
    fn read_span<'a>(
        &'a self,
        span: &miette::SourceSpan,
        context_lines_before: usize,
        context_lines_after: usize,
    ) -> Result<Box<dyn miette::SpanContents<'a> + 'a>, miette::MietteError> {
        todo!()
    }
}

impl From<Node> for SourceSpan {
    fn from(value: Node) -> Self {
        todo!()
    }
}
