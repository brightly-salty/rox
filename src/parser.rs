use crate::ast::{Expr, Stmt};
use crate::error;
use crate::tokens::TokenType::{
    Bang, BangEqual, Class, Eof, Equal, EqualEqual, False, For, Fun, Greater, GreaterEqual,
    Identifier, If, LeftBrace, LeftParen, Less, LessEqual, Minus, Nil, Number, Plus, Print, Return,
    RightBrace, RightParen, Semicolon, Slash, Star, String_, True, Var, While,
};
use crate::tokens::{Literal, Token, TokenType};
use anyhow::Result;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();
        loop {
            if let Some(stmt) = self.declaration() {
                statements.push(stmt);
            }
            if self.is_at_end() {
                break;
            }
        }
        statements
    }

    fn declaration(&mut self) -> Option<Stmt> {
        if self.matches(&[Var]) {
            if let Ok(stmt) = self.var_declaration() {
                Some(stmt)
            } else {
                self.synchronize();
                None
            }
        } else if let Ok(stmt) = self.statement() {
            Some(stmt)
        } else {
            self.synchronize();
            None
        }
    }

    fn var_declaration(&mut self) -> Result<Stmt> {
        let name = self.consume(&Identifier, "Expect variable name.")?;
        let initializer = if self.matches(&[Equal]) {
            self.expression().ok()
        } else {
            None
        };
        self.consume(&Semicolon, "Expect ';' after variable declaration.")?;
        Ok(Stmt::Var(name, initializer))
    }

    fn statement(&mut self) -> Result<Stmt> {
        if self.matches(&[Print]) {
            self.print_statement()
        } else if self.matches(&[LeftBrace]) {
            Ok(Stmt::Block(self.block()?))
        } else {
            self.expression_statement()
        }
    }

    fn block(&mut self) -> Result<Vec<Stmt>> {
        let mut statements = Vec::new();
        loop {
            if let Some(stmt) = self.declaration() {
                statements.push(stmt);
            }
            if self.check(&RightBrace) || self.is_at_end() {
                break;
            }
        }
        self.consume(&RightBrace, "Expect ';' after block.")?;
        Ok(statements)
    }

    fn print_statement(&mut self) -> Result<Stmt> {
        let value = self.expression()?;
        self.consume(&Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Print(value))
    }

    fn expression_statement(&mut self) -> Result<Stmt> {
        let expr = self.expression()?;
        self.consume(&Semicolon, "Expect ';' after expression.")?;
        Ok(Stmt::Expression(expr))
    }

    fn expression(&mut self) -> Result<Expr> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr> {
        let expr = self.equality()?;
        if self.matches(&[Equal]) {
            let equals = self.previous();
            let value = self.assignment()?;
            if let Expr::Variable(name) = expr {
                Ok(Expr::Assign(name, Box::new(value)))
            } else {
                error(equals.line, "Invalid assignment taarget.");
                Ok(expr)
            }
        } else {
            Ok(expr)
        }
    }

    fn equality(&mut self) -> Result<Expr> {
        let mut expr = self.comparison()?;
        while self.matches(&[BangEqual, EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr> {
        let mut expr = self.term()?;
        while self.matches(&[Greater, GreaterEqual, Less, LessEqual]) {
            let operator = self.previous();
            let right = self.term()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr> {
        let mut expr = self.factor()?;
        while self.matches(&[Plus, Minus]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr> {
        let mut expr = self.unary()?;
        while self.matches(&[Slash, Star]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr> {
        if self.matches(&[Bang, Minus]) {
            let operator = self.previous();
            let right = self.unary()?;
            Ok(Expr::Unary(operator, Box::new(right)))
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expr> {
        if self.matches(&[False]) {
            return Ok(Expr::Literal(Literal::Bool(false)));
        }
        if self.matches(&[True]) {
            return Ok(Expr::Literal(Literal::Bool(true)));
        }
        if self.matches(&[Nil]) {
            return Ok(Expr::Literal(Literal::Nil));
        }
        if self.matches(&[Number, String_]) {
            return Ok(Expr::Literal(match self.previous().literal {
                Some(l) => l,
                None => Literal::Nil,
            }));
        }
        if self.matches(&[Identifier]) {
            return Ok(Expr::Variable(self.previous()));
        }
        if self.matches(&[LeftParen]) {
            let expr = self.expression()?;
            self.consume(&RightParen, "Expect `)` after expression")?;
            return Ok(Expr::Grouping(Box::new(expr)));
        }
        crate::error_at_token(&self.peek(), "Expect expression");
        Err(anyhow!("Parse error"))
    }

    fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if self.previous().type_ == Semicolon {
                return;
            }
            match self.peek().type_ {
                Class | Fun | Var | For | If | While | Print | Return => {
                    return;
                }
                _ => {}
            }
            self.advance();
        }
    }

    fn matches(&mut self, types: &[TokenType]) -> bool {
        for type_ in types {
            if self.check(type_) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn consume(&mut self, type_: &TokenType, message: &str) -> Result<Token> {
        if self.check(type_) {
            Ok(self.advance())
        } else {
            crate::error_at_token(&self.peek(), message);
            Err(anyhow!("Parse error"))
        }
    }

    fn check(&self, type_: &TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            &self.peek().type_ == type_
        }
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().type_ == Eof
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }
}
