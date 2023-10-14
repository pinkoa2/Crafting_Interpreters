use super::{expression::Expression, printer::Visitor};
use crate::token::token::Token;

#[allow(dead_code)]
pub struct Unary<'a> {
    pub operator: Token,
    pub right: &'a dyn Expression,
}

impl<'a> Expression for Unary<'a> {
    fn accept(&self, visitor: &dyn Visitor) -> String {
        visitor.visit_unary(self)
    }
}

#[allow(dead_code)]
impl<'a> Unary<'a> {
    pub fn new(operator: Token, right: &'a dyn Expression) -> Unary<'a> {
        Unary { operator, right }
    }
}
