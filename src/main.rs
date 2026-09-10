use rust_calculator::calculator;
use rust_calculator::format_result;

use std::env;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        run_repl();
    } else {
        let expr = args.join(" ");
        match calculator::evaluate(&expr) {
            Ok(result) => println!("{}", format_result(result)),
            Err(e) => {
                eprintln!("error: {e}");
                std::process::exit(1);
            }
        }
    }
}

fn run_repl() {
    println!("Rust Calculator — enter an expression (+ - * / % ^, parentheses), or 'quit' to exit.");
    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        print!("> ");
        io::stdout().flush().ok();
        input.clear();

        if stdin.read_line(&mut input).unwrap_or(0) == 0 {
            break; // EOF (e.g. Ctrl-D)
        }

        let line = input.trim();
        if line.is_empty() {
            continue;
        }
        if matches!(line, "quit" | "exit") {
            break;
        }

        match calculator::evaluate(line) {
            Ok(result) => println!("{}", format_result(result)),
            Err(e) => println!("error: {e}"),
        }
    }
}
