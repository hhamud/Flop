mod eval;
mod helpers;
mod lexer;
mod parser;
mod repl;

use crate::repl::repl;

fn main() {
    repl()
}
