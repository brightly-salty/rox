use crate::tokens::Token;

#[derive(PartialEq, Clone, Debug)]
pub enum Expr {
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
    Unary(Unary),
}

pub trait Visitor<T> {
    fn visit_binary_expr(expr: Binary) -> T;
    fn visit_grouping_expr(expr: Grouping) -> T;
    fn visit_literal_expr(expr: Literal) -> T;
    fn visit_unary_expr(expr: Unary) -> T;
}

#[derive(PartialEq, Clone, Debug)]
pub struct Binary {
    left: Box<Expr>,
    operator: Token,
    right: Box<Expr>,
}

impl Binary {
    pub fn new(left: Box<Expr>, operator: Token, right: Box<Expr>) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Grouping {
    expression: Box<Expr>,
}

impl Grouping {
    pub fn new(expression: Box<Expr>) -> Self {
        Self { expression }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Literal {
    value: crate::tokens::Literal,
}

impl Literal {
    pub fn new(value: crate::tokens::Literal) -> Self {
        Self { value }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Unary {
    operator: Token,
    right: Box<Expr>,
}

impl Unary {
    pub fn new(operator: Token, right: Box<Expr>) -> Self {
        Self { operator, right }
    }
}
