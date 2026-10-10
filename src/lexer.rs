use std::str::CharIndices;

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
    chars: CharIndices<'a>,
    prev: char,
    lines: usize,
    offset: usize,
}

// Constructor
impl<'a> Lexer<'a> {
    pub fn new(source_code: &'a str) -> Self {
        Self {
            source_code,
            chars: source_code.char_indices(),
            prev: '\0',
            lines: 1,
            offset: 0,
        }
    }
}

// Helpers
impl<'a> Lexer<'a> {
    fn make_token(&self, kind: TokenKind, literal: Option<&str>) -> Token {
        let line = self.lines;
        let offset = self.offset;
        match kind {
            TokenKind::LeftParan => Token::new(kind, "(", None, line, offset),
            TokenKind::RightParan => Token::new(kind, ")", None, line, offset),
            TokenKind::LeftBrace => Token::new(kind, "{", None, line, offset),
            TokenKind::RightBrace => Token::new(kind, "}", None, line, offset),
            TokenKind::Comma => Token::new(kind, ",", None, line, offset),
            TokenKind::Dot => Token::new(kind, ".", None, line, offset),
            TokenKind::Minus => Token::new(kind, "-", None, line, offset),
            TokenKind::Plus => Token::new(kind, "+", None, line, offset),
            TokenKind::Star => Token::new(kind, "*", None, line, offset),
            TokenKind::SemiColon => Token::new(kind, ";", None, line, offset),
            TokenKind::Bang => Token::new(kind, "!", None, line, offset),
            TokenKind::BangEqual => Token::new(kind, "!=", None, line, offset),
            TokenKind::Equal => Token::new(kind, "=", None, line, offset),
            TokenKind::EqualEqual => Token::new(kind, "==", None, line, offset),
            TokenKind::LessEqual => Token::new(kind, "<=", None, line, offset),
            TokenKind::GreaterEqual => Token::new(kind, ">=", None, line, offset),
            TokenKind::Less => Token::new(kind, "<", None, line, offset),
            TokenKind::Greater => Token::new(kind, ">", None, line, offset),
            TokenKind::Slash => Token::new(kind, "/", None, line, offset),
            TokenKind::Eof => Token::new(kind, "\0", None, line, offset),
            TokenKind::String => {
                let lexeme = literal.unwrap();
                let literal = lexeme.trim_matches('\"');
                Token::new(kind, lexeme, Some(literal), line, offset)
            }
            TokenKind::Number => Token::new(kind, literal.unwrap(), literal, line, offset),
            TokenKind::Identifier => Token::new(kind, literal.unwrap(), literal, line, offset),
            TokenKind::And => Token::new(kind, "and", None, line, offset),
            TokenKind::Class => Token::new(kind, "class", None, line, offset),
            TokenKind::Else => Token::new(kind, "else", None, line, offset),
            TokenKind::False => Token::new(kind, "false", None, line, offset),
            TokenKind::For => Token::new(kind, "for", None, line, offset),
            TokenKind::Fun => Token::new(kind, "fun", None, line, offset),
            TokenKind::If => Token::new(kind, "if", None, line, offset),
            TokenKind::Nil => Token::new(kind, "nil", None, line, offset),
            TokenKind::Or => Token::new(kind, "or", None, line, offset),
            TokenKind::Print => Token::new(kind, "print", None, line, offset),
            TokenKind::Return => Token::new(kind, "return", None, line, offset),
            TokenKind::Super => Token::new(kind, "super", None, line, offset),
            TokenKind::This => Token::new(kind, "this", None, line, offset),
            TokenKind::True => Token::new(kind, "true", None, line, offset),
            TokenKind::Var => Token::new(kind, "var", None, line, offset),
            TokenKind::While => Token::new(kind, "while", None, line, offset),
            TokenKind::LineComment => Token::new(
                kind,
                literal.unwrap(),
                literal.map(|s| &s[2..]).clone(),
                line,
                offset,
            ),
            TokenKind::BlockComment => Token::new(
                kind,
                literal.unwrap(),
                literal
                    .map(|s| s.trim_start_matches("/*").trim_end_matches("*/"))
                    .clone(),
                line - literal
                    .map(|s| s.chars().filter(|&c| c == '\n').count())
                    .unwrap(),
                offset,
            ),
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
        Ok(self.make_token(
            TokenKind::LineComment,
            Some(&format!("//{}", comment_content)),
        ))
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
        Ok(self.make_token(TokenKind::BlockComment, Some(&comment)))
    }

    fn slash(&mut self) -> miette::Result<Token> {
        debug_assert!(self.prev == '/');
        match self.first() {
            '/' => self.line_comment(),
            '*' => self.block_comment(),
            _ => Ok(self.make_token(TokenKind::Slash, None)),
        }
    }

    fn bang(&mut self) -> miette::Result<Token> {
        debug_assert!(self.prev == '!');
        match self.first() {
            '=' => {
                self.advance();
                Ok(self.make_token(TokenKind::BangEqual, None))
            }
            _ => Ok(self.make_token(TokenKind::Bang, None)),
        }
    }

    fn equal(&mut self) -> miette::Result<Token> {
        debug_assert!(self.prev == '=');
        match self.first() {
            '=' => {
                self.advance();
                Ok(self.make_token(TokenKind::EqualEqual, None))
            }
            _ => Ok(self.make_token(TokenKind::Equal, None)),
        }
    }

    fn greater(&mut self) -> miette::Result<Token> {
        debug_assert!(self.prev == '>');
        match self.first() {
            '=' => {
                self.advance();
                Ok(self.make_token(TokenKind::GreaterEqual, None))
            }
            _ => Ok(self.make_token(TokenKind::Greater, None)),
        }
    }

    fn less(&mut self) -> miette::Result<Token> {
        debug_assert!(self.prev == '<');
        match self.first() {
            '=' => {
                self.advance();
                Ok(self.make_token(TokenKind::LessEqual, None))
            }
            _ => Ok(self.make_token(TokenKind::Less, None)),
        }
    }

    fn string(&mut self) -> miette::Result<Token> {
        debug_assert!(self.prev == '"');
        let literal = self.grab_until(|c| c == '"', vec!['"']);
        if literal.chars().last().unwrap_or(EOF_CHAR) != '"' {
            return Err(StringTerminationError {
                source_code: self.source_code.into(),
                err_span: (self.offset, literal.len()).into(),
                token: self.make_token(TokenKind::String, Some(&format!("\"{}", literal))),
            }
            .into());
        }
        Ok(self.make_token(TokenKind::String, Some(&format!("\"{}", literal))))
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
        let token = self.make_token(TokenKind::Number, Some(&combined));
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

        let token_type = KEYWORDS.get(&id).cloned().unwrap_or(TokenKind::Identifier);
        match token_type {
            TokenKind::Identifier => Ok(self.make_token(token_type, Some(&id))),
            _ => Ok(self.make_token(token_type, None)),
        }
    }

    fn advance_token(&mut self) -> miette::Result<Token> {
        let Some((offset, first_char)) = self.advance() else {
            return Ok(self.make_token(TokenKind::Eof, None));
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
            '\0' => Ok(self.make_token(TokenKind::Eof, None)),
            '(' => Ok(self.make_token(TokenKind::LeftParan, None)),
            ')' => Ok(self.make_token(TokenKind::RightParan, None)),
            '{' => Ok(self.make_token(TokenKind::LeftBrace, None)),
            '}' => Ok(self.make_token(TokenKind::RightBrace, None)),
            ',' => Ok(self.make_token(TokenKind::Comma, None)),
            '.' => Ok(self.make_token(TokenKind::Dot, None)),
            '-' => Ok(self.make_token(TokenKind::Minus, None)),
            '+' => Ok(self.make_token(TokenKind::Plus, None)),
            '*' => Ok(self.make_token(TokenKind::Star, None)),
            ';' => Ok(self.make_token(TokenKind::SemiColon, None)),
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
