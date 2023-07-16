mod lexer;
mod parser;

use crate::lexer::tokenise;
use crate::parser::Program;

fn main() {
    let code = "(+ 1 2)".to_string();
    let mut tokens = tokenise(code);
    let mut program = Program::new();

    match program.parse(&mut tokens) {
        Ok(_) => println!("{:?}", program.body),
        Err(e) => todo!()
    }
}
