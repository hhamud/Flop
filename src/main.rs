#![allow(dead_code)]

mod ast;
mod error;
mod eval;
mod file;
mod helpers;
mod lexer;
mod parser;
mod repl;
mod stack;

use crate::file::read_file;
use crate::repl::repl;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Opts {
    #[arg(short, long, help = "Use REPL mode")]
    repl: bool,

    #[arg(short, long, help = "Use read file mode")]
    file: Option<String>,
}

fn main() {
    let opts = Opts::parse();

    if opts.repl {
        println!("Starting REPL mode...");
        repl();
    } else if let Some(file) = opts.file {
        println!("Reading from file: {:?}", file);
        read_file(file);
    } else {
        println!("Please specify a mode: --repl or --file <FILE_PATH>");
    }
}
