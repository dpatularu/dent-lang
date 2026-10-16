use crate::{runtime::{Value, errors::{RuntimeError, RuntimeErrorKind}}, syntax::{expressions::{self, BinaryOperator, Expression, UnaryOperator}, statements::{Statement, StatementKind}}};


impl BinaryOperator {
    pub fn apply(&self, left: Value, right: Value) -> Result<Value, RuntimeErrorKind> {
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

pub struct Interpreter {

}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn interpret(&mut self, statements: Vec<Statement>) -> Result<(), RuntimeError> {
        for statement in statements{
           match statement.kind{
                StatementKind::PrintStatement(expression) => Self::interpret_print(expression)?,
                StatementKind::ExpressionStatement(expression) => Self::interpret_expression_statement(expression)?,
                StatementKind::VariableDeclaration(expression) => Self::interpret_expression_statement(expression)?
            };
        }
        Ok(())
    }

    fn interpret_print(expression: Expression) -> Result<(), RuntimeError>{
        println!("{}", expression.evaluate()?);
        Ok(())
    }

    fn interpret_expression_statement(expression: Expression) -> Result<(), RuntimeError> {
        expression.evaluate()?;
        println!("Evaluated {}", expression.evaluate()?);
        Ok(())
    }
}