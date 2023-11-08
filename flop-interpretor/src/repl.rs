use crate::env::Environment;
use crate::eval::{evaluate, EvalResult};
use flop_frontend::{lexer::tokenise, parser::parse};
use std::io::{self, Write};

pub fn repl() {
    println!("Starting REPL mode...");
    let mut env = Environment::new();

    loop {
        print!("> ");

        if let Err(e) = io::stdout().flush() {
            println!("Error flushing stdout: {:?}", e);
            continue;
        }

        let mut input = String::new();

        if let Err(e) = io::stdin().read_line(&mut input) {
            println!("Error reading line: {:?}", e);
            continue;
        }

        if input.trim() == "exit" || input.trim() == "quit" {
            break;
        }

        let mut tokens = tokenise(input);

        match parse(&mut tokens) {
            Ok(ast) => match evaluate(&ast, &mut env) {
                Ok(EvalResult::Integer(n)) => println!("{:?}", n),
                Ok(EvalResult::StringLiteral(n)) => println!("{:?}", n),
                Ok(EvalResult::List(n)) => println!("{:?}", n),
                Ok(EvalResult::Bool(n)) => println!("{:?}", n),
                Ok(EvalResult::Void) => {}
                Err(e) => println!("Evaluation error: {:?}", e),
            },
            Err(e) => println!("Parsing error: {:?}", e),
        }
    }
}
