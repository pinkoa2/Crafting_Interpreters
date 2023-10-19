use super::{expression::Expression, printer::Visitor};
use crate::token::token::Token;

#[allow(dead_code)]
pub struct Unary {
    pub operator: Token,
    pub right: Box<dyn Expression>,
}

impl<'a> Expression for Unary {
    fn accept(&self, visitor: &dyn Visitor) -> String {
        visitor.visit_unary(self)
    }
}

#[allow(dead_code)]
impl Unary {
    pub fn new(operator: Token, right: Box<dyn Expression>) -> Unary {
        Unary { operator, right }
    }
}
