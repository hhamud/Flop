use flop_frontend::ast::{FunctionDefinition, VariableDefinition};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Environment {
    pub functions: HashMap<String, FunctionDefinition>,
    pub variables: HashMap<String, VariableDefinition>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
            variables: HashMap::new(),
        }
    }
}
