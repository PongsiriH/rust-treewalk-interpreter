use crate::token::{Literal, Token};

#[derive(Debug)]
pub enum Expr {
    Literal(Literal),
    Grouping(Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Unary(Token, Box<Expr>),
}

pub trait Visitor<R> {
    fn visit_literal(&self, literal: &Literal) -> R;
    fn visit_grouping(&self, expr: &Box<Expr>) -> R;
    fn visit_binary(&self, left: &Box<Expr>, operator: &Token, right: &Box<Expr>) -> R;
    fn visit_unary(&self, operator: &Token, right: &Box<Expr>) -> R;
}

impl Expr {
    pub fn accept<T: Visitor<R>, R>(&self, visitor: &T) -> R {
        match self {
            Expr::Literal(literal) => visitor.visit_literal(literal),
            Expr::Grouping(expr) => visitor.visit_grouping(expr),
            Expr::Binary(left, op, right) => visitor.visit_binary(left, op, right),
            Expr::Unary(op, right) => visitor.visit_unary(op, right),
        }
    }

    // fn string(&self) -> String {
    //     match self {
    //         Expr::Literal(literal) => format!("{literal}"),
    //         Expr::Grouping(expression) => format!("({})", expression.string()),
    //         Expr::Binary(left, op, right) => {
    //             format!("{} binaryOp {}", left.string(), right.string())
    //         }
    //         Expr::Unary(op, right) => format!("unaryOp {}", right.string()),
    //     }
    // }
}

