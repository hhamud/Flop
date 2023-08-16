use crate::eval::{evaluate, EvalResult};
use crate::lexer::tokenise;
use crate::parser::Program;

pub fn eval_test(code: &str) -> Result<i64, String> {
    let mut tokens = tokenise(code.to_string());
    let mut program = Program::new();
    let ast = program.parse(&mut tokens).unwrap();
    let eval = evaluate(&ast)?;
    match eval {
        EvalResult::Integer(n) => Ok(n),
        _ => {
            panic!("failed eval test helper")
        }
    }
}
