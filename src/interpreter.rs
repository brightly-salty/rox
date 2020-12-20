use crate::expr::{Binary, Expr, Grouping, Literal, Unary, Visitor};
use crate::tokens::Literal as LiteralToken;
use crate::tokens::TokenType::*;
use std::fmt;

#[derive(PartialEq, Debug, Clone)]
pub enum Value {
    String_(String),
    Bool(bool),
    Number(f64),
    Nil,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::String_(s) => write!(f, "{}", s),
            Value::Nil => write!(f, "nil"),
            Value::Number(n) => {
                let s = n.to_string();
                write!(
                    f,
                    "{}",
                    if s.ends_with(".0") {
                        &s[..(s.len() - 2)]
                    } else {
                        &s[..]
                    }
                )
            }
            Value::Bool(b) => write!(f, "{}", b),
        }
    }
}

impl From<LiteralToken> for Value {
    fn from(l: LiteralToken) -> Self {
        match l {
            LiteralToken::String_(s) => Value::String_(s),
            LiteralToken::Bool(b) => Value::Bool(b),
            LiteralToken::Number(n) => Value::Number(n),
            LiteralToken::Nil => Value::Nil,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Self {
        Self
    }

    pub fn interpret(&self, expression: Expr) {
        let value = self.evaluate(expression);
        println!("{}", value);
    }

    fn evaluate(&self, expr: Expr) -> Value {
        match expr {
            Expr::Binary(b) => self.visit_binary_expr(b),
            Expr::Grouping(g) => self.visit_grouping_expr(g),
            Expr::Literal(l) => self.visit_literal_expr(l),
            Expr::Unary(u) => self.visit_unary_expr(u),
        }
    }

    fn is_truthy(&self, value: Value) -> bool {
        match value {
            Value::Nil => false,
            Value::Bool(b) => b,
            _ => true,
        }
    }

    fn is_equal(&self, a: Value, b: Value) -> bool {
        match (a, b) {
            (Value::Nil, Value::Nil) => true,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::String_(a), Value::String_(b)) => a == b,
            (Value::Number(a), Value::Number(b)) => a == b,
            _ => false,
        }
    }
}

impl Visitor<Value> for Interpreter {
    fn visit_binary_expr(&self, expr: Binary) -> Value {
        let left = self.evaluate(*expr.left);
        let right = self.evaluate(*expr.right);
        match expr.operator.type_ {
            Minus => {
                if let (Value::Number(l), Value::Number(r)) = (left.clone(), right.clone()) {
                    Value::Number(l - r)
                } else {
                    panic!("{:?} and {:?} must be numbers", left, right);
                }
            }
            Slash => {
                if let (Value::Number(l), Value::Number(r)) = (left.clone(), right.clone()) {
                    Value::Number(l / r)
                } else {
                    panic!("{:?} and {:?} must be numbers", left, right);
                }
            }
            Star => {
                if let (Value::Number(l), Value::Number(r)) = (left.clone(), right.clone()) {
                    Value::Number(l * r)
                } else {
                    panic!("{:?} and {:?} must be numbers", left, right);
                }
            }
            Plus => {
                if let (Value::Number(l), Value::Number(r)) = (left.clone(), right.clone()) {
                    Value::Number(l + r)
                } else if let (Value::String_(l), Value::String_(r)) = (left.clone(), right.clone())
                {
                    Value::String_(l + &r)
                } else {
                    panic!(
                        "{:?} and {:?} must both be numbers or both be strings",
                        left, right
                    );
                }
            }
            Greater => {
                if let (Value::Number(l), Value::Number(r)) = (left.clone(), right.clone()) {
                    Value::Bool(l > r)
                } else {
                    panic!("{:?} and {:?} must be numbers", left, right);
                }
            }
            GreaterEqual => {
                if let (Value::Number(l), Value::Number(r)) = (left.clone(), right.clone()) {
                    Value::Bool(l >= r)
                } else {
                    panic!("{:?} and {:?} must be numbers", left, right);
                }
            }
            Less => {
                if let (Value::Number(l), Value::Number(r)) = (left.clone(), right.clone()) {
                    Value::Bool(l < r)
                } else {
                    panic!("{:?} and {:?} must be numbers", left, right);
                }
            }
            LessEqual => {
                if let (Value::Number(l), Value::Number(r)) = (left.clone(), right.clone()) {
                    Value::Bool(l <= r)
                } else {
                    panic!("{:?} and {:?} must be numbers", left, right);
                }
            }
            BangEqual => Value::Bool(!self.is_equal(left, right)),
            EqualEqual => Value::Bool(self.is_equal(left, right)),
            _ => Value::Nil,
        }
    }
    fn visit_grouping_expr(&self, expr: Grouping) -> Value {
        self.evaluate(*expr.expression)
    }
    fn visit_literal_expr(&self, expr: Literal) -> Value {
        expr.value.into()
    }
    fn visit_unary_expr(&self, expr: Unary) -> Value {
        let right = self.evaluate(*expr.right);
        match expr.operator.type_ {
            Minus => {
                if let Value::Number(n) = right {
                    Value::Number(-n)
                } else {
                    panic!("{:?} must be a number", right);
                }
            }
            Bang => Value::Bool(!self.is_truthy(right)),
            _ => Value::Nil,
        }
    }
}
