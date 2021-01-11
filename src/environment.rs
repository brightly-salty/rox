use crate::tokens::Token;
use crate::value::Value;
use anyhow::Result;
use std::collections::HashMap;

#[derive(Clone, PartialEq, Debug, Default)]
pub struct Environment {
    enclosing: Option<Box<Environment>>,
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn new_from(enclosing: Self) -> Self {
        Self {
            enclosing: Some(Box::new(enclosing)),
            values: HashMap::default(),
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &Token) -> Result<Value> {
        if self.values.contains_key(&name.lexeme) {
            Ok(self.values.get(&name.lexeme).unwrap().clone())
        } else if self.enclosing.is_some() {
            self.enclosing.clone().unwrap().get(name)
        } else {
            Err(anyhow!(format!("Undefined variable '{}'.", name.lexeme)))
        }
    }

    pub fn assign(&mut self, name: Token, value: Value) -> Result<()> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme, value);
            Ok(())
        } else if self.enclosing.is_some() {
            self.enclosing.clone().unwrap().assign(name, value)
        } else {
            Err(anyhow!(format!("Undefined variable '{}'.", name.lexeme)))
        }
    }
}
