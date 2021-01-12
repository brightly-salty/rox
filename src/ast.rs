use crate::tokens::{Literal, Token};

#[derive(PartialEq, Clone, Debug)]
pub enum Stmt {
    Block(Vec<Stmt>),
    Expression(Expr),
    If(Expr, Box<Stmt>, Box<Option<Stmt>>),
    Print(Expr),
    Var(Token, Option<Expr>),
    While(Expr, Box<Stmt>)
}

pub trait StmtVisitor<T> {
    fn execute(&mut self, stmt: Stmt) -> T {
        match stmt {
            Stmt::If(condition, then_branch, else_branch) => {
                self.visit_if_stmt(condition, then_branch, else_branch)
            }
            Stmt::Block(stmts) => self.visit_block_stmt(stmts),
            Stmt::Expression(stmt) => self.visit_expression_stmt(stmt),
            Stmt::Print(stmt) => self.visit_print_stmt(stmt),
            Stmt::Var(name, initializer) => self.visit_var_stmt(name, initializer),
            Stmt::While(condition, body) => self.visit_while_stmt(condition, body),
        }
    }
    fn visit_if_stmt(
        &mut self,
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Box<Option<Stmt>>,
    ) -> T;
    fn visit_block_stmt(&mut self, stmts: Vec<Stmt>) -> T;
    fn visit_expression_stmt(&mut self, stmt: Expr) -> T;
    fn visit_print_stmt(&mut self, stmt: Expr) -> T;
    fn visit_var_stmt(&mut self, name: Token, initializer: Option<Expr>) -> T;
    fn visit_while_stmt(&mut self, condition: Expr, body: Box<Stmt>) -> T;
}

#[derive(PartialEq, Clone, Debug)]
pub enum Expr {
    Assign(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Literal),
    Logical(Box<Expr>, Token, Box<Expr>),
    Unary(Token, Box<Expr>),
    Variable(Token),
}

pub trait ExprVisitor<T> {
    fn evaluate(&mut self, expr: Expr) -> T {
        match expr {
            Expr::Assign(name, value) => self.visit_assign_expr(name, value),
            Expr::Binary(b, o, b2) => self.visit_binary_expr(b, o, b2),
            Expr::Grouping(g) => self.visit_grouping_expr(g),
            Expr::Literal(l) => self.visit_literal_expr(l),
            Expr::Unary(operator, right) => self.visit_unary_expr(operator, right),
            Expr::Variable(v) => self.visit_variable_expr(v),
            Expr::Logical(left, operator, right) => self.visit_logical_expr(left, operator, right),
        }
    }
    fn visit_assign_expr(&mut self, name: Token, value: Box<Expr>) -> T;
    fn visit_binary_expr(&mut self, left: Box<Expr>, operator: Token, right: Box<Expr>) -> T;
    fn visit_grouping_expr(&mut self, expr: Box<Expr>) -> T;
    fn visit_literal_expr(&mut self, literal: Literal) -> T;
    fn visit_logical_expr(&mut self, left: Box<Expr>, operator: Token, right: Box<Expr>) -> T;
    fn visit_unary_expr(&mut self, operator: Token, right: Box<Expr>) -> T;
    fn visit_variable_expr(&self, expr: Token) -> T;
}
