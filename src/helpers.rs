use crate::eval::{evaluate, Environment, EvalResult};
use crate::lexer::tokenise;
use crate::parser::Parser;

pub fn eval_test(code: &str) -> Result<i64, String> {
    let mut tokens = tokenise(code.to_string());
    let mut program = Parser::new();
    let ast = program.parse(&mut tokens).unwrap();
    let mut env = Environment::new();
    let eval = evaluate(&ast, &mut env)?;
    match eval {
        EvalResult::Integer(n) => Ok(n),
        _ => {
            panic!("failed eval test helper")
        }
    }
}
