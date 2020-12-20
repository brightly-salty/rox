use crate::expr::{Binary, Expr, Grouping, Literal as LiteralExpr, Unary};
use crate::tokens::TokenType::*;
use crate::tokens::{Literal, Token, TokenType};
use anyhow::{anyhow, Result};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Option<Expr> {
        self.expression().ok()
    }

    fn expression(&mut self) -> Result<Expr> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr> {
        let mut expr = self.comparison()?;
        while self.matches(vec![BangEqual, EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Expr::Binary(Binary::new(Box::new(expr), operator, Box::new(right)));
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr> {
        let mut expr = self.term()?;
        while self.matches(vec![Greater, GreaterEqual, Less, LessEqual]) {
            let operator = self.previous();
            let right = self.term()?;
            expr = Expr::Binary(Binary::new(Box::new(expr), operator, Box::new(right)));
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr> {
        let mut expr = self.factor()?;
        while self.matches(vec![Plus, Minus]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::Binary(Binary::new(Box::new(expr), operator, Box::new(right)));
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr> {
        let mut expr = self.unary()?;
        while self.matches(vec![Slash, Star]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Expr::Binary(Binary::new(Box::new(expr), operator, Box::new(right)));
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr> {
        if self.matches(vec![Bang, Minus]) {
            let operator = self.previous();
            let right = self.unary()?;
            Ok(Expr::Unary(Unary::new(operator, Box::new(right))))
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expr> {
        if self.matches(vec![False]) {
            return Ok(Expr::Literal(LiteralExpr::new(Literal::Bool(false))));
        }
        if self.matches(vec![True]) {
            return Ok(Expr::Literal(LiteralExpr::new(Literal::Bool(true))));
        }
        if self.matches(vec![Nil]) {
            return Ok(Expr::Literal(LiteralExpr::new(Literal::Nil)));
        }
        if self.matches(vec![Number, String_]) {
            return Ok(Expr::Literal(LiteralExpr::new(
                match self.previous().literal {
                    Some(l) => l,
                    None => Literal::Nil,
                },
            )));
        }
        if self.matches(vec![LeftParen]) {
            let expr = self.expression()?;
            self.consume(&RightParen, "Expect `)` after expression")?;
            return Ok(Expr::Grouping(Grouping::new(Box::new(expr))));
        }
        crate::error_at_token(self.peek(), "Expect expression");
        return Err(anyhow!("Parse error"));
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

    fn matches(&mut self, types: Vec<TokenType>) -> bool {
        for type_ in &types {
            if self.check(type_) {
                self.advance();
                return true;
            }
        }
        return false;
    }

    fn consume(&mut self, type_: &TokenType, message: &str) -> Result<()> {
        if self.check(type_) {
            self.advance();
            Ok(())
        } else {
            crate::error_at_token(self.peek(), message);
            return Err(anyhow!("Parse error"));
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
