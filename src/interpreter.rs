use crate::ast::{Expr, ExprVisitor, Stmt, StmtVisitor};
use crate::environment::Environment;
use crate::tokens::TokenType::{
    self, Bang, BangEqual, EqualEqual, Greater, GreaterEqual, Less, LessEqual, Minus, Plus, Slash,
    Star,
};
use crate::tokens::{Literal, Token};
use crate::value::Value;

#[derive(Debug, PartialEq, Clone)]
pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::default(),
        }
    }

    pub fn interpret(&mut self, statements: &[Stmt]) {
        for statement in statements {
            self.execute(statement.clone());
        }
    }

    fn execute_block(&mut self, statements: &[Stmt], environment: Environment) {
        let previous = self.environment.clone();
        self.environment = environment;
        for statement in statements {
            self.execute(statement.clone());
        }
        self.environment = previous;
    }

    const fn is_truthy(value: &Value) -> bool {
        if let Value::Bool(b) = value {
            *b
        } else {
            !matches!(value, Value::Nil)
        }
    }

    fn is_equal(a: Value, b: Value) -> bool {
        match (a, b) {
            (Value::Nil, Value::Nil) => true,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::String_(a), Value::String_(b)) => a == b,
            (Value::Number(a), Value::Number(b)) => (a - b).abs() < std::f64::EPSILON,
            _ => false,
        }
    }
}

impl StmtVisitor<()> for Interpreter {
    fn visit_while_stmt(&mut self, condition: Expr, body: Box<Stmt>) {
        while Self::is_truthy(&self.evaluate(condition.clone())) {
            self.execute(*body.clone());
        }
    }
    fn visit_if_stmt(
        &mut self,
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Box<Option<Stmt>>,
    ) {
        if Self::is_truthy(&self.evaluate(condition)) {
            self.execute(*then_branch);
        } else if let Some(else_branch) = *else_branch {
            self.execute(else_branch);
        }
    }
    fn visit_block_stmt(&mut self, statements: Vec<Stmt>) {
        self.execute_block(&statements, Environment::new_from(self.environment.clone()));
    }

    fn visit_expression_stmt(&mut self, stmt: Expr) {
        self.evaluate(stmt);
    }

    fn visit_print_stmt(&mut self, stmt: Expr) {
        let value = self.evaluate(stmt);
        println!("{}", value);
    }

    fn visit_var_stmt(&mut self, name: Token, initializer: Option<Expr>) {
        let value =
            initializer.map_or_else(|| Value::Nil, |initializer| self.evaluate(initializer));
        self.environment.define(name.lexeme, value);
    }
}

impl ExprVisitor<Value> for Interpreter {
    fn visit_logical_expr(&mut self, left: Box<Expr>, operator: Token, right: Box<Expr>) -> Value {
        let left = self.evaluate(*left);
        if operator.type_ == TokenType::Or {
            if Self::is_truthy(&left) {
                return left;
            }
        } else if !Self::is_truthy(&left) {
            return left;
        }
        self.evaluate(*right)
    }
    fn visit_assign_expr(&mut self, name: Token, value: Box<Expr>) -> Value {
        let value = self.evaluate(*value);
        self.environment.assign(name, value.clone()).unwrap();
        value
    }

    fn visit_binary_expr(&mut self, left: Box<Expr>, operator: Token, right: Box<Expr>) -> Value {
        let left = self.evaluate(*left);
        let right = self.evaluate(*right);
        match operator.type_ {
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
            BangEqual => Value::Bool(!Self::is_equal(left, right)),
            EqualEqual => Value::Bool(Self::is_equal(left, right)),
            _ => Value::Nil,
        }
    }
    fn visit_grouping_expr(&mut self, expression: Box<Expr>) -> Value {
        self.evaluate(*expression)
    }
    fn visit_literal_expr(&mut self, value: Literal) -> Value {
        value.into()
    }
    fn visit_unary_expr(&mut self, operator: Token, right: Box<Expr>) -> Value {
        let right = self.evaluate(*right);
        match operator.type_ {
            Minus => {
                if let Value::Number(n) = right {
                    Value::Number(-n)
                } else {
                    panic!("{:?} must be a number", right);
                }
            }
            Bang => Value::Bool(!Self::is_truthy(&right)),
            _ => Value::Nil,
        }
    }

    fn visit_variable_expr(&self, name: Token) -> Value {
        self.environment.get(&name).unwrap()
    }
}
