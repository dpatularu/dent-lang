use std::{iter::Enumerate, str::Chars};

use crate::tokens::{
    Token,
    TokenKind::{self},
};
use miette::{Error, Result};
use phf::phf_map;

const EOF_CHAR: char = '\0';

static KEYWORDS: phf::Map<&'static str, TokenKind> = phf_map! {
    "and" => TokenKind::And,
    "class" => TokenKind::Class,
    "else" => TokenKind::Else,
    "false" => TokenKind::False,
    "for" => TokenKind::For,
    "fun" => TokenKind::Fun,
    "if" => TokenKind::If,
    "nil" => TokenKind::Nil,
    "or" => TokenKind::Or,
    "print" => TokenKind::Print,
    "return" => TokenKind::Return,
    "super" => TokenKind::Super,
    "this" => TokenKind::This,
    "true" => TokenKind::True,
    "var" => TokenKind::Var,
    "while" => TokenKind::While,
};

pub struct Lexer<'a> {
    chars: Enumerate<Chars<'a>>,
    prev: char,
    lines: usize,
    offset: usize,
}

// Constructor
impl<'a> Lexer<'a> {
    pub fn new(source_code: &'a str) -> Self {
        Self {
            chars: source_code.chars().enumerate(),
            prev: '\0',
            lines: 1,
            offset: 0,
        }
    }
}

// Helpers
impl<'a> Lexer<'a> {
    fn make_token(&self, kind: TokenKind, literal: Option<&str>) -> Result<Token, Error> {
        let line = self.lines;
        let offset = self.offset;
        match kind {
            TokenKind::LeftParan => Ok(Token::new(kind, "(", None, line, offset)),
            TokenKind::RightParan => Ok(Token::new(kind, ")", None, line, offset)),
            TokenKind::LeftBrace => Ok(Token::new(kind, "{", None, line, offset)),
            TokenKind::RightBrace => Ok(Token::new(kind, "}", None, line, offset)),
            TokenKind::Comma => Ok(Token::new(kind, ",", None, line, offset)),
            TokenKind::Dot => Ok(Token::new(kind, ".", None, line, offset)),
            TokenKind::Minus => Ok(Token::new(kind, "-", None, line, offset)),
            TokenKind::Plus => Ok(Token::new(kind, "+", None, line, offset)),
            TokenKind::Star => Ok(Token::new(kind, "*", None, line, offset)),
            TokenKind::SemiColon => Ok(Token::new(kind, ";", None, line, offset)),
            TokenKind::Bang => Ok(Token::new(kind, "!", None, line, offset)),
            TokenKind::BangEqual => Ok(Token::new(kind, "!=", None, line, offset)),
            TokenKind::Equal => Ok(Token::new(kind, "=", None, line, offset)),
            TokenKind::EqualEqual => Ok(Token::new(kind, "==", None, line, offset)),
            TokenKind::LessEqual => Ok(Token::new(kind, "<=", None, line, offset)),
            TokenKind::GreaterEqual => Ok(Token::new(kind, ">=", None, line, offset)),
            TokenKind::Less => Ok(Token::new(kind, "<", None, line, offset)),
            TokenKind::Greater => Ok(Token::new(kind, ">", None, line, offset)),
            TokenKind::Slash => Ok(Token::new(kind, "/", None, line, offset)),
            TokenKind::Eof => Ok(Token::new(kind, "\0", None, line, offset)),
            TokenKind::String => {
                let lexeme = literal.unwrap();
                let literal = lexeme.trim_matches('\"');
                Ok(Token::new(kind, lexeme, Some(literal), line, offset))
            }
            TokenKind::Number => {
                let parse_attempt = literal.unwrap().parse::<f64>();
                Ok(Token::new(kind, literal.unwrap(), literal, line, offset))
            }
            TokenKind::Identifier => Ok(Token::new(kind, literal.unwrap(), literal, line, offset)),
            TokenKind::And => Ok(Token::new(kind, "and", None, line, offset)),
            TokenKind::Class => Ok(Token::new(kind, "class", None, line, offset)),
            TokenKind::Else => Ok(Token::new(kind, "else", None, line, offset)),
            TokenKind::False => Ok(Token::new(kind, "false", None, line, offset)),
            TokenKind::For => Ok(Token::new(kind, "for", None, line, offset)),
            TokenKind::Fun => Ok(Token::new(kind, "fun", None, line, offset)),
            TokenKind::If => Ok(Token::new(kind, "if", None, line, offset)),
            TokenKind::Nil => Ok(Token::new(kind, "nil", None, line, offset)),
            TokenKind::Or => Ok(Token::new(kind, "or", None, line, offset)),
            TokenKind::Print => Ok(Token::new(kind, "print", None, line, offset)),
            TokenKind::Return => Ok(Token::new(kind, "return", None, line, offset)),
            TokenKind::Super => Ok(Token::new(kind, "super", None, line, offset)),
            TokenKind::This => Ok(Token::new(kind, "this", None, line, offset)),
            TokenKind::True => Ok(Token::new(kind, "true", None, line, offset)),
            TokenKind::Var => Ok(Token::new(kind, "var", None, line, offset)),
            TokenKind::While => Ok(Token::new(kind, "while", None, line, offset)),
            TokenKind::LineComment => Ok(Token::new(
                kind,
                literal.unwrap(),
                literal.map(|s| &s[2..]).clone(),
                line,
                offset,
            )),
            TokenKind::BlockComment => Ok(Token::new(
                kind,
                literal.unwrap(),
                literal
                    .map(|s| s.trim_start_matches("/*").trim_end_matches("*/"))
                    .clone(),
                line - literal
                    .map(|s| s.chars().filter(|&c| c == '\n').count())
                    .unwrap(),
                offset,
            )),
        }
    }

