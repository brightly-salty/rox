use crate::tokens::Token;

#[derive(PartialEq, Clone, Debug)]
pub enum Expr {
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
    Unary(Unary),
}

pub trait Visitor<T> {
    fn visit_binary_expr(&self, expr: Binary) -> T;
    fn visit_grouping_expr(&self, expr: Grouping) -> T;
    fn visit_literal_expr(&self, expr: Literal) -> T;
    fn visit_unary_expr(&self, expr: Unary) -> T;
}

#[derive(PartialEq, Clone, Debug)]
pub struct Binary {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
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
    pub expression: Box<Expr>,
}

impl Grouping {
    pub fn new(expression: Box<Expr>) -> Self {
        Self { expression }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Literal {
    pub value: crate::tokens::Literal,
}

impl Literal {
    pub fn new(value: crate::tokens::Literal) -> Self {
        Self { value }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Unary {
    pub operator: Token,
    pub right: Box<Expr>,
}

impl Unary {
    pub fn new(operator: Token, right: Box<Expr>) -> Self {
        Self { operator, right }
    }
}
