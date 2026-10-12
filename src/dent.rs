use std::{
    fs::{self},
    io::{self, BufRead, BufReader},
    process::exit,
};

use crate::{lexer::Lexer, parser::Parser};

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
        let lexer = Lexer::new(source)
            .filter_map(|t| {
                if t.is_ok() {
                    Some(t.unwrap())
                } else {
                    println!("{:?}", t);
                    None
                }
            })
            .collect();
        let parser = Parser::new(lexer, &source);
        for token in parser {
            match token {
                Ok(t) => match t.evaluate() {
                    Ok(e) => println!("{:?}", e),
                    Err(e) => println!("{:?}", e.into_miette_report(source)),
                },
                Err(e) => {
                    println!("{:?}", e);
                }
            }
        }
    }
}
