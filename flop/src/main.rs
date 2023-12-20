use clap::Parser;
use flop_interpretor::repl::Repl;
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
        (Some(true), _) => Repl::new().run(),

        (_, Some(_file)) => todo!(),
        //(_, Some(file)) => read_file(file),
        _ => Ok(println!(
            "Please specify a mode: --repl or --file <FILE_PATH>"
        )),
    }
}
