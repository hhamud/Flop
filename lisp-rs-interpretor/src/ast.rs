use crate::parser::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDefinition {
    pub name: String,
    pub parameters: Vec<String>,
    pub docstrings: Option<String>,
    pub body: Node,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    pub name: String,
    pub assignment: Node,
}
