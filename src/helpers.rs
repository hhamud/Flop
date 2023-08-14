use crate::eval::evaluate;
use crate::lexer::tokenise;
use crate::parser::{Node, Program};

pub fn eval_test(code: &str) -> Result<i64, String> {
    let mut tokens = tokenise(code.to_string());
    let mut program = Program::new();
    let ast = program.parse(&mut tokens).unwrap();
    let eval = evaluate(&ast);
    Ok(eval?)
}
