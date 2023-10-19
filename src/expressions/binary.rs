use super::{expression::Expression, printer::Visitor};
use crate::token::token::Token;

#[allow(dead_code)]
pub struct Binary {
    pub left: Box<dyn Expression>,
    pub operator: Token,
    pub right: Box<dyn Expression>,
}

impl Expression for Binary {
    fn accept(&self, visitor: &dyn Visitor) -> String {
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
