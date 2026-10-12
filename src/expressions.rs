use std::ops::Range;

use miette::Result;

use crate::runtime_errors::{RuntimeError, RuntimeErrorKind};

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

#[derive(Debug, Clone)]
pub enum Value {
    Nil,
    Number(f64),
    String(String),
    Boolean(bool),
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Number(l0), Self::Number(r0)) => l0 == r0,
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            (Self::Boolean(l0), Self::Boolean(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
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

impl BinaryOperator {
    fn apply(&self, left: Value, right: Value) -> Result<Value, RuntimeErrorKind> {
        match (left, self, right) {
            (a, Self::Equal, b) => Ok(Value::Boolean(a == b)),
            (a, Self::NotEqual, b) => Ok(Value::Boolean(a != b)),
            (Value::Number(a), Self::Plus, Value::Number(b)) => Ok(Value::Number(a + b)),
            (Value::String(a), Self::Plus, Value::String(b)) => Ok(Value::String(a + &b)),
            (Value::Number(a), Self::Minus, Value::Number(b)) => Ok(Value::Number(a - b)),
            (Value::Number(a), Self::Multiply, Value::Number(b)) => Ok(Value::Number(a * b)),
            (Value::Number(a), Self::Divide, Value::Number(b)) => Ok(Value::Number(a / b)),
            (Value::Number(a), Self::Greater, Value::Number(b)) => Ok(Value::Boolean(a > b)),
            (Value::Number(a), Self::Less, Value::Number(b)) => Ok(Value::Boolean(a < b)),
            (Value::Number(a), Self::GreaterEqual, Value::Number(b)) => Ok(Value::Boolean(a >= b)),
            (Value::Number(a), Self::LessEqual, Value::Number(b)) => Ok(Value::Boolean(a <= b)),
            (Value::Boolean(a), Self::And, Value::Boolean(b)) => Ok(Value::Boolean(a && b)),
            (Value::Boolean(a), Self::Or, Value::Boolean(b)) => Ok(Value::Boolean(a || b)),

            _ => Err(RuntimeErrorKind::BinaryApplicationError),
        }
    }
}

impl UnaryOperator {
    pub fn apply(&self, value: Value) -> Result<Value, RuntimeErrorKind> {
        match (self, value) {
            (UnaryOperator::Negative, Value::Number(a)) => Ok(Value::Number(-a)),
            (UnaryOperator::Not, Value::Boolean(b)) => Ok(Value::Boolean(!b)),
            _ => Err(RuntimeErrorKind::UnaryApplicationError),
        }
    }
}
