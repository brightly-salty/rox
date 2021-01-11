use crate::tokens::{Literal, Token};

#[derive(PartialEq, Clone, Debug)]
pub enum Stmt {
    Block(Vec<Stmt>),
    Expression(Expr),
    Print(Expr),
    Var(Token, Option<Expr>),
}

pub trait StmtVisitor<T> {
    fn visit_block_stmt(&mut self, stmts: Vec<Stmt>) -> T;
    fn visit_expression_stmt(&mut self, stmt: Expr) -> T;
    fn visit_print_stmt(&mut self, stmt: Expr) -> T;
    fn visit_var_stmt(&mut self, name: Token, initializer: Option<Expr>) -> T;
}

#[derive(PartialEq, Clone, Debug)]
pub enum Expr {
    Assign(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Literal),
    Unary(Token, Box<Expr>),
    Variable(Token),
}

pub trait ExprVisitor<T> {
    fn visit_assign_expr(&mut self, name: Token, value: Box<Expr>) -> T;
    fn visit_binary_expr(&mut self, left: Box<Expr>, operator: Token, right: Box<Expr>) -> T;
    fn visit_grouping_expr(&mut self, expr: Box<Expr>) -> T;
    fn visit_literal_expr(&self, literal: Literal) -> T;
    fn visit_unary_expr(&mut self, operator: Token, right: Box<Expr>) -> T;
    fn visit_variable_expr(&self, expr: Token) -> T;
}
