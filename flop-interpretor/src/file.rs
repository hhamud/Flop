use crate::env::Environment;
use crate::eval::{evaluate, EvalResult};
use flop_frontend::{lexer::tokenise, parser::parse};

use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn read_file(path: impl AsRef<Path>) {
    let mut env = Environment::new();

    let mut file = File::open(path).unwrap();
    let mut content = String::new();

    file.read_to_string(&mut content).unwrap();

    let mut tokens = tokenise(content);
    let ast = parse(&mut tokens).unwrap();

    match evaluate(&ast, &mut env).unwrap() {
        EvalResult::Integer(n) => println!("{:?}", n),
        EvalResult::StringLiteral(n) => println!("{:?}", n),
        EvalResult::List(n) => println!("{:?}", n),
        EvalResult::Void => {}
        _ => panic!("failed to evaluate"),
    }
}
