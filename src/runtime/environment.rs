use std::collections::HashMap;

use crate::runtime::{Value};

pub struct Environment {
    values: HashMap<String, Value>
}

impl Environment {
    pub fn new() -> Self {
        Self { values: HashMap::default() }
    }

    pub fn define(&mut self, name: &str, value: Value) {
        self.values.insert(name.into(), value);
    }

    pub fn get(&self, name: &str) -> Result<&Value, String> {
        match self.values.get(name) {
            Some(v) => Ok(v),
            None => Err("bad".into())
        }
    }
}