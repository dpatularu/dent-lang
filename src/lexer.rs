use std::{iter::Peekable, str::CharIndices};

use crate::{
    errors::{MalformedNumberError, SingleTokenError, StringTerminationError},
    tokens::{
        Token,
        TokenKind::{self},
    },
};
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
    source_code: &'a str,
    chars: Peekable<CharIndices<'a>>,
    prev: char,
    lines: usize,
    offset: usize,
}

// Constructor
impl<'a> Lexer<'a> {
    pub fn new(source_code: &'a str) -> Self {
        Self {
            source_code,
            chars: source_code.char_indices().peekable(),
            prev: '\0',
            lines: 1,
            offset: 0,
        }
    }
}

// Helpers
impl<'a> Lexer<'a> {
    fn make_token(&self, kind: TokenKind) -> Token {
        let line = self.lines;
        let offset = self.offset;
        match kind {
            TokenKind::LeftParan => Token::new(kind, line, offset),
            TokenKind::RightParan => Token::new(kind, line, offset),
            TokenKind::LeftBrace => Token::new(kind, line, offset),
            TokenKind::RightBrace => Token::new(kind, line, offset),
            TokenKind::Comma => Token::new(kind, line, offset),
            TokenKind::Dot => Token::new(kind, line, offset),
            TokenKind::Minus => Token::new(kind, line, offset),
            TokenKind::Plus => Token::new(kind, line, offset),
            TokenKind::Star => Token::new(kind, line, offset),
            TokenKind::SemiColon => Token::new(kind, line, offset),
            TokenKind::Bang => Token::new(kind, line, offset),
            TokenKind::BangEqual => Token::new(kind, line, offset),
            TokenKind::Equal => Token::new(kind, line, offset),
            TokenKind::EqualEqual => Token::new(kind, line, offset),
            TokenKind::LessEqual => Token::new(kind, line, offset),
            TokenKind::GreaterEqual => Token::new(kind, line, offset),
            TokenKind::Less => Token::new(kind, line, offset),
            TokenKind::Greater => Token::new(kind, line, offset),
            TokenKind::Slash => Token::new(kind, line, offset),
            TokenKind::Eof => Token::new(kind, line, offset),
            TokenKind::String(_) => Token::new(kind, line, offset),
            TokenKind::Number(_) => Token::new(kind, line, offset),
            TokenKind::Identifier(_) => Token::new(kind, line, offset),
            TokenKind::And => Token::new(kind, line, offset),
            TokenKind::Class => Token::new(kind, line, offset),
            TokenKind::Else => Token::new(kind, line, offset),
            TokenKind::False => Token::new(kind, line, offset),
            TokenKind::For => Token::new(kind, line, offset),
            TokenKind::Fun => Token::new(kind, line, offset),
            TokenKind::If => Token::new(kind, line, offset),
            TokenKind::Nil => Token::new(kind, line, offset),
            TokenKind::Or => Token::new(kind, line, offset),
            TokenKind::Print => Token::new(kind, line, offset),
            TokenKind::Return => Token::new(kind, line, offset),
            TokenKind::Super => Token::new(kind, line, offset),
            TokenKind::This => Token::new(kind, line, offset),
            TokenKind::True => Token::new(kind, line, offset),
            TokenKind::Var => Token::new(kind, line, offset),
            TokenKind::While => Token::new(kind, line, offset),
            TokenKind::LineComment(_) => Token::new(kind, line, offset),
            TokenKind::BlockComment(_) => Token::new(kind, line, offset),
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

    fn grab_until<F: Fn(char) -> bool>(&mut self, f: F, include: Vec<char>) -> String {
        let mut string_literal: Vec<char> = vec![];
        while let Some((_, c)) = self.advance()
            && c != EOF_CHAR
        {
            if f(c) {
                if include.contains(&c) {
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
    fn line_comment(&mut self) -> miette::Result<Token> {
        debug_assert!(self.prev == '/' && self.first() == '/');
        self.advance();
        let comment_content = self.grab_until(|c| c == '\n', vec![]);
        Ok(self.make_token(TokenKind::LineComment(comment_content.into())))
    }

    fn block_comment(&mut self) -> miette::Result<Token> {
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
        let comment = comment
            .trim_start_matches("/*")
            .trim_end_matches("*/")
            .to_owned();

        Ok(self.make_token(TokenKind::BlockComment(comment)))
    }

    fn slash(&mut self) -> miette::Result<Token> {
        debug_assert!(self.prev == '/');
        match self.first() {
            '/' => self.line_comment(),
            '*' => self.block_comment(),
            _ => Ok(self.make_token(TokenKind::Slash)),
        }
    }

    fn bang(&mut self) -> miette::Result<Token> {
        debug_assert!(self.prev == '!');
        match self.first() {
            '=' => {
                self.advance();
                Ok(self.make_token(TokenKind::BangEqual))
            }
            _ => Ok(self.make_token(TokenKind::Bang)),
        }
    }

    fn equal(&mut self) -> miette::Result<Token> {
        debug_assert!(self.prev == '=');
        match self.first() {
            '=' => {
                self.advance();
                Ok(self.make_token(TokenKind::EqualEqual))
            }
            _ => Ok(self.make_token(TokenKind::Equal)),
        }
    }

    fn greater(&mut self) -> miette::Result<Token> {
        debug_assert!(self.prev == '>');
        match self.first() {
            '=' => {
                self.advance();
                Ok(self.make_token(TokenKind::GreaterEqual))
            }
            _ => Ok(self.make_token(TokenKind::Greater)),
        }
    }

    fn less(&mut self) -> miette::Result<Token> {
        debug_assert!(self.prev == '<');
        match self.first() {
            '=' => {
                self.advance();
                Ok(self.make_token(TokenKind::LessEqual))
            }
            _ => Ok(self.make_token(TokenKind::Less)),
        }
    }

    fn string(&mut self) -> miette::Result<Token> {
        debug_assert!(self.prev == '"');
        let literal = self.grab_until(|c| c == '"', vec!['"']);
        if literal.chars().last().unwrap_or(EOF_CHAR) != '"' {
            return Err(StringTerminationError {
                source_code: self.source_code.into(),
                err_span: (self.offset, literal.len()).into(),
                token: self.make_token(TokenKind::String(literal)),
            }
            .into());
        }
        Ok(self.make_token(TokenKind::String(literal)))
    }

    fn number(&mut self, n: char) -> miette::Result<Token> {
        debug_assert!(n.is_digit(10));
        let first_part = self.grab_until(|c| !c.is_digit(10), vec!['.']);
        let combined: String = if self.prev == '.' {
            let second_part = self.grab_until(|c| !c.is_digit(10), vec![]);
            format!("{}{}{}", n, first_part, second_part)
        } else {
            format!("{}{}", n, first_part)
        };
        let token = self.make_token(TokenKind::Number(combined.parse().unwrap()));
        if combined.parse::<f64>().is_err() {
            return Err(MalformedNumberError {
                source_code: self.source_code.into(),
                err_span: (self.offset, combined.len()).into(),
                token,
            }
            .into());
        };
        Ok(token)
    }

    fn identifier(&mut self, id_start: char) -> miette::Result<Token> {
        let rest = self.grab_until(|c| !c.is_alphanumeric(), vec![]);
        let id = format!("{}{}", id_start, rest);

        let token_type = KEYWORDS
            .get(&id)
            .cloned()
            .unwrap_or(TokenKind::Identifier(id.into()));
        match token_type {
            TokenKind::Identifier(_) => Ok(self.make_token(token_type)),
            _ => Ok(self.make_token(token_type)),
        }
    }

    fn advance_token(&mut self) -> miette::Result<Token> {
        let Some((offset, first_char)) = self.advance() else {
            return Ok(self.make_token(TokenKind::Eof));
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
            '\0' => Ok(self.make_token(TokenKind::Eof)),
            '(' => Ok(self.make_token(TokenKind::LeftParan)),
            ')' => Ok(self.make_token(TokenKind::RightParan)),
            '{' => Ok(self.make_token(TokenKind::LeftBrace)),
            '}' => Ok(self.make_token(TokenKind::RightBrace)),
            ',' => Ok(self.make_token(TokenKind::Comma)),
            '.' => Ok(self.make_token(TokenKind::Dot)),
            '-' => Ok(self.make_token(TokenKind::Minus)),
            '+' => Ok(self.make_token(TokenKind::Plus)),
            '*' => Ok(self.make_token(TokenKind::Star)),
            ';' => Ok(self.make_token(TokenKind::SemiColon)),
            '/' => self.slash(),
            '<' => self.less(),
            '>' => self.greater(),
            '!' => self.bang(),
            '=' => self.equal(),
            '"' => self.string(),
            n @ '0'..'9' => self.number(n),
            id_start if id_start.is_alphanumeric() || id_start == '_' => self.identifier(id_start),
            c => Err(SingleTokenError {
                source_code: self.source_code.into(),
                bad_char: c,
                err_span: (self.offset, c.len_utf8()).into(),
            }
            .into()),
        };

        token
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = miette::Result<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.advance_token();
        if token.is_ok() {
            let token = token.unwrap();
            if token.kind != TokenKind::Eof {
                Some(Ok(token))
            } else {
                None
            }
        } else {
            Some(token)
        }
    }
}