    fn advance(&mut self) -> Option<(usize, char)> {
        let (offset, c) = self.chars.next()?;
        if self.prev == '\n' {
            self.lines += 1;
        }
        self.prev = c;
        Some((offset, c))
    }

    fn first(&self) -> char {
        let f = self.chars.clone().next().map(|(_, c)| c);
        f.unwrap_or(EOF_CHAR)
    }

    fn grab_until<F: Fn(char) -> bool>(&mut self, f: F, include: bool) -> String {
        let mut string_literal: Vec<char> = vec![];
        while let Some((_, c)) = self.advance()
            && c != EOF_CHAR
        {
            if f(c) {
                if include {
                    string_literal.push(c);
                }
                break;
            } else {
                string_literal.push(c);
            }
        }
        string_literal.to_owned().iter().collect()
    }
}

// Token producers
impl<'a> Lexer<'a> {
    fn line_comment(&mut self) -> Result<Token, Error> {
        debug_assert!(self.prev == '/' && self.first() == '/');
        self.advance();
        let comment_content = self.grab_until(|c| c == '\n', false);
        self.make_token(
            TokenKind::LineComment,
            Some(&format!("//{}", comment_content)),
        )
    }

    fn block_comment(&mut self) -> Result<Token, Error> {
        debug_assert!(self.prev == '/' && self.first() == '*');
        let mut comment = vec![self.prev];

        while let Some((_, c)) = self.advance() {
            comment.push(c);
            if c == '*' && self.first() == '/' {
                let ending_slash = self.advance().map(|(_, c)| c).unwrap();
                comment.push(ending_slash);
                break;
            }
        }
        let comment: String = comment.to_owned().iter().collect();
        self.make_token(TokenKind::BlockComment, Some(&comment))
    }

    fn slash(&mut self) -> Result<Token, Error> {
        debug_assert!(self.prev == '/');
        match self.first() {
            '/' => self.line_comment(),
            '*' => self.block_comment(),
            _ => self.make_token(TokenKind::Slash, None),
        }
    }

