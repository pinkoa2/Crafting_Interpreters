use super::{expression::Expression, visitor::Visitor};

#[allow(dead_code)]
pub struct Literal {
    pub value: String,
}

impl Expression for Literal {
    fn accept(&self, visitor: &dyn Visitor) -> String {
        visitor.visit_literal(self)
    }
}
#[allow(dead_code)]
impl Literal {
    pub fn new(value: String) -> Literal {
        Literal {value}
    }
}
