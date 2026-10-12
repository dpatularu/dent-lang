use core::panic;
use std::env;

mod dent;
mod expressions;
mod interpreter;
mod lexer;
mod lexer_errors;
mod lexer_miette_errors;
mod parser;
mod parser_errors;
mod parser_miette_errors;
mod precedence;
mod runtime_errors;
mod runtime_miette_errors;
mod scanner;
mod statements;
mod tokens;
use dent::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut dent = Dent::new();
    match args.len() {
        1 => dent.run_prompt(),
        2 => dent.run_file(&args[1]),
        _ => panic!("Usage: dent [script]"),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn a() {
        println!("{:?}", "02".parse::<f64>());
    }
}
