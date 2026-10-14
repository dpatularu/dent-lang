use std::{iter::Peekable, str::CharIndices};

use phf::phf_map;

use crate::lexer::{
    errors::LexerError,
    tokens::{Token, TokenKind},
};

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

impl<'a> Lexer<'a> {
    pub fn new(source_code: &'a str) -> Self {
        Self {
            source_code,
            chars: source_code.char_indices().peekable(),
            prev: '\0',
            offset: 0,
        }
    }

    fn current_position(&mut self) -> usize {
        self.chars
            .peek()
            .map(|(index, _)| *index)
            .unwrap_or(self.source_code.len())
    }

    fn make_token(&mut self, kind: TokenKind) -> Token {
        let start = self.offset;
        let end = self.current_position();

        Token::new(kind, start..end)
    }

    fn next_char(&mut self) -> Option<char> {
        let (_, c) = self.chars.next()?;
        self.prev = c;
        Some(c)
    }

    fn peek(&mut self) -> Option<char> {
        self.chars.peek().map(|(_, c)| *c)
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
        let mut literal = String::new();
        while let Some(c) = self.peek()
            && !f(c)
        {
            self.next_char();
            literal.push(c);
        }
        literal
    }

    fn either_or(
        &mut self,
        to_match: char,
        former: TokenKind,
        latter: TokenKind,
    ) -> Token {
        if self.advance_if(|c| c == to_match).is_some() {
            self.make_token(latter)
        } else {
            self.make_token(former)
        }
    }

    // Error constructors
    fn make_string_termination_error(
        &self,
    ) -> Result<Token, LexerError> {
        Err(LexerError::UnterminatedString {
            span: self.offset..self.source_code.len(),
        })
    }

    fn make_malformed_number_error(
        &mut self,
    ) -> Result<Token, LexerError> {
        let end = self.current_position();

        Err(LexerError::MalformedNumber {
            span: self.offset..end,
        })
    }

    fn make_unknown_character_error(
        &self,
        c: char,
    ) -> Result<Token, LexerError> {
        Err(LexerError::UnknownCharacter {
            character: c,
            span: self.offset..self.offset + c.len_utf8(),
        })
    }

    fn make_unterminated_block_comment(
        &self,
    ) -> Result<Token, LexerError> {
        Err(LexerError::UnterminatedBlockComment {
            span: self.offset..self.source_code.len(),
        })
    }

    // Token producers
    fn line_comment(&mut self) -> Result<Token, LexerError> {
        self.next_char(); // Consume the second slash.
        let comment = self.advance_until(|c| c == '\n');
        self.next_char(); // Consume newline

        Ok(self.make_token(TokenKind::LineComment(comment)))
    }

    fn block_comment(&mut self) -> Result<Token, LexerError> {
        self.next_char(); // Consume '*'.

        let mut comment = String::new();
        while let Some(c) = self.next_char() {
            if c == '*'
                && self.peek() == Some('/')
            {
                self.next_char(); // Consume '/'.
                return Ok(
                    self.make_token(TokenKind::BlockComment(comment))
                );
            }
            comment.push(c);
        }
        self.make_unterminated_block_comment()
    }

    fn string(&mut self) -> Result<Token, LexerError> {
        let literal = self.advance_until(|c| c == '"');
        if self.next_char().is_none() {
            return self.make_string_termination_error();
        }

        Ok(self.make_token(TokenKind::String(literal)))
    }

    fn number(&mut self) -> Result<Token, LexerError> {
        let mut number = String::from(self.prev);
        let first_part = self.advance_until(|c| !c.is_ascii_digit());
        number.push_str(&first_part);
        if self.peek() == Some('.') {
            self.next_char();
            if self.peek().is_some_and(|c| c.is_ascii_digit()) {
                let second_part =
                    self.advance_until(|c| !c.is_ascii_digit());
                number.push('.');
                number.push_str(&second_part);
            } else {
                number.push('.');
            }
        }
        match number.parse::<f64>() {
            Ok(n) => Ok(self.make_token(TokenKind::Number(n))),
            Err(_) => self.make_malformed_number_error(),
        }
    }

    fn identifier(&mut self) -> Result<Token, LexerError> {
        let mut id = String::from(self.prev);
        let rest = self.advance_until(|c| !c.is_alphanumeric() && c != '_');
        id.push_str(&rest);

        let kind = KEYWORDS
            .get(&id)
            .cloned()
            .unwrap_or_else(|| TokenKind::Identifier(id));

        Ok(self.make_token(kind))
    }

    fn next_token(
        &mut self,
        char_with_index: (usize, char),
    ) -> Option<Result<Token, LexerError>> {
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
                if self.peek().is_some_and(|c| c.is_ascii_digit()) {
                    self.number()
                } else {
                    Ok(self.make_token(TokenKind::Dot))
                }
            }
            '-' => Ok(self.make_token(TokenKind::Minus)),
            '+' => Ok(self.make_token(TokenKind::Plus)),
            '*' => Ok(self.make_token(TokenKind::Star)),
            ';' => Ok(self.make_token(TokenKind::SemiColon)),
            '?' => Ok(self.make_token(TokenKind::QuestionMark)),
            ':' => Ok(self.make_token(TokenKind::Colon)),
            '/' => match self.peek() {
                Some('/') => self.line_comment(),
                Some('*') => self.block_comment(),
                _ => Ok(self.make_token(TokenKind::Slash)),
            },
            '<' => Ok(self.either_or(
                '=',
                TokenKind::Less,
                TokenKind::LessEqual,
            )),
            '>' => Ok(self.either_or(
                '=',
                TokenKind::Greater,
                TokenKind::GreaterEqual,
            )),
            '!' => Ok(self.either_or(
                '=',
                TokenKind::Bang,
                TokenKind::BangEqual,
            )),
            '=' => Ok(self.either_or(
                '=',
                TokenKind::Equal,
                TokenKind::EqualEqual,
            )),
            '"' => self.string(),
            c if c.is_ascii_digit() => self.number(),
            c if c.is_alphabetic() || c == '_' => self.identifier(),
            c => self.make_unknown_character_error(c),
        };

        Some(token)
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = miette::Result<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(char_with_index) = self.chars.next() {
            if let Some(token) = self.next_token(char_with_index) {
                return Some(
                    token.map_err(|error| {
                        error.into_miette_report(self.source_code)
                    }),
                );
            }
        }
        None
    }
}
