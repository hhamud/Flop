mod eval;
mod helpers;
mod lexer;
mod parser;
mod repl;

use crate::lexer::tokenise;
use crate::parser::Program;
use crate::repl::repl;

fn main() {
    repl()
}
