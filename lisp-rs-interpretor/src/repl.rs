use crate::env::Environment;
use crate::eval::{evaluate, EvalResult};
use crate::lexer::tokenise;
use crate::parser::parse;
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

        match evaluate(&ast, &mut env).unwrap() {
            EvalResult::Integer(n) => println!("{:?}", n),
            EvalResult::StringLiteral(n) => println!("{:?}", n),
            EvalResult::List(n) => println!("{:?}", n),
            EvalResult::Bool(n) => println!("{:?}", n),
            EvalResult::Void => {}
            _ => panic!("failed to evaluate"),
        }
    }
}
