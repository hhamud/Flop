use std::io::{self, Write};
use crate::lexer::tokenise;
use crate::parser::Program;
use crate::eval::evaluate;

pub fn repl() {
    loop {

        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "exit" || input.trim() == "quit" {
            break;
        }

        let mut tokens = tokenise(input);
        let mut program = Program::new();

        let ast =  program.parse(&mut tokens).unwrap();

        match evaluate(&ast) {
            Ok(result) => println!("{}", result),
            Err(e) => println!("Error: {}", e),
        }
    }
}
