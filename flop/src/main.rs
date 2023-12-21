use std::path::Path;
use clap::Parser;
use flop_interpretor::{repl::Repl, file::Program};
use miette::Result;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Opts {
    #[arg(short, long, help = "Use REPL mode")]
    repl: Option<bool>,

    #[arg(short, long, help = "Use read file mode")]
    file: Option<String>,
}

fn main() -> Result<()> {
    let opts = Opts::parse();

    match (&opts.repl, &opts.file) {
        (Some(_), _) => Repl::new().run(),
        (_, Some(file)) => Program::new().run(file),
        _ => Ok(println!(
            "Please specify a mode: --repl or --file <FILE_PATH>"
        )),
    }
}
