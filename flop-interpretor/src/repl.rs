use crate::env::Environment;
use crate::evaluation::{evaluate_node, EvalResult};
use flop_frontend::{lexer::tokenise, parser::parse};
use miette::Result;
use std::io::{self, Write};
use std::path::PathBuf;

pub struct Repl {
    state: Environment,
}

impl Repl {
    pub fn new() -> Self {
        Self {
            state: Environment::new(),
        }
    }

    pub fn run(&mut self) -> Result<()> {
        println!("Starting REPL mode...");

        loop {
            print!("> ");

            let mut input = String::new();

            if let Err(e) = io::stdout().flush() {
                println!("Error flushing stdout: {:?}", e);
            }

            if let Err(e) = io::stdin().read_line(&mut input) {
                println!("Error reading line: {:?}", e);
            }

            if input.trim() == "exit" || input.trim() == "quit" {
                break;
            }

            // temp namespace
            let mut namespace = PathBuf::new();

            namespace.push("repl");

            let mut tokens = tokenise(&input, &namespace)?;

            let mut parse = parse(&mut tokens)?;

            while let Some(node) = parse.pop_front() {
                let eval = evaluate_node(node, &mut self.state)?;

                match eval {
                    EvalResult::Void => {}
                    EvalResult::List(n) => {
                        println!("{:?}", n.data);
                    }
                    EvalResult::Literal(n) => {
                        println!("{:?}", n);
                    }
                }
            }
        }

        Ok(())
    }
}
