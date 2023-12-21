use crate::{
    env::Environment,
    evaluation::{evaluate, EvalResult},
};

use flop_frontend::{lexer::tokenise, parser::parse};
use miette::Result;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

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

        // temp namespace
        let mut namespace = PathBuf::new();

        namespace.push("file");

        let _ = file
            .read_to_string(&mut content)
            .expect("Error reading from file");

        let mut tokens = tokenise(&content, &namespace)?;

        let mut parse = parse(&mut tokens)?;

        let eval = evaluate(&mut parse, &mut self.state)?;

        match eval {
            EvalResult::Void => {}
            EvalResult::List(_) => todo!(),
            EvalResult::Literal(n) => {
                println!("{:?}", n);
            }
        };

        Ok(())
    }
}
