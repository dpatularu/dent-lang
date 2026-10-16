use std::{iter::Peekable, ops::Range};

use crate::{lexer::tokens::{Token, TokenKind}, parser::errors::ParserError, syntax::{expressions::{BinaryOperator, Expression, ExpressionKind, UnaryOperator}, statements::{Statement, StatementKind}}};

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
    
    fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        let Some(token) = self.peek() else {
            return Err(ParserError::UnexpectedEof)
        };


        let statement_kind = match &token.kind {
            TokenKind::True
            | TokenKind::False
            | TokenKind::Nil
            | TokenKind::String(_)
            | TokenKind::Number(_)
            | TokenKind::LeftParen
            | TokenKind::Identifier(_)
             => self.parse_expression_statement(),
            TokenKind::Print => self.parse_print(),
            TokenKind::Var => self.parse_variable_declaration(),
             _ => todo!()
        };
        if statement_kind.is_err(){
            self.synchronize();
        }else{
            self.advance();
        }

        Ok(Statement::new(statement_kind?, self.error_span.clone()))

    }

    fn parse_print(&mut self) -> Result<StatementKind, ParserError>{
        self.advance();
        let expression = self.parse_ternary()?;
        if self.take_if(&[TokenKind::SemiColon]).is_none() {
            return Err(ParserError::ExpectedSemicolon { span: expression.span.end..expression.span.end });
        }
        Ok(StatementKind::PrintStatement(expression))
        
    }

    fn parse_expression_statement(&mut self) -> Result<StatementKind, ParserError> {
        let expression = self.parse_ternary()?;
        if self.take_if(&[TokenKind::SemiColon]).is_none() {
            return Err(ParserError::ExpectedSemicolon { span: expression.span.end..expression.span.end });
        }
        Ok(StatementKind::ExpressionStatement(expression))
    }

    fn parse_variable_declaration(&mut self) -> Result<StatementKind, ParserError> {
        self.advance();
        let Some(identifier) = self.advance() else {
            return Err(ParserError::UnexpectedEof);
        };
        match identifier.kind {
            TokenKind::Identifier(id) => {
                if self.take_if(&[TokenKind::Equal]).is_none() {
                    return Err(ParserError::ExpectedEqual { span: self.error_span.end..self.error_span.end });
                }
                let expression = self.parse_ternary()?;
                if self.take_if(&[TokenKind::SemiColon]).is_none() {
                    return Err(ParserError::ExpectedSemicolon { span: expression.span.end..expression.span.end });
                }
                Ok(StatementKind::VariableDeclaration(id, expression))
            },
            _ => return Err(ParserError::ExpectedIdentifier { span: identifier.span}),
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
    let t = self.error_span.clone();
    let token = self.peek().ok_or(ParserError::IncompleteBinaryOperation{span: t})?;

    match &token.kind {
        TokenKind::False => {
            self.advance();
            Ok(ExpressionKind::Boolean(false))
        }
        TokenKind::True => {
            self.advance();
            Ok(ExpressionKind::Boolean(true))
        }
        TokenKind::Nil => {
            self.advance();
            Ok(ExpressionKind::Nil)
        }
        TokenKind::Number(number) => {
            let number = *number;
            self.advance();
            Ok(ExpressionKind::Number(number))
        }
        TokenKind::String(string) => {
            let string = string.clone();
            self.advance();
            Ok(ExpressionKind::String(string))
        }
        TokenKind::LeftParen => {
            self.advance();

            let expression = self.parse_expression()?;

            if self.take_if(&[TokenKind::RightParen]).is_none() {
                return Err(ParserError::UnclosedGroup {
                    span: self.error_span.clone(),
                });
            }

            Ok(ExpressionKind::Group(Box::new(
                self.make_expression(expression),
            )))
        },
        TokenKind::Identifier(id) =>{
            let id = id.clone();
            self.advance();
            Ok(ExpressionKind::Identifier(id))
        }
        _ => Err(ParserError::UnexpectedExpression {
            span: token.span.clone(),
        }),
    }
}
}

impl Iterator for Parser<'_> {
    type Item = miette::Result<Statement>;

    fn next(&mut self) -> Option<Self::Item> {
        self.peek()?;
        self.error_span = self.peek()?.span.start..self.peek()?.span.start;
        Some(
            self.parse_statement()
                .map_err(|err| err.into_miette_report(self.source_code)),
        )
    }
}
