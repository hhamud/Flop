use crate::env::Environment;
use crate::eval::{evaluate, EvalResult};
use flop_frontend::{lexer::tokenise, parser::parse};

use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn read_file(path: impl AsRef<Path>) {
    let mut env = Environment::new();

    let mut file = File::open(&path).expect("Error opening file");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Error reading from file");

    let mut tokens = tokenise(content);
    let ast = parse(&mut tokens).expect("Error parsing tokens");

    let eval_result = evaluate(&ast, &mut env).expect("Error during evaluation");

    match eval_result {
        EvalResult::Integer(n) => println!("{:?}", n),
        EvalResult::StringLiteral(n) => println!("{:?}", n),
        EvalResult::List(n) => println!("{:?}", n),
        EvalResult::Void => {}
        _ => panic!("Unexpected evaluation result"),
    }
}
