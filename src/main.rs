use core::panic;
use std::env;

mod lexer;
mod lox;
mod scanner;
mod tokens;
use lox::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut lox = Lox::new();
    match args.len() {
        1 => lox.run_prompt(),
        2 => lox.run_file(&args[1]),
        _ => panic!("Usage: rlox [script]"),
    }
}
