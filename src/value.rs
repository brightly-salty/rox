use crate::tokens::Literal;
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
            Self::String_(s) => write!(f, "{}", s),
            Self::Nil => write!(f, "nil"),
            Self::Number(n) => {
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
            Self::Bool(b) => write!(f, "{}", b),
        }
    }
}

impl From<Literal> for Value {
    fn from(l: Literal) -> Self {
        match l {
            Literal::String_(s) => Self::String_(s),
            Literal::Bool(b) => Self::Bool(b),
            Literal::Number(n) => Self::Number(n),
            Literal::Nil => Self::Nil,
        }
    }
}
