use std::env;

mod cli;
use cli::Lox;

fn main() {
    let args: Vec<String> = env::args().collect();
    Lox::run(args);
}