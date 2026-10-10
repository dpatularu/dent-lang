use std::{
    fs::{self},
    io::{self, BufRead, BufReader},
    process::exit,
};

use crate::{lexer::Lexer, tokens::Token};

pub struct Dent {
    had_error: bool,
}

impl Dent {
    pub fn new() -> Self {
        Self { had_error: false }
    }

    pub fn run_file(&self, file_path: &str) {
        let script = fs::read_to_string(file_path).unwrap();

        Self::run(&script);
        if self.had_error {
            exit(65);
        }
    }

    pub fn run_prompt(&mut self) {
        let stdin = io::stdin();
        let mut reader = BufReader::new(stdin);

        loop {
            print!("> ");
            let mut buffer = String::new();
            reader.read_line(&mut buffer).unwrap();
            Self::run(&buffer);
            self.had_error = false;
        }
    }

    fn run(source: &str) {
        let mut tokens: Vec<Token> = vec![];
        let mut had_error = false;
        for token in Lexer::new(&source) {
            match token {
                Ok(t) => {
                    println!("{:?}", t);
                    tokens.push(t);
                }
                Err(e) => {
                    println!("{:?}", e);
                    had_error = true;
                }
            }
        }
        if had_error {}
    }
}
