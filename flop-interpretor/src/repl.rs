use crate::env::Environment;
use crate::eval::{evaluate, EvalResult};
use ariadne::{Color, ColorGenerator, Fmt, Label, Report, ReportKind, Source};
use flop_frontend::{lexer::tokenise, parser::parse};
use std::io::{self, Write};

pub fn repl() {
    println!("Starting REPL mode...");

    let mut colors = ColorGenerator::new();

    // Generate & choose some colours for each of our elements
    let a = colors.next();
    let b = colors.next();
    let out = Color::Fixed(81);

    let mut env = Environment::new();

    loop {
        print!("> ");

        if let Err(e) = io::stdout().flush() {
            println!("Error flushing stdout: {:?}", e);
            continue;
        }

        let mut input = String::new();

        if let Err(e) = io::stdin().read_line(&mut input) {
            println!("Error reading line: {:?}", e);
            continue;
        }

        if input.trim() == "exit" || input.trim() == "quit" {
            break;
        }

        let mut tokens = tokenise(input.clone());

        match parse(&mut tokens) {
            Ok(ast) => match evaluate(&ast, &mut env) {
                Ok(EvalResult::Integer(n)) => println!("{:?}", n),
                Ok(EvalResult::StringLiteral(n)) => println!("{:?}", n),
                Ok(EvalResult::List(n)) => println!("{:?}", n),
                Ok(EvalResult::Bool(n)) => println!("{:?}", n),
                Ok(EvalResult::Void) => {}
                Err(err) => {
                    Report::build(ReportKind::Error, (), 1)
                        .with_code(3)
                        .with_message(err.to_string())
                        .with_label(
                            Label::new(0..3)
                                .with_message(err.to_string())
                                .with_color(Color::Red),
                        )
                        .finish()
                        .print(Source::from(input))
                        .unwrap();
                }
            },
            Err(e) => println!("Parsing error: {:?}", e),
        }
    }
}
