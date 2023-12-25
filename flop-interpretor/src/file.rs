use crate::{
    env::Environment,
    evaluation::{evaluate_node, EvalResult},
};

use flop_frontend::{lexer::tokenise, parser::parse};
use miette::Result;
use std::{fs::File, io::Read, path::PathBuf};

pub struct Program {
    state: Environment,
}

impl Program {
    pub fn new() -> Self {
        Self {
            state: Environment::new(),
        }
    }

    pub fn run(&mut self, path: &String) -> Result<()> {
        let mut file = File::open(&path).expect("Error opening file");

        let mut content = String::new();

        let mut namespace = PathBuf::new();

        namespace.push(&path);

        let _ = file
            .read_to_string(&mut content)
            .expect("Error reading from file");

        let mut tokens = tokenise(&content, &namespace)?;

        let mut parse = parse(&mut tokens)?;

        while let Some(node) = parse.pop_front() {
            let eval = evaluate_node(node, &mut self.state)?;

            match eval {
                EvalResult::Void => {}
                EvalResult::List(stack) => {
                    println!("{:?}", stack.data)
                }
                EvalResult::Literal(n) => {
                    println!("{:?}", n);
                }
            }
        }

        Ok(())
    }
}
