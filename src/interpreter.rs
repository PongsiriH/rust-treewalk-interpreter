use crate::{
    expression::{Expr, Visitor},
    token::{Literal, Token, TokenType},
};

fn is_truthy(literal: &Literal) -> bool {
    *literal != Literal::Nil && *literal != Literal::Boolean(false)
}

fn is_eqaul(a: &Literal, b: &Literal) -> bool {
    a == b
}

pub struct Interpreter {}

impl Interpreter {
    pub fn interpret(&self, expr: &Box<Expr>) -> Literal {
        expr.accept(self)
    }
}

impl Visitor<Literal> for Interpreter {
    fn visit_literal(&self, literal: &Literal) -> Literal {
        literal.clone()
    }

    fn visit_grouping(&self, expr: &Box<Expr>) -> Literal {
        expr.accept(self)
    }

    fn visit_unary(&self, operator: &Token, right: &Box<Expr>) -> Literal {
        let right = right.accept(self);
        match operator.token_type {
            TokenType::Minus => {
                if let Literal::Number(value) = right {
                    return Literal::Number(-value);
                }
                // panic!("Unary minus can only be applied to numbers");
            }
            TokenType::Bang => return Literal::Boolean(!is_truthy(&right)),
            _ => panic!("Interpreter does not support this"),
        }
        Literal::Nil
    }

    fn visit_binary(&self, left: &Box<Expr>, operator: &Token, right: &Box<Expr>) -> Literal {
        let left = left.accept(self);
        let right = right.accept(self);

        match operator.token_type {
            TokenType::Minus => {
                if let (Literal::Number(value_left), Literal::Number(value_right)) = (&left,&right) {
                    return Literal::Number(value_left - value_right);
                }
            }
            TokenType::Star => {
                if let (Literal::Number(value_left), Literal::Number(value_right)) = (&left,&right) {
                    return Literal::Number(value_left * value_right);
                }
            }
            TokenType::Slash => {
                if let (Literal::Number(value_left), Literal::Number(value_right)) = (&left,&right) {
                    return Literal::Number(value_left / value_right);
                }
            }
            TokenType::Plus => {
                if let (Literal::Number(value_left), Literal::Number(value_right)) = (&left, &right)
                {
                    return Literal::Number(value_left + value_right);
                }
                if let (Literal::Str(value_left), Literal::Str(value_right)) = (&left, &right) {
                    let mut result = value_left.clone(); // Clone to avoid moving
                    result.push_str(value_right);
                    return Literal::Str(result);
                }
                // panic!("Operands must be two numbers or two strings for `+` operator.");
            }
            TokenType::Greater => {
                if let (Literal::Number(value_left), Literal::Number(value_right)) = (&left,&right) {
                    return Literal::Boolean(value_left > value_right);
                }
            }
            TokenType::GreaterEqual => {
                if let (Literal::Number(value_left), Literal::Number(value_right)) = (&left,&right) {
                    return Literal::Boolean(value_left >= value_right);
                }
            }
            TokenType::Less => {
                if let (Literal::Number(value_left), Literal::Number(value_right)) = (&left,&right) {
                    return Literal::Boolean(value_left < value_right);
                }
            }
            TokenType::LessEqual => {
                if let (Literal::Number(value_left), Literal::Number(value_right)) = (&left,&right) {
                    return Literal::Boolean(value_left <= value_right);
                }
            }
            TokenType::BangEqual => {
                return Literal::Boolean(!is_eqaul(&left, &right));
            }
            TokenType::EqualEqual => {
                return Literal::Boolean(is_eqaul(&left, &right));
            }
            _ => {
                panic!("Interpreter does not support this");
            }
        }
        Literal::Nil
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expression::Expr;
    use crate::token::{Literal, Token, TokenType};

    #[test]
    fn test_literal() {
        let interpreter = Interpreter {};
        let literal = Literal::Number(42.0);
        assert_eq!(interpreter.visit_literal(&literal), Literal::Number(42.0));
    }

    #[test]
    fn test_unary_minus() {
        let interpreter = Interpreter {};
        let expr = Expr::Unary(
            Token {
                token_type: TokenType::Minus,
                lexeme: "-".to_string(),
                literal: None,
                line: 1,
            },
            Box::new(Expr::Literal(Literal::Number(10.0))),
        );
        assert_eq!(expr.accept(&interpreter), Literal::Number(-10.0));
    }

    #[test]
    fn test_unary_not() {
        let interpreter = Interpreter {};
        let expr = Expr::Unary(
            Token {
                token_type: TokenType::Bang,
                lexeme: "!".to_string(),
                literal: None,
                line: 1,
            },
            Box::new(Expr::Literal(Literal::Boolean(true))),
        );
        assert_eq!(expr.accept(&interpreter), Literal::Boolean(false));
    }

    #[test]
    fn test_binary_addition() {
        let interpreter = Interpreter {};
        let expr = Expr::Binary(
            Box::new(Expr::Literal(Literal::Number(5.0))),
            Token {
                token_type: TokenType::Plus,
                lexeme: "+".to_string(),
                literal: None,
                line: 1,
            },
            Box::new(Expr::Literal(Literal::Number(3.0))),
        );
        assert_eq!(expr.accept(&interpreter), Literal::Number(8.0));
    }

    #[test]
    fn test_binary_multiplication() {
        let interpreter = Interpreter {};
        let expr = Expr::Binary(
            Box::new(Expr::Literal(Literal::Number(4.0))),
            Token {
                token_type: TokenType::Star,
                lexeme: "*".to_string(),
                literal: None,
                line: 1,
            },
            Box::new(Expr::Literal(Literal::Number(2.0))),
        );
        assert_eq!(expr.accept(&interpreter), Literal::Number(8.0));
    }
}

