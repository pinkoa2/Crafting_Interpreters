use super::expression::Expression;
use super::{binary::Binary, literal::Literal, unary::Unary};

pub trait Visitor {
    fn visit_binary(&self, element: &Binary) -> String;
    fn visit_literal(&self, element: &Literal) -> String;
    fn visit_unary(&self, element: &Unary) -> String;
}

pub struct Printer {}

impl Visitor for Printer {
    fn visit_binary(&self, element: &super::binary::Binary) -> String {
        let name = element.operator.lexem.clone();
        let left = element.left;
        let right = element.right;
        self.parenthesis(&name, &vec![left, right])
    }
    fn visit_literal(&self, element: &super::literal::Literal) -> String {
        return element.value.clone();
    }
    fn visit_unary(&self, element: &super::unary::Unary) -> String {
        let name = element.operator.lexem.clone();
        let right = element.right;
        self.parenthesis(&name, &vec![right])
    }
}

#[allow(dead_code)]
impl Printer {
    pub fn convert(&self, exp: &dyn Expression) -> String {
        return exp.accept(self);
    }

    fn parenthesis(&self, name: &String, expressions: &Vec<&dyn Expression>) -> String {
        let mut expression = format!("({}", name);
        for exp in expressions {
            expression = expression + " " + exp.accept(self).trim();
        }
        return expression + ")";
    }
}

#[cfg(test)]
mod tests {
    use super::Printer;
    const PRINTER: Printer = Printer {};
    use crate::expressions::unary::Unary;
    use crate::token::token_type::Token_Type;
    use crate::{
        expressions::{binary::Binary, literal::Literal},
        token::token::Token,
    };

    #[test]
    fn test_printer_literal() {
        let literal = Literal::new("Hi".to_string());
        assert_eq!(PRINTER.convert(&literal), "Hi")
    }

    #[test]
    fn test_printer_binary() {
        let left_literal = Literal::new("10".to_string());
        let right_literal = Literal::new("20".to_string());
        let operator = Token::new(Token_Type::PLUS, "+".to_string(), "".to_string(), 1);
        let binary = Binary::new(&left_literal, operator, &right_literal);
        assert_eq!(PRINTER.convert(&binary), "(+ 10 20)");
    }

    #[test]
    fn test_printer_inner_binary() {
        let inner_left_literal = Literal::new("10".to_string());
        let inner_right_literal = Literal::new("20".to_string());
        let inner_operator = Token::new(Token_Type::PLUS, "+".to_string(), "".to_string(), 1);
        let inner_binary = Binary::new(&inner_left_literal, inner_operator, &inner_right_literal);
        let left_literal = Literal::new("5".to_string());
        let outer_operator = Token::new(Token_Type::STAR, "*".to_string(), "".to_string(), 1);
        let binary = Binary::new(&left_literal, outer_operator, &inner_binary);
        assert_eq!(PRINTER.convert(&binary), "(* 5 (+ 10 20))");
    }

    #[test]
    fn test_printer_unary() {
        let operator = Token::new(Token_Type::MINUS, "-".to_string(), "".to_string(), 1);
        let right_literal = Literal::new("5".to_string());
        let unary = Unary::new(operator, &right_literal);
        assert_eq!(PRINTER.convert(&unary), "(- 5)");
    }

    #[test]
    fn test_printer_inner_unary() {
        let inner_operator = Token::new(Token_Type::MINUS, "-".to_string(), "".to_string(), 1);
        let inner_right_literal = Literal::new("10".to_string());
        let inner_unary = Unary::new(inner_operator, &inner_right_literal);
        let operator = Token::new(Token_Type::MINUS, "-".to_string(), "".to_string(), 1);
        let unary = Unary::new(operator, &inner_unary);
        assert_eq!(PRINTER.convert(&unary), "(- (- 10))");
    }
}
