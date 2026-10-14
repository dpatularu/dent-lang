use std::env;

mod cli;
use cli::Dent;

fn main() {
    let args: Vec<String> = env::args().collect();
    Dent::run(args);
}