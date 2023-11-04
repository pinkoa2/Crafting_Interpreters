use super::{expression::Expression, visitor::Visitor, literal::Literal};
use crate::token::token::Token;

#[allow(dead_code)]
pub struct Unary {
    pub operator: Token,
    pub right: Box<dyn Expression>,
}

impl<'a> Expression for Unary {
    fn accept_printer(&self, visitor: &dyn Visitor<String>) -> String {
        visitor.visit_unary(self)
    }
    fn accept_interpreter(&self, visitor: &dyn Visitor<Result<Literal, String>>) -> Result<Literal, String> {
        visitor.visit_unary(self)
    }
}

#[allow(dead_code)]
impl Unary {
    pub fn new(operator: Token, right: Box<dyn Expression>) -> Unary {
        Unary { operator, right }
    }
}
