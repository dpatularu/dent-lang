// use phf::phf_map;
//
// use crate::tokens::{Token, TokenKind};
//
// static KEYWORDS: phf::Map<&'static str, TokenKind> = phf_map! {
//     "and" => TokenKind::And,
//     "class" => TokenKind::Class,
//     "else" => TokenKind::Else,
//     "false" => TokenKind::False,
//     "for" => TokenKind::For,
//     "fun" => TokenKind::Fun,
//     "if" => TokenKind::If,
//     "nil" => TokenKind::Nil,
//     "or" => TokenKind::Or,
//     "print" => TokenKind::Print,
//     "return" => TokenKind::Return,
//     "super" => TokenKind::Super,
//     "this" => TokenKind::This,
//     "true" => TokenKind::True,
//     "var" => TokenKind::Var,
//     "while" => TokenKind::While,
//
// };
//
// #[derive(Debug)]
// pub struct Scanner {
//     source: Vec<char>,
//     pub tokens: Vec<Token>,
//     start: usize,
//     current: usize,
//     line: usize,
// }
//
// impl Scanner {
//     pub fn new(source: &str) -> Self {
//         Self {
//             source: source.chars().collect(),
//             tokens: vec![],
//             start: 0,
//             current: 0,
//             line: 0,
//         }
//     }
//
//     pub fn scan_tokens(&mut self) {
//         while !Self::is_at_end(&self) {
//             self.start = self.current;
//             Self::scan_token(self);
//         }
//     }
//
//     fn scan_token(&mut self) {
//         let c = self.advance();
//
//         match c {
//             '(' => self.add_token(TokenKind::LeftParan, None),
//             ')' => self.add_token(TokenKind::RightParan, None),
//             '}' => self.add_token(TokenKind::RightBrace, None),
//             '{' => self.add_token(TokenKind::LeftBrace, None),
//             '.' => self.add_token(TokenKind::Dot, None),
//             ',' => self.add_token(TokenKind::Comma, None),
//             '-' => self.add_token(TokenKind::Minus, None),
//             '+' => self.add_token(TokenKind::Plus, None),
//             ';' => self.add_token(TokenKind::SemiColon, None),
//             '*' => self.add_token(TokenKind::Star, None),
//             '!' => {
//                 if self.look_ahead('=') {
//                     self.add_token(TokenKind::BangEqual, None);
//                 } else {
//                     self.add_token(TokenKind::Bang, None);
//                 }
//             }
//             '=' => {
//                 if self.look_ahead('=') {
//                     self.add_token(TokenKind::EqualEqual, None);
//                 } else {
//                     self.add_token(TokenKind::Equal, None);
//                 }
//             }
//             '<' => {
//                 if self.look_ahead('=') {
//                     self.add_token(TokenKind::LessEqual, None);
//                 } else {
//                     self.add_token(TokenKind::Less, None);
//                 }
//             }
//             '>' => {
//                 if self.look_ahead('=') {
//                     self.add_token(TokenKind::GreaterEqual, None);
//                 } else {
//                     self.add_token(TokenKind::Greater, None);
//                 }
//             }
//             '/' => {
//                 if self.look_ahead('/') {
//                     while self.peek() != '\n' && !self.is_at_end() {
//                         self.advance();
//                     }
//                 } else {
//                     self.add_token(TokenKind::Slash, None);
//                 }
//             }
//             '\"' => {
//                 let literal = self.string();
//                 self.add_token(TokenKind::String, Some(literal))
//             }
//             ' ' | '\r' | '\t' => {}
//             '\n' => self.line += 1,
//             x => {
//                 if x.is_digit(10) {
//                     let literal = self.number();
//                     self.add_token(
//                         TokenKind::Number(literal.parse::<f64>().unwrap()),
//                         Some(literal),
//                     );
//                 } else if x.is_alphabetic() || x == '_' {
//                     let identifier = self.identifier();
//                     let token_type = KEYWORDS
//                         .get(&identifier)
//                         .cloned()
//                         .unwrap_or(TokenKind::Identifier);
//                     self.add_token(token_type, Some(identifier));
//                 } else {
//                     eprintln!("Unexpected character {}", c);
//                 }
//             }
//         }
//     }
//
//     fn add_token(&mut self, token: TokenKind, literal: Option<String>) {
//         let lexeme = self.source[self.start..self.current].iter().collect();
//
//         self.tokens
//             .push(Token::new(token, lexeme, literal, self.line));
//     }
//
//     fn advance(&mut self) -> char {
//         let c = self.source[self.current];
//         self.current += 1;
//         c
//     }
//
//     fn is_at_end(&self) -> bool {
//         self.current >= self.source.len()
//     }
//
//     fn look_ahead(&mut self, expected: char) -> bool {
//         if Self::is_at_end(&self) {
//             return false;
//         };
//         if self.source[self.current] != expected {
//             return false;
//         }
//         self.current += 1;
//         return true;
//     }
//
//     fn peek(&self) -> char {
//         if self.is_at_end() {
//             return '\0';
//         }
//         return self.source[self.current];
//     }
//
//     fn peek_next(&self) -> char {
//         if self.current + 1 >= self.source.len() {
//             return '\0';
//         }
//         return self.source[self.current + 1];
//     }
//
//     fn string(&mut self) -> String {
//         while self.peek() != '"' && !self.is_at_end() {
//             if self.peek() == '\n' {
//                 self.line += 1;
//             }
//             self.advance();
//         }
//
//         if self.is_at_end() {
//             eprintln!("Unterminated string at line {}", self.line);
//         }
//
//         self.advance();
//
//         self.source[self.start + 1..self.current - 1]
//             .iter()
//             .collect()
//     }
//
//     fn number(&mut self) -> String {
//         while self.peek().is_digit(10) {
//             self.advance();
//         }
//         if self.peek() == '.' && self.peek_next().is_digit(10) {
//             self.advance();
//         }
//         while self.peek().is_digit(10) {
//             self.advance();
//         }
//
//         self.source[self.start..self.current].iter().collect()
//     }
//
//     fn identifier(&mut self) -> String {
//         while self.peek().is_alphanumeric() {
//             self.advance();
//         }
//         self.source[self.start..self.current].iter().collect()
//     }
// }
