use crate::env::Environment;
use crate::evaluation::{evaluate, EvalResult};
use flop_frontend::{lexer::tokenise, parser::parse};
use miette::Result;
use std::io::{self, Write};
use std::path::PathBuf;

pub fn repl() -> Result<EvalResult> {
    println!("Starting REPL mode...");

    let mut env = Environment::new();

    print!("> ");

    if let Err(e) = io::stdout().flush() {
        println!("Error flushing stdout: {:?}", e);
    }

    let mut input = String::new();

    if let Err(e) = io::stdin().read_line(&mut input) {
        println!("Error reading line: {:?}", e);
    }

    if input.trim() == "exit" || input.trim() == "quit" {
        return Ok(EvalResult::Void);
        //break;
    }

    // temp namespace
    let mut namespace = PathBuf::new();
    namespace.push("repl");

    let mut tokens = tokenise(input.clone(), &namespace)?;

    let mut parse = parse(&mut tokens)?;

    let eval = evaluate(&mut parse, &mut env)?;

    Ok(eval)
}
