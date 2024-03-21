use clap::Parser;
use flop_interpretor::{file::Program, repl::Repl};
use miette::Result;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Opts {
    #[arg(short, long, help = "Use read file mode")]
    file: Option<String>,
}

fn main() -> Result<()> {
    let opts = Opts::parse();

    match &opts.file {
        Some(file) => Program::new().run(file),
        None => Repl::new().run(),
    }
}
