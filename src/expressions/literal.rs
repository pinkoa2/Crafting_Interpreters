use super::{expression::Expression, printer::Visitor};
use std::any::Any;

#[derive(PartialEq, Eq, Clone)]
pub enum LiteralEnum {
    NIL,
    BOOLEAN,
    NUMBER,
    STRING,
}

#[allow(dead_code)]
pub struct Literal {
    pub value: Box<dyn Any>,
    pub literal_type: LiteralEnum,
}

impl Expression for Literal {
    fn accept(&self, visitor: &dyn Visitor) -> String {
        visitor.visit_literal(self)
    }
}
#[allow(dead_code)]
impl Literal {
    pub fn new(value: Box<dyn Any>, literal_type: LiteralEnum) -> Literal {
        Literal {
            value,
            literal_type,
        }
    }
}
