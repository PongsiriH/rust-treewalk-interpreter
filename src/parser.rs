use crate::{
    expression::Expr,
    token::{Literal, Token, TokenType},
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        return Parser { tokens, current: 0 };
    }

    pub fn parse(&mut self) -> Box<Expr> {
        return self.expression();
    }

    fn is_next(&self, expected: &[TokenType]) -> bool {
        for &token in expected {
            if self.tokens[self.current].token_type == token {
                return true;
            }
        }
        return false;
    }

    fn expression(&mut self) -> Box<Expr> {
        return self.equality();
    }

    fn equality(&mut self) -> Box<Expr> {
        let expr = self.comparison();
        if self.is_next(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let op = self.tokens[self.current].clone();
            self.current += 1;
            let right = self.comparison();
            let expr = Box::new(Expr::Binary(expr, op, right));
            return expr;
        }
        return expr;
    }

    fn comparison(&mut self) -> Box<Expr> {
        let expr = self.term();
        if self.is_next(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let op = self.tokens[self.current].clone();
            self.current += 1;
            let right = self.term();
            let expr = Box::new(Expr::Binary(expr, op, right));
            return expr;
        }
        return expr;
    }

    fn term(&mut self) -> Box<Expr> {
        let expr = self.factor();
        if self.is_next(&[TokenType::Minus, TokenType::Plus]) {
            let op = self.tokens[self.current].clone();
            self.current += 1;
            let right = self.factor();
            let expr = Box::new(Expr::Binary(expr, op, right));
            return expr;
        }
        return expr;
    }

    fn factor(&mut self) -> Box<Expr> {
        let expr = self.unary();
        if self.is_next(&[TokenType::Slash, TokenType::Star]) {
            let op = self.tokens[self.current].clone();
            self.current += 1;
            let right = self.unary();
            let expr = Box::new(Expr::Binary(expr, op, right));
            return expr;
        }
        return expr;
    }

    fn unary(&mut self) -> Box<Expr> {
        if self.is_next(&[TokenType::Bang, TokenType::Minus]) {
            let op = self.tokens[self.current].clone();
            self.current += 1;
            let right = self.unary();
            return Box::new(Expr::Unary(op, right));
        }
        return self.primary();
    }

    fn primary(&mut self) -> Box<Expr> {
        match self.tokens[self.current].token_type {
            TokenType::False => {
                self.current += 1;
                return Box::new(Expr::Literal(Literal::Boolean(false)));
            }
            TokenType::True => {
                self.current += 1;
                return Box::new(Expr::Literal(Literal::Boolean(true)));
            }
            TokenType::Nil => {
                self.current += 1;
                return Box::new(Expr::Literal(Literal::Nil));
            }
            TokenType::Number => {
                self.current += 1;
                return Box::new(Expr::Literal(
                    self.tokens[self.current-1].literal.clone().unwrap(),
                ));
            }
            TokenType::StringLiteral => {
                self.current += 1;
                return Box::new(Expr::Literal(
                    self.tokens[self.current-1].literal.clone().unwrap(),
                ));
            }
            TokenType::LeftParen => {
                self.current += 1;
                let expr = self.expression();
                if self.tokens[self.current].token_type != TokenType::RightParen {
                    println!("Parser.primary(), expected closing paren )");
                }
                self.current += 1;
                return Box::new(Expr::Grouping(expr));
            }
            _ => {
                panic!("Error in primary: Have yet to support this");
            }
        }
    }
}
