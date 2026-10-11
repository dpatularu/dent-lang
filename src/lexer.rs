use std::{iter::Peekable, str::CharIndices};

use crate::{
    lexer_errors::LexerError::{self},
    tokens::{
        Token,
        TokenKind::{self},
    },
};
use phf::phf_map;

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
    offset: usize,
}

// Constructor
impl<'a> Lexer<'a> {
    pub fn new(source_code: &'a str) -> Self {
        Self {
            source_code,
            chars: source_code.char_indices().peekable(),
            prev: '\0',
            offset: 0,
        }
    }
}

// Error constructors
impl<'a> Lexer<'a> {
    fn make_string_termination_error(&self, literal: &str) -> Result<Token, LexerError> {
        Err(LexerError::UnterminatedString {
            span: self.offset..(self.offset + literal.len()),
        })
    }

    fn make_malformed_number_error(&self, number: &str) -> Result<Token, LexerError> {
        Err(LexerError::MalformedNumber {
            span: self.offset..(self.offset + number.len()),
        })
    }

    fn make_unknown_character_error(&self, c: char) -> Result<Token, LexerError> {
        Err(LexerError::UnknownCharacter {
            character: c,
            span: self.offset..(self.offset + c.len_utf8()),
        })
    }

    fn make_unterminated_block_comment(&self, block_comment: &str) -> Result<Token, LexerError> {
        Err(LexerError::UnterminatedBlockComment {
            span: self.offset..(self.offset + block_comment.len()),
        })
    }
}

// Helpers
impl<'a> Lexer<'a> {
    fn make_token(&self, kind: TokenKind) -> Token {
        let offset = self.offset;
        match kind {
            TokenKind::LeftParen => Token::new(kind, offset),
            TokenKind::RightParen => Token::new(kind, offset),
            TokenKind::LeftBrace => Token::new(kind, offset),
            TokenKind::RightBrace => Token::new(kind, offset),
            TokenKind::Comma => Token::new(kind, offset),
            TokenKind::Dot => Token::new(kind, offset),
            TokenKind::Minus => Token::new(kind, offset),
            TokenKind::Plus => Token::new(kind, offset),
            TokenKind::Star => Token::new(kind, offset),
            TokenKind::SemiColon => Token::new(kind, offset),
            TokenKind::Bang => Token::new(kind, offset),
            TokenKind::BangEqual => Token::new(kind, offset),
            TokenKind::Equal => Token::new(kind, offset),
            TokenKind::EqualEqual => Token::new(kind, offset),
            TokenKind::LessEqual => Token::new(kind, offset),
            TokenKind::GreaterEqual => Token::new(kind, offset),
            TokenKind::Less => Token::new(kind, offset),
            TokenKind::Greater => Token::new(kind, offset),
            TokenKind::Slash => Token::new(kind, offset),
            TokenKind::String(_) => Token::new(kind, offset),
            TokenKind::Number(_) => Token::new(kind, offset),
            TokenKind::Identifier(_) => Token::new(kind, offset),
            TokenKind::And => Token::new(kind, offset),
            TokenKind::Class => Token::new(kind, offset),
            TokenKind::Else => Token::new(kind, offset),
            TokenKind::False => Token::new(kind, offset),
            TokenKind::For => Token::new(kind, offset),
            TokenKind::Fun => Token::new(kind, offset),
            TokenKind::If => Token::new(kind, offset),
            TokenKind::Nil => Token::new(kind, offset),
            TokenKind::Or => Token::new(kind, offset),
            TokenKind::Print => Token::new(kind, offset),
            TokenKind::Return => Token::new(kind, offset),
            TokenKind::Super => Token::new(kind, offset),
            TokenKind::This => Token::new(kind, offset),
            TokenKind::True => Token::new(kind, offset),
            TokenKind::Var => Token::new(kind, offset),
            TokenKind::While => Token::new(kind, offset),
            TokenKind::LineComment(_) => Token::new(kind, offset),
            TokenKind::BlockComment(_) => Token::new(kind, offset),
        }
    }

    fn next_char(&mut self) -> Option<char> {
        let (_, c) = self.chars.next()?;
        self.prev = c;
        Some(c)
    }

    fn peek(&mut self) -> Option<char> {
        self.chars.peek().map(|(_, c)| c.to_owned())
    }

    fn advance_if<F: Fn(char) -> bool>(&mut self, f: F) -> Option<char> {
        if let Some(c) = self.peek()
            && f(c)
        {
            self.next_char()
        } else {
            None
        }
    }

    fn advance_until<F: Fn(char) -> bool>(&mut self, f: F) -> String {
        let mut string_literal = String::new();
        while let Some(c) = self.peek()
            && !f(c)
        {
            self.next_char();
            string_literal.push(c);
        }
        string_literal
    }

