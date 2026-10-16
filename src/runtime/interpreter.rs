use crate::{runtime::{Value, environment::Environment, errors::{RuntimeError, RuntimeErrorKind}}, syntax::{expressions::{self, BinaryOperator, Expression, UnaryOperator}, statements::{Statement, StatementKind}}};


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
    environment: Environment

}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::new()
        }
    }

    pub fn interpret(&mut self, statements: Vec<Statement>) -> Result<(), RuntimeError> {
        for statement in statements{
           match statement.kind{
                StatementKind::PrintStatement(expression) => self.interpret_print(expression)?,
                StatementKind::ExpressionStatement(expression) => self.interpret_expression_statement(expression)?,
                StatementKind::VariableDeclaration(identifier, expression) => self.interpret_variable_declaration(&identifier, expression)?
            };
        }
        Ok(())
    }

    fn interpret_print(&self, expression: Expression) -> Result<(), RuntimeError>{
        println!("{}", expression.evaluate()?);
        Ok(())
    }

    fn interpret_expression_statement(&self, expression: Expression) -> Result<(), RuntimeError> {
        expression.evaluate()?;
        println!("Evaluated {}", expression.evaluate()?);
        Ok(())
    }

    fn interpret_variable_declaration(&mut self, identifier: &str, expression: Expression) -> Result<(), RuntimeError> {
        let value = expression.evaluate()?;
        self.environment.define(&identifier, value);
        Ok(())
    }
}