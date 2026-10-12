use std::{iter::Peekable, ops::Range};

use crate::{
    expressions::{BinaryOperator, Expression, ExpressionKind, UnaryOperator},
    parser_errors::ParserError,
    tokens::{Token, TokenKind},
};

pub struct Parser<'a> {
    tokens: Peekable<std::vec::IntoIter<Token>>,
    source_code: &'a str,
    error_span: Range<usize>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token>, source_code: &'a str) -> Self {
        Self {
            tokens: tokens.into_iter().peekable(),
            source_code,
            error_span: Range::default(),
        }
    }

    fn peek(&mut self) -> Option<&Token> {
        self.tokens.peek()
    }

    fn advance(&mut self) -> Option<Token> {
        let token = self.tokens.next()?;
        self.error_span.end = token.span.end;
        Some(token)
    }

    fn take_if(&mut self, kinds: &[TokenKind]) -> Option<Token> {
        if self.peek().is_some_and(|token| kinds.contains(&token.kind)) {
            self.advance()
        } else {
            None
        }
    }

    fn synchronize(&mut self) {
        while let Some(token) = self.advance() {
            if token.kind == TokenKind::SemiColon {
                break;
            }
            let Some(token) = self.peek() else {
                break;
            };

            if matches!(
                token.kind,
                TokenKind::Class
                    | TokenKind::Fun
                    | TokenKind::Var
                    | TokenKind::For
                    | TokenKind::If
                    | TokenKind::While
                    | TokenKind::Print
                    | TokenKind::Return
            ) {
                break;
            }
        }
    }

    fn make_expression(&self, kind: ExpressionKind) -> Expression {
        Expression {
            kind,
            span: self.error_span.clone(),
        }
    }

    fn parse_binary_operator(token: &Token) -> Result<BinaryOperator, ParserError> {
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
            TokenKind::And => Ok(BinaryOperator::And),
            TokenKind::Or => Ok(BinaryOperator::Or),
            _ => Err(ParserError::UnexpectedBinaryOperator {
                span: token.span.clone(),
            }),
        }
    }

    fn parse_unary_operator(token: &Token) -> Result<UnaryOperator, ParserError> {
        match token.kind {
            TokenKind::Minus => Ok(UnaryOperator::Negative),
            TokenKind::Bang => Ok(UnaryOperator::Not),
            _ => Err(ParserError::UnexpectedUnaryOperator {
                span: token.span.clone(),
            }),
        }
    }

    fn parse_ternary(&mut self) -> Result<Expression, ParserError> {
        let condition = self.parse_comparison()?;

        if self.take_if(&[TokenKind::QuestionMark]).is_none() {
            return Ok(self.make_expression(condition));
        }

        let then_branch = self.parse_expression()?;

        if self.take_if(&[TokenKind::Colon]).is_none() {
            return Err(ParserError::UnexpectedEof);
        }

        let else_branch = self.parse_expression()?;

        let ternary = self.make_expression(ExpressionKind::Ternary {
            condition: Box::new(self.make_expression(condition)),
            then_branch: Box::new(self.make_expression(then_branch)),
            else_branch: Box::new(self.make_expression(else_branch)),
        });

        Ok(ternary)
    }

    fn parse_expression(&mut self) -> Result<ExpressionKind, ParserError> {
        let mut expression = self.parse_comparison()?;

        while let Some(token) = self.take_if(&[TokenKind::BangEqual, TokenKind::EqualEqual]) {
            let right = self.parse_comparison()?;

            expression = ExpressionKind::Binary {
                left: Box::new(self.make_expression(expression)),
                operator: Self::parse_binary_operator(&token)?,
                right: Box::new(self.make_expression(right)),
            };
        }
        Ok(expression)
    }

    fn parse_comparison(&mut self) -> Result<ExpressionKind, ParserError> {
        let mut expression = self.parse_term()?;

        while let Some(token) = self.take_if(&[
            TokenKind::Greater,
            TokenKind::GreaterEqual,
            TokenKind::Less,
            TokenKind::LessEqual,
            TokenKind::And,
            TokenKind::Or,
        ]) {
            let right = self.parse_term()?;
            expression = ExpressionKind::Binary {
                left: Box::new(self.make_expression(expression)),
                operator: Self::parse_binary_operator(&token)?,
                right: Box::new(self.make_expression(right)),
            };
        }
        Ok(expression)
    }

    fn parse_term(&mut self) -> Result<ExpressionKind, ParserError> {
        let mut expression = self.parse_factor()?;

        while let Some(token) = self.take_if(&[TokenKind::Plus, TokenKind::Minus]) {
            let right = self.parse_factor()?;
            expression = ExpressionKind::Binary {
                left: Box::new(self.make_expression(expression)),
                operator: Self::parse_binary_operator(&token)?,
                right: Box::new(self.make_expression(right)),
            };
        }
        Ok(expression)
    }

    fn parse_factor(&mut self) -> Result<ExpressionKind, ParserError> {
        let mut expression = self.parse_unary()?;

        while let Some(token) = self.take_if(&[TokenKind::Star, TokenKind::Slash]) {
            let right = self.parse_unary()?;
            expression = ExpressionKind::Binary {
                left: Box::new(self.make_expression(expression)),
                operator: Self::parse_binary_operator(&token)?,
                right: Box::new(self.make_expression(right)),
            };
        }
        Ok(expression)
    }

    fn parse_unary(&mut self) -> Result<ExpressionKind, ParserError> {
        if let Some(token) = self.take_if(&[TokenKind::Bang, TokenKind::Minus]) {
            let expression = self.parse_unary()?;
            return Ok(ExpressionKind::Unary {
                operator: Self::parse_unary_operator(&token)?,
                expression: Box::new(self.make_expression(expression)),
            });
        }

        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<ExpressionKind, ParserError> {
        let token = self.advance().ok_or(ParserError::UnexpectedEof)?;

        match token.kind {
            TokenKind::False => Ok(ExpressionKind::Boolean(false)),
            TokenKind::True => Ok(ExpressionKind::Boolean(true)),
            TokenKind::Nil => Ok(ExpressionKind::Nil),
            TokenKind::Number(number) => Ok(ExpressionKind::Number(number)),
            TokenKind::String(string) => Ok(ExpressionKind::String(string.to_string())),
            TokenKind::LeftParen => {
                let expression = self.parse_expression()?;
                if self.take_if(&[TokenKind::RightParen]).is_none() {
                    return Err(ParserError::UnclosedGroup {
                        span: self.error_span.clone(),
                    });
                }

                Ok(ExpressionKind::Group(Box::new(
                    self.make_expression(expression),
                )))
            }
            _ => Err(ParserError::UnexpectedExpression { span: token.span }),
        }
    }
}

impl Iterator for Parser<'_> {
    type Item = miette::Result<Expression>;

    fn next(&mut self) -> Option<Self::Item> {
        self.peek()?;
        self.error_span = self.peek()?.span.start..self.peek()?.span.start;
        Some(
            self.parse_ternary()
                .map_err(|err| err.into_miette_report(self.source_code)),
        )
    }
}
