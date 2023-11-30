use crate::env::Environment;
use crate::evaluation::{evaluate, EvalResult};
use flop_frontend::{ast::parse, lexer::tokenise};
use std::io::{self, Write};
use std::path::PathBuf;

pub fn repl() {
    println!("Starting REPL mode...");

    let mut env = Environment::new();

    loop {
        print!("> ");

        if let Err(e) = io::stdout().flush() {
            println!("Error flushing stdout: {:?}", e);
        }

        let mut input = String::new();

        if let Err(e) = io::stdin().read_line(&mut input) {
            println!("Error reading line: {:?}", e);
        }

        if input.trim() == "exit" || input.trim() == "quit" {
            break;
        }

        // temp namespace
        let mut namespace = PathBuf::new();
        namespace.push("repl");

        let mut tokens = tokenise(input.clone(), &namespace).unwrap();

        match parse(&mut tokens) {
            Ok(mut ast) => match evaluate(&mut ast, &mut env) {
                Ok(n) => match n {
                    EvalResult::Void => {}
                    EvalResult::Literal(n) => {
                        println!("{:?}", n);
                    }
                    EvalResult::List(n) => {
                        println!("{:?}", n);
                    }
                },
                Err(err) => {
                    eprintln!("{:?}", err)
                }
            },

            //TODO: add more detail to parsing errors
            Err(err) => {
                eprintln!("{:?}", err)
            }
        }
    }
}
