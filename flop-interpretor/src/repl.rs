use crate::env::Environment;
use crate::evaluation::{evaluate, EvalResult};
use ariadne::{Color, ColorGenerator, Fmt, Label, Report, ReportKind, Source};
use flop_frontend::{ast::parse, lexer::tokenise};
use std::io::{self, Write};
use std::path::PathBuf;

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
        }

        let mut input = String::new();

        if let Err(e) = io::stdin().read_line(&mut input) {
            println!("Error reading line: {:?}", e);
        }

        if input.trim() == "exit" || input.trim() == "quit" {
            break;
        }

        // temp namespace
        let mut namespace = PathBuf::new();
        namespace.push("repl");

        let mut tokens = tokenise(input.clone(), &namespace).unwrap();

        match parse(&mut tokens) {
            Ok(mut ast) => match evaluate(&mut ast, &mut env) {
                Ok(n) => match n {
                    EvalResult::Void => {}
                    EvalResult::Literal(n) => {
                        println!("{:?}", n);
                    }
                    EvalResult::List(n) => {
                        println!("{:?}", n);
                    }
                },
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

            //TODO: add more detail to parsing errors
            Err(err) => {
                Report::build(ReportKind::Error, (), err.start())
                    .with_code(3)
                    .with_message(err.reason())
                    .with_label(
                        Label::new(err.span())
                            .with_message(err.reason())
                            .with_color(Color::Red),
                    )
                    .finish()
                    .print(Source::from(input))
                    .unwrap();
            }
        }
    }
}