    fn bang(&mut self) -> Result<Token, Error> {
        debug_assert!(self.prev == '!');
        match self.first() {
            '=' => {
                self.advance();
                self.make_token(TokenKind::BangEqual, None)
            }
            _ => self.make_token(TokenKind::Bang, None),
        }
    }

    fn equal(&mut self) -> Result<Token, Error> {
        debug_assert!(self.prev == '=');
        match self.first() {
            '=' => {
                self.advance();
                self.make_token(TokenKind::EqualEqual, None)
            }
            _ => self.make_token(TokenKind::Equal, None),
        }
    }

    fn greater(&mut self) -> Result<Token, Error> {
        debug_assert!(self.prev == '>');
        match self.first() {
            '=' => {
                self.advance();
                self.make_token(TokenKind::GreaterEqual, None)
            }
            _ => self.make_token(TokenKind::Greater, None),
        }
    }

    fn less(&mut self) -> Result<Token, Error> {
        debug_assert!(self.prev == '<');
        match self.first() {
            '=' => {
                self.advance();
                self.make_token(TokenKind::LessEqual, None)
            }
            _ => self.make_token(TokenKind::Less, None),
        }
    }

    fn string(&mut self) -> Result<Token, Error> {
        debug_assert!(self.prev == '"');
        let literal = self.grab_until(|c| c == '"', true);
        self.make_token(TokenKind::String, Some(&format!("\"{}", literal)))
    }

    fn number(&mut self, n: char) -> Result<Token, Error> {
        debug_assert!(n.is_digit(10));
        let first_part = self.grab_until(|c| !c.is_digit(10) || c == '.', true);
        if self.prev == '.' {
            let second_part = self.grab_until(|c| !c.is_digit(10), false);
            debug_assert!(second_part.len() != 0);
            let combined = format!("{}{}{}", n, first_part, second_part);
            self.make_token(TokenKind::Number, Some(&combined))
        } else {
            self.make_token(TokenKind::Number, Some(&first_part))
        }
    }

    fn identifier(&mut self, id_start: char) -> Result<Token, Error> {
        let rest = self.grab_until(|c| !c.is_alphanumeric(), false);
        let id = format!("{}{}", id_start, rest);
        let token_type = KEYWORDS.get(&id).cloned().unwrap_or(TokenKind::Identifier);
        match token_type {
            TokenKind::Identifier => self.make_token(token_type, Some(&id)),
            _ => self.make_token(token_type, None),
        }
    }

    fn advance_token(&mut self) -> Result<Token, Error> {
        let Some((offset, first_char)) = self.advance() else {
            return self.make_token(TokenKind::Eof, None);
        };
        self.offset = offset;

        let skipped_comments = match first_char {
            c if c.is_whitespace() => true,
            _ => false,
        };
        if skipped_comments {
            return self.advance_token();
        }

        let token = match first_char {
            '\0' => self.make_token(TokenKind::Eof, None),
            '(' => self.make_token(TokenKind::LeftParan, None),
            ')' => self.make_token(TokenKind::RightParan, None),
            '{' => self.make_token(TokenKind::LeftBrace, None),
            '}' => self.make_token(TokenKind::RightBrace, None),
            ',' => self.make_token(TokenKind::Comma, None),
            '.' => self.make_token(TokenKind::Dot, None),
            '-' => self.make_token(TokenKind::Minus, None),
            '+' => self.make_token(TokenKind::Plus, None),
            '*' => self.make_token(TokenKind::Star, None),
            ';' => self.make_token(TokenKind::SemiColon, None),
            '/' => self.slash(),
            '<' => self.less(),
            '>' => self.greater(),
            '!' => self.bang(),
            '=' => self.equal(),
            '"' => self.string(),
            n @ '0'..'9' => self.number(n),
            id_start if id_start.is_alphabetic() || id_start == '_' => self.identifier(id_start),
            _ => todo!(),
        };

        token
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.advance_token();
        if token.kind != TokenKind::Eof {
            Some(token)
        } else {
            None
        }
    }
}
