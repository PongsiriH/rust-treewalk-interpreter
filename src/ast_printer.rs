use crate::{
    expression::{Expr, Visitor},
    token::{Literal, Token},
};

pub struct AstPrinter {}
impl AstPrinter {
    pub fn print(&self, expr: &Box<Expr>) -> String {
        expr.accept(self)
    }
    fn parenthesize(&self, name: &str, exprs: &[&Box<Expr>]) -> String {
        let mut builder = String::new();
        builder.push('(');
        builder.push_str(name);
        for expr in exprs {
            builder.push(' ');
            builder.push_str(&expr.accept(self));
        }
        builder.push(')');
        return builder;
    }
}

impl Visitor<String> for AstPrinter {
    fn visit_literal(&self, expr: &Literal) -> String {
        expr.to_string()
    }
    fn visit_grouping(&self, expr: &Box<Expr>) -> String {
        self.parenthesize("grouping", &[expr])
    }
    fn visit_binary(&self, left: &Box<Expr>, operator: &Token, right: &Box<Expr>) -> String {
        self.parenthesize(&operator.lexeme, &[left, right])
    }
    fn visit_unary(&self, operator: &Token, right: &Box<Expr>) -> String {
        self.parenthesize(&operator.lexeme, &[right])
    }
}

#[cfg(test)]
mod tests {
    use crate::token::TokenType;

    use super::*;

    #[test]
    fn test_binary_op() {
        let ast_printer = AstPrinter {};
        let text = ast_printer.visit_binary(
            &Box::new(Expr::Literal(Literal::Number(5.0))),
            &Token::new(TokenType::Plus, String::from("+"), None, 1),
            &Box::new(Expr::Literal(Literal::Number(2.0))),
        );
        assert_eq!(text, "(+ 5 2)");

        let text = ast_printer.visit_binary(
            &Box::new(Expr::Literal(Literal::Number(5.2))),
            &Token::new(TokenType::Plus, String::from("+"), None, 1),
            &Box::new(Expr::Literal(Literal::Number(2.5))),
        );
        assert_eq!(text, "(+ 5.2 2.5)");

        let text = ast_printer.visit_binary(
            &Box::new(Expr::Literal(Literal::Number(5.2))),
            &Token::new(TokenType::Minus, String::from("-"), None, 1),
            &Box::new(Expr::Literal(Literal::Number(2.5))),
        );
        assert_eq!(text, "(- 5.2 2.5)");
    }

    #[test]
    fn test_complex_op() {
        let ast_printer = AstPrinter {};

        // Create a complex expression: (+ (* 5 2) (- 10 3))
        let multiply = Box::new(Expr::Binary(
            Box::new(Expr::Literal(Literal::Number(5.0))),
            Token::new(TokenType::Star, String::from("*"), None, 1),
            Box::new(Expr::Literal(Literal::Number(2.0))),
        ));

        let subtract = Box::new(Expr::Binary(
            Box::new(Expr::Literal(Literal::Number(10.0))),
            Token::new(TokenType::Minus, String::from("-"), None, 1),
            Box::new(Expr::Literal(Literal::Number(3.0))),
        ));

        let text = ast_printer.visit_binary(
            &multiply,
            &Token::new(TokenType::Plus, String::from("+"), None, 1),
            &subtract,
        );

        assert_eq!(text, "(+ (* 5 2) (- 10 3))");

        // Test with a unary operation inside a binary operation
        // Create: (* (- 5) 3)
        let negate = Box::new(Expr::Unary(
            Token::new(TokenType::Minus, String::from("-"), None, 1),
            Box::new(Expr::Literal(Literal::Number(5.0))),
        ));

        let text = ast_printer.visit_binary(
            &negate,
            &Token::new(TokenType::Star, String::from("*"), None, 1),
            &Box::new(Expr::Literal(Literal::Number(3.0))),
        );

        assert_eq!(text, "(* (- 5) 3)");
    }

    #[test]
    fn more_complex_grouping() {
        let ast_printer = AstPrinter {};

        // Test a grouped expression: (group (+ 2 (* 3 4)))
        let multiply = Box::new(Expr::Binary(
            Box::new(Expr::Literal(Literal::Number(3.0))),
            Token::new(TokenType::Star, String::from("*"), None, 1),
            Box::new(Expr::Literal(Literal::Number(4.0))),
        ));

        let addition = Box::new(Expr::Binary(
            Box::new(Expr::Literal(Literal::Number(2.0))),
            Token::new(TokenType::Plus, String::from("+"), None, 1),
            multiply,
        ));

        let grouped = Box::new(Expr::Grouping(addition));

        let text = ast_printer.print(&grouped);
        assert_eq!(text, "(grouping (+ 2 (* 3 4)))");

        // Test nested groupings: (group (group 42))
        let inner_group = Box::new(Expr::Grouping(Box::new(Expr::Literal(Literal::Number(
            42.0,
        )))));

        let outer_group = Box::new(Expr::Grouping(inner_group));

        let text = ast_printer.print(&outer_group);
        assert_eq!(text, "(grouping (grouping 42))");
    }
}
