use crate::{runtime::{Value, errors::RuntimeErrorKind}, syntax::expressions::{BinaryOperator, UnaryOperator}};


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
