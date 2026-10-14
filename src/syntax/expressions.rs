use std::ops::Range;

use miette::Result;

use crate::runtime::{errors::{RuntimeError, RuntimeErrorKind}, value::Value};

#[derive(Debug, Clone)]
pub enum UnaryOperator {
    Negative,
    Not,
}

#[derive(Debug, Clone)]
pub enum BinaryOperator {
    Equal,
    NotEqual,
    And,
    Or,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    Plus,
    Minus,
    Multiply,
    Divide,
}

#[derive(Debug, Clone)]
pub struct Expression {
    pub kind: ExpressionKind,
    pub span: Range<usize>,
}

#[derive(Debug, Clone)]
pub enum ExpressionKind {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
    Group(Box<Expression>),
    Unary {
        operator: UnaryOperator,
        expression: Box<Expression>,
    },
    Binary {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    },
    Ternary {
        condition: Box<Expression>,
        then_branch: Box<Expression>,
        else_branch: Box<Expression>,
    },
}

impl Expression {
    pub fn evaluate(&self) -> Result<Value, RuntimeError> {
        self.kind.evaluate().map_err(|kind| RuntimeError {
            kind,
            span: self.span.clone(),
        })
    }
}

impl ExpressionKind {
    pub fn evaluate(&self) -> Result<Value, RuntimeErrorKind> {
        match self {
            ExpressionKind::Number(num) => Ok(Value::Number(*num)),
            ExpressionKind::String(s) => Ok(Value::String(s.clone())),
            ExpressionKind::Boolean(b) => Ok(Value::Boolean(*b)),
            ExpressionKind::Nil => Ok(Value::Nil),
            ExpressionKind::Group(expression) => expression.kind.evaluate(),
            ExpressionKind::Unary {
                operator,
                expression,
            } => operator.apply(expression.kind.evaluate()?),
            ExpressionKind::Binary {
                left,
                operator,
                right,
            } => operator.apply(left.kind.evaluate()?, right.kind.evaluate()?),
            ExpressionKind::Ternary {
                condition,
                then_branch,
                else_branch,
            } => {
                let condition = condition.kind.evaluate()?;
                let Value::Boolean(b) = condition else {
                    return Err(RuntimeErrorKind::TernaryConditionalError);
                };

                if b {
                    Ok(then_branch.kind.evaluate()?)
                } else {
                    Ok(else_branch.kind.evaluate()?)
                }
            }
        }
    }
}
