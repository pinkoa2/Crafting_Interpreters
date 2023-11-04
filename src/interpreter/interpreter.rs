use std::any::Any;

use crate::{
    expressions::{expression::Expression, literal::{Literal, LiteralEnum}, visitor::Visitor},
    token::{token_type::Token_Type, token::Token}, lox::Lox,
};

pub struct Interpreter {}

impl Visitor<Result<Literal, String>> for Interpreter {
    fn visit_literal(&self, element: &crate::expressions::literal::Literal) -> Result<Literal, String> {
        Ok(element.clone())
    }

    fn visit_grouping(&self, element: &crate::expressions::grouping::Grouping) -> Result<Literal, String> {
        self.evaluate(&element.exp)
    }

    fn visit_unary(&self, element: &crate::expressions::unary::Unary) -> Result<Literal, String> {
        let operator: &Token_Type = &element.operator.token_type;
        let right = &element.right;
        let expression: Literal = match self.evaluate(right) {
            Ok(literal) => literal,
            Err(m) => return Err(m)
        };

        match operator {
            Token_Type::BANG => {
                let new_operand = !self.is_truthy(&expression);
                return Ok(Literal::new(Box::new(new_operand), LiteralEnum::BOOLEAN));
            }
            Token_Type::MINUS => {
                // Check that it's a number
                if expression.literal_type != LiteralEnum::NUMBER {
                    return Err(self.runtime_error(&element.operator, "Operand must be a number."))
                }
                let value = expression.value.downcast_ref::<f64>().unwrap().clone() * -1.0;
                return Ok(Literal::new(Box::new(value), LiteralEnum::NUMBER))
            }
            _ => Err(self.runtime_error(&element.operator, "Invalid Unary, should not have occured"))
        }
    }

    fn visit_binary(&self, element: &crate::expressions::binary::Binary) -> Result<Literal, String> {
        panic!()
    }

}

impl Interpreter {
    fn evaluate(&self, expression: &Box<dyn Expression>) -> Result<Literal, String> {
        expression.accept_interpreter(self)
    }

    fn runtime_error(&self, token: &Token, message: &str) -> String {
        Lox::error_token(token, message);
        message.to_string()
    }

    fn is_truthy(&self, literal: &Literal) -> bool {
        match literal.literal_type {
            LiteralEnum::NIL => false,
            LiteralEnum::BOOLEAN => literal.value.downcast_ref::<bool>().unwrap().clone(),
            _ => true
        }
    }

}
