use super::{expression::Expression, visitor::Visitor};
use crate::token::token::Token;

#[allow(dead_code)]
pub struct Binary<'a> {
    left: &'a dyn Expression,
    operator: Token,
    right: &'a dyn Expression,
}

impl<'a> Expression for Binary<'a> {
    fn accept(&self, visitor: &dyn Visitor) {
        visitor.visit_binary(self)
    }

}
#[allow(dead_code)]
impl<'a> Binary<'a> {
    pub fn new(left: &'a dyn Expression, operator: Token, right: &'a dyn Expression) -> Binary<'a> {
        Binary {
            left,
            operator,
            right,
        }
    }
}
