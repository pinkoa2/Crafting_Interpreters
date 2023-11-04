use super::{expression::Expression, visitor::Visitor, literal::Literal};
use crate::token::token::Token;

#[allow(dead_code)]
pub struct Binary {
    pub left: Box<dyn Expression>,
    pub operator: Token,
    pub right: Box<dyn Expression>,
}

impl Expression for Binary {
    fn accept_printer(&self, visitor: &dyn Visitor<String>) -> String {
        visitor.visit_binary(self)
    }
    fn accept_interpreter(&self, visitor: &dyn Visitor<Result<Literal, String>>) -> Result<Literal, String> {
        visitor.visit_binary(self)
    }
}
#[allow(dead_code)]
impl Binary {
    pub fn new(left: Box<dyn Expression>, operator: Token, right: Box<dyn Expression>) -> Binary {
        Binary {
            left,
            operator,
            right,
        }
    }
}
