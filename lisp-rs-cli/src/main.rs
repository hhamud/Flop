use lisp_rs::file::read_file;
use lisp_rs::repl::repl;

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

    match (&opts.repl, &opts.file) {
        (true, _) => repl(),
        (_, Some(file)) => read_file(file),
        _ => println!("Please specify a mode: --repl or --file <FILE_PATH>"),
    }
}
