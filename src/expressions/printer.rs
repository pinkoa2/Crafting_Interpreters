use super::{expression::Expression, visitor::Visitor};

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
    static PRINTER: Printer = Printer {};
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
}