    fn either_or(&mut self, to_match: char, former: TokenKind, latter: TokenKind) -> Token {
        if let Some(_) = self.advance_if(|c| c == to_match) {
            self.make_token(former)
        } else {
            self.make_token(latter)
        }
    }
}

// Token producers
impl<'a> Lexer<'a> {
    fn line_comment(&mut self) -> Result<Token, LexerError> {
        self.next_char(); // Consume second slash in comment

        let comment_content = self.advance_until(|c| c == '\n');
        self.next_char(); // Consume newline;
        Ok(self.make_token(TokenKind::LineComment(comment_content.into())))
    }

    fn block_comment(&mut self) -> Result<Token, LexerError> {
        self.next_char(); // Consume first * in comment

        let mut comment = String::new();
        while let Some(c) = self.next_char() {
            comment.push(c);
            if c == '*'
                && let Some('/') = self.peek()
            {
                self.next_char(); // Consume last slash
                comment.pop(); // Remove last '*'
                break;
            } else if let None = self.peek() {
                return self.make_unterminated_block_comment(&comment);
            }
        }
        let comment: String = comment;

        Ok(self.make_token(TokenKind::BlockComment(comment)))
    }

    fn string(&mut self) -> Result<Token, LexerError> {
        let literal = self.advance_until(|c| c == '"');
        if let None = self.next_char() {
            self.make_string_termination_error(&literal)
        } else {
            Ok(self.make_token(TokenKind::String(literal)))
        }
    }

    fn number(&mut self) -> Result<Token, LexerError> {
        let mut number = String::from(self.prev);
        let first_part = self.advance_until(|n| !n.is_numeric());
        number.push_str(&first_part);
        if let Some('.') = self.peek() {
            self.next_char();
            if let Some('0'..'9') = self.peek() {
                let second_part = self.advance_until(|n| !n.is_numeric());
                number.push('.');
                number.push_str(&second_part);
            }
        }
        match number.parse() {
            Ok(n) => Ok(self.make_token(TokenKind::Number(n))),
            Err(_) => self.make_malformed_number_error(&number),
        }
    }

    fn identifier(&mut self) -> Result<Token, LexerError> {
        let mut id = String::from(self.prev);
        let rest = self.advance_until(|c| !c.is_alphanumeric());
        id.push_str(&rest);

        let token_type = KEYWORDS
            .get(&id)
            .cloned()
            .unwrap_or(TokenKind::Identifier(id.into()));

        Ok(self.make_token(token_type))
    }

    fn next_token(&mut self, char_with_index: (usize, char)) -> Option<Result<Token, LexerError>> {
        let (offset, first_char) = char_with_index;
        self.prev = first_char;
        self.offset = offset;

        if first_char.is_whitespace() {
            return None;
        }

        let token = match first_char {
            '(' => Ok(self.make_token(TokenKind::LeftParen)),
            ')' => Ok(self.make_token(TokenKind::RightParen)),
            '{' => Ok(self.make_token(TokenKind::LeftBrace)),
            '}' => Ok(self.make_token(TokenKind::RightBrace)),
            ',' => Ok(self.make_token(TokenKind::Comma)),
            '.' => {
                if let Some('0'..'9') = self.peek() {
                    self.number()
                } else {
                    Ok(self.make_token(TokenKind::Dot))
                }
            }
            '-' => Ok(self.make_token(TokenKind::Minus)),
            '+' => Ok(self.make_token(TokenKind::Plus)),
            '*' => Ok(self.make_token(TokenKind::Star)),
            ';' => Ok(self.make_token(TokenKind::SemiColon)),
            '/' => match self.peek() {
                Some('/') => self.line_comment(),
                Some('*') => self.block_comment(),
                _ => Ok(self.make_token(TokenKind::Slash)),
            },
            '<' => Ok(self.either_or('=', TokenKind::Less, TokenKind::LessEqual)),
            '>' => Ok(self.either_or('=', TokenKind::Greater, TokenKind::GreaterEqual)),
            '!' => Ok(self.either_or('=', TokenKind::Bang, TokenKind::BangEqual)),
            '=' => Ok(self.either_or('=', TokenKind::Equal, TokenKind::EqualEqual)),
            '"' => self.string(),
            n if n.is_numeric() => self.number(),
            id_start if id_start.is_alphanumeric() || id_start == '_' => self.identifier(),
            c => self.make_unknown_character_error(c),
        };

        Some(token)
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = miette::Result<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(char_with_index) = self.chars.next() {
            return self
                .next_token(char_with_index)
                .map(|token| token.map_err(|err| err.into_miette_report(self.source_code)));
        }
        None
    }
}
