use clap::Parser;
//use flop_interpretor::{file::read_file, repl::repl};
use flop_interpretor::{evaluation::EvalResult, repl::repl};
use miette::Result;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Opts {
    #[arg(short, long, help = "Use REPL mode")]
    repl: bool,

    #[arg(short, long, help = "Use read file mode")]
    file: Option<String>,
}

fn main() -> Result<()> {
    let opts = Opts::parse();

    match (&opts.repl, &opts.file) {
        (true, _) => loop {
            let res = repl()?;

            match res {
                EvalResult::Void | EvalResult::List(_) => todo!(),
                EvalResult::Literal(n) => {
                    println!("{:?}", n);
                }
            };
        },

        (_, Some(_file)) => todo!(),
        //(_, Some(file)) => read_file(file),
        _ => Ok(println!(
            "Please specify a mode: --repl or --file <FILE_PATH>"
        )),
    }
}
