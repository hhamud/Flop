use crate::env::Environment;
use crate::eval::{evaluate, EvalResult};

use flop_frontend::{lexer::tokenise, parser::parse};
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn read_file(path: &Path) {
    let mut env = Environment::new();

    let mut file = File::open(&path).expect("Error opening file");

    let mut content = String::new();

    let _ = file
        .read_to_string(&mut content)
        .expect("Error reading from file");

    let mut tokens = tokenise(content.clone());

    match parse(&mut tokens) {
        Ok(ast) => match evaluate(&ast, &mut env) {
            Ok(EvalResult::Integer(n)) => println!("{:?}", n),
            Ok(EvalResult::StringLiteral(n)) => println!("{:?}", n),
            Ok(EvalResult::List(n)) => println!("{:?}", n),
            Ok(EvalResult::Bool(n)) => println!("{:?}", n),
            Ok(EvalResult::Void) => {}
            Err(err) => {

                // err.namespace
            }
        },
        Err(err) => {
        }
    }
}
