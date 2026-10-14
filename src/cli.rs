use std::{
    fs::{self},
    io::{self, BufRead, BufReader},
    process::exit,
};

use dent_lang::{lexer::lexer::Lexer, parser::parser::Parser};



pub struct Dent {
    args: Vec<String>,
    had_error: bool,
}

impl Dent {
    fn new(args: Vec<String>) -> Self {
        Self { had_error: false, args}
    }

    pub fn run(args: Vec<String>) {
        let mut dent = Dent::new(args);
        match dent.args.len() {
            1 => dent.execute_prompt(),
            2 => dent.execute_file(&dent.args[1]),
            _ => panic!("Usage: dent [script]"),
        }
    }

    fn execute_file(&self, file_path: &str) {
        let script = fs::read_to_string(file_path).unwrap();

        Self::execute(&script);
        if self.had_error {
            exit(65);
        }
    }

    fn execute_prompt(&mut self) {
        let stdin = io::stdin();
        let mut reader = BufReader::new(stdin);

        loop {
            print!("> ");
            let mut buffer = String::new();
            reader.read_line(&mut buffer).unwrap();
            Self::execute(&buffer);
            self.had_error = false;
        }
    }

    fn execute(source: &str) {
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
