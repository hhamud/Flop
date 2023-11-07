use crate::env::Environment;
use crate::eval::{evaluate, EvalResult};
use flop_frontend::{lexer::tokenise, parser::parse};
use std::io::{self, Write};

pub fn repl() {
    println!("Starting REPL mode...");
    let mut env = Environment::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "exit" || input.trim() == "quit" {
            break;
        }

        let mut tokens = tokenise(input);
        let ast = parse(&mut tokens).unwrap();

        match evaluate(&ast, &mut env) {
            Ok(EvalResult::Integer(n)) => println!("{:?}", n),
            Ok(EvalResult::StringLiteral(n)) => println!("{:?}", n),
            Ok(EvalResult::List(n)) => println!("{:?}", n),
            Ok(EvalResult::Bool(n)) => println!("{:?}", n),
            Ok(EvalResult::Void) => println!("void"),
            Err(e) => panic!("{e}"),
        }
    }
}
