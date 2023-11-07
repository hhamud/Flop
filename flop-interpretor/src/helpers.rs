use crate::env::Environment;
use crate::eval::{evaluate, EvalResult};
use flop_frontend::{lexer::tokenise, parser::parse};

pub fn eval_test(code: &str) -> Result<i64, String> {
    let mut tokens = tokenise(code.to_string());
    let ast = parse(&mut tokens).unwrap();
    let mut env = Environment::new();
    let eval = evaluate(&ast, &mut env);
    match eval {
        Ok(EvalResult::Integer(n)) => Ok(n),
        Err(e) => panic!("{e}"),
        _ => {
            panic!("failed eval test helper")
        }
    }
}
