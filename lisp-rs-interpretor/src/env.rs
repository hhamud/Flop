use crate::ast::{FunctionDefinition, Variable};
use std::{collections::HashMap, rc::Rc};

#[derive(Debug)]
pub struct Environment {
    pub functions: HashMap<String, Rc<FunctionDefinition>>,
    pub variables: HashMap<String, Rc<Variable>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
            variables: HashMap::new(),
        }
    }
}
