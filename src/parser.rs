use crate::{
    errors::ExpressionParseError,
    grammar::Expression,
    tokens::{
        Token,
        TokenKind::{self, LeftParan},
    },
};

pub struct Parser<'a> {
    tokens: Vec<Token>,
    source_code: &'a str,
    position: usize,
}

impl<'a> Parser<'a> {
    fn new(tokens: Vec<Token>, source_code: &'a str) -> Self {
        Self {
            tokens,
            source_code,
            position: 0,
        }
    }

    fn next(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.position).to_owned().map(|t| t);
        self.position += 1;
        token
    }

    fn peek(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.position).to_owned().map(|t| t);
        token
    }

    fn expression(&mut self) -> miette::Result<Expression> {
        if let Some(token) = self.next() {
            match token.kind {
                TokenKind::LeftParan => {}
                TokenKind::Minus | TokenKind::Bang => {}
                TokenKind::Number(_) => {}
                TokenKind::String(_) => {}
                TokenKind::True => {}
                TokenKind::False => {}
                TokenKind::Nil => {}
                _ => {}
            }
            Ok(Expression::True)
        } else {
            Err(ExpressionParseError {
                source_code: self.source_code.into(),
            }
            .into())
        }
    }
}
