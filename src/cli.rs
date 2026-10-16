use std::{
    fs::{self},
    io::{self, BufRead, BufReader},
    process::exit,
};

use lox_lang::{lexer::{Lexer, TokenKind}, parser::Parser, runtime::interpreter::Interpreter, syntax::statements::Statement};

pub struct Lox {
    args: Vec<String>,
    had_error: bool,
}

impl Lox {
    fn new(args: Vec<String>) -> Self {
        Self { had_error: false, args}
    }

    pub fn run(args: Vec<String>) {
        let mut lox = Lox::new(args);
        match lox.args.len() {
            1 => lox.execute_prompt(),
            2 => lox.execute_file(&lox.args[1]),
            _ => panic!("Usage: lox [script]"),
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
        let tokens = Lexer::new(source).filter_map(|t| {
            match t {
                Ok(token) => {
                    match token.kind {
                        TokenKind::BlockComment(_) | TokenKind::LineComment(_) => None,
                        _ => Some(token)
                    }
                },
                Err(e) => {println!("{:?}", e); None},
            }
        }).collect();
        let statements: Vec<Statement> = Parser::new(tokens, &source).filter_map(|t| {
            match t {
                Ok(statement) => Some(statement),
                Err(e) => {println!("{:?}", e); None},
            }
        }).collect();

        let mut interpreter = Interpreter::new();
        if let Err(e) =  interpreter.interpret(statements){
            println!("{}", e.into_miette_report(source))
        }
    }
}
