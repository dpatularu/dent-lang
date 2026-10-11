use std::{iter::Peekable, vec::IntoIter};

use crate::{
    expressions::{BinaryOperator, Expression, UnaryOperator},
    parser_errors::ParserError,
    tokens::{
        Token,
        TokenKind::{self},
    },
};

pub struct Parser<'a> {
    tokens: Peekable<IntoIter<Token>>,
    source_code: &'a str,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token>, source_code: &'a str) -> Self {
        Self {
            tokens: tokens.into_iter().peekable(),
            source_code,
        }
    }

    fn next(&mut self) -> Option<Token> {
        self.tokens.next()
    }

    fn peek(&mut self) -> Option<&Token> {
        self.tokens.peek()
    }

    fn next_token_is_either(&mut self, tokens_to_match: &[TokenKind]) -> Option<Token> {
        if let Some(t) = self.tokens.next_if(|t| tokens_to_match.contains(&t.kind)) {
            Some(t.clone())
        } else {
            None
        }
    }

    pub fn parse_binary_operator(&mut self, token: &Token) -> Result<BinaryOperator, ParserError> {
        match token.kind {
            TokenKind::EqualEqual => Ok(BinaryOperator::Equal),
            TokenKind::BangEqual => Ok(BinaryOperator::NotEqual),
            TokenKind::Less => Ok(BinaryOperator::Less),
            TokenKind::LessEqual => Ok(BinaryOperator::LessEqual),
            TokenKind::Greater => Ok(BinaryOperator::Greater),
            TokenKind::GreaterEqual => Ok(BinaryOperator::GreaterEqual),
            TokenKind::Plus => Ok(BinaryOperator::Plus),
            TokenKind::Minus => Ok(BinaryOperator::Minus),
            TokenKind::Star => Ok(BinaryOperator::Multiply),
            TokenKind::Slash => Ok(BinaryOperator::Divide),
            _ => Err(ParserError::UnexpectedBinaryOperator {
                span: token.offset..token.offset,
            }),
        }
    }

    fn parse_unary_operator(&self, token: &Token) -> Result<UnaryOperator, ParserError> {
        match token.kind {
            TokenKind::Minus => Ok(UnaryOperator::Negative),
            TokenKind::Bang => Ok(UnaryOperator::Not),
            _ => Err(ParserError::UnexpectedUnaryOperator {
                span: token.offset..token.offset,
            }),
        }
    }

    fn parse_expression(&mut self) -> Result<Expression, ParserError> {
        let mut expression = self.parse_comparison()?;
        while let Some(token) =
            self.next_token_is_either(&[TokenKind::BangEqual, TokenKind::EqualEqual])
        {
            let right = self.parse_comparison()?;
            expression = Expression::Binary {
                left: Box::new(expression),
                operator: self.parse_binary_operator(&token)?,
                right: Box::new(right),
            };
        }
        Ok(expression)
    }

    fn parse_comparison(&mut self) -> Result<Expression, ParserError> {
        let mut expression = self.parse_term()?;

        while let Some(token) = self.next_token_is_either(&[
            TokenKind::Greater,
            TokenKind::GreaterEqual,
            TokenKind::Less,
            TokenKind::LessEqual,
        ]) {
            let right = self.parse_term()?;
            expression = Expression::Binary {
                left: Box::new(expression),
                operator: self.parse_binary_operator(&token)?,
                right: Box::new(right),
            };
        }
        Ok(expression)
    }

    fn parse_term(&mut self) -> Result<Expression, ParserError> {
        let mut expression = self.parse_factor()?;

        while let Some(token) = self.next_token_is_either(&[TokenKind::Minus, TokenKind::Plus]) {
            let right = self.parse_factor()?;
            expression = Expression::Binary {
                left: Box::new(expression),
                operator: self.parse_binary_operator(&token)?,
                right: Box::new(right),
            };
        }
        Ok(expression)
    }
    fn parse_factor(&mut self) -> Result<Expression, ParserError> {
        let mut expression = self.parse_unary()?;

        while let Some(token) = self.next_token_is_either(&[TokenKind::Slash, TokenKind::Star]) {
            let right = self.parse_unary()?;
            expression = Expression::Binary {
                left: Box::new(expression),
                operator: self.parse_binary_operator(&token)?,
                right: Box::new(right),
            };
        }
        Ok(expression)
    }

    fn parse_unary(&mut self) -> Result<Expression, ParserError> {
        if let Some(token) = self.next_token_is_either(&[TokenKind::Bang, TokenKind::Minus]) {
            let right = self.parse_unary()?;
            return Ok(Expression::Unary {
                operator: self.parse_unary_operator(&token)?,
                expression: Box::new(right),
            });
        }
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<Expression, ParserError> {
        let token = self.next().unwrap();
        if let Some(token) = self.next() {
            match token.kind {
                TokenKind::False => Ok(Expression::Boolean(false)),
                TokenKind::True => Ok(Expression::Boolean(true)),
                TokenKind::Nil => Ok(Expression::Nil),
                TokenKind::Number(num) => Ok(Expression::Number(num)),
                TokenKind::String(s) => Ok(Expression::String(s)),
                TokenKind::LeftParen => {
                    let expression = self.parse_expression()?;
                    if self
                        .next_token_is_either(&[TokenKind::RightParen])
                        .is_none()
                    {
                        return Err(ParserError::UnclosedGroup {
                            span: token.offset..token.offset,
                        });
                    }

                    Ok(Expression::Group(Box::new(expression)))
                }
                _ => Err(ParserError::UnexpectedLiteral {
                    span: token.offset..token.offset,
                }),
            }
        } else {
            Err(ParserError::UnexpectedLiteral {
                span: token.offset..token.offset,
            })
        }
    }
}

impl Iterator for Parser<'_> {
    type Item = miette::Result<Expression>;

    fn next(&mut self) -> Option<Self::Item> {
        self.tokens.peek()?;

        Some(
            self.parse_expression()
                .map_err(|err| err.into_miette_report(self.source_code)),
        )
    }
}
