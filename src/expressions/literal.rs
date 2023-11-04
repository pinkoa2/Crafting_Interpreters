use super::{expression::Expression, visitor::Visitor};
use std::any::Any;

#[derive(PartialEq, Eq, Clone)]
pub enum LiteralEnum {
    NIL,
    BOOLEAN,
    NUMBER,
    STRING,
}

pub struct Literal {
    pub value: Box<dyn Any>,
    pub literal_type: LiteralEnum,
}

impl Clone for Literal {
    fn clone(&self) -> Literal {
        let literal_type = self.literal_type.clone();
        let value: Box<dyn Any> = match literal_type {
            LiteralEnum::NIL => Box::new(0),
            LiteralEnum::BOOLEAN => Box::new(self.value.downcast_ref::<bool>().unwrap().clone()),
            LiteralEnum::NUMBER => Box::new(self.value.downcast_ref::<f64>().unwrap().clone()),
            LiteralEnum::STRING => Box::new(self.value.downcast_ref::<String>().unwrap().clone()),
        };
        Literal {
            value,
            literal_type
        }
    }
}

impl Expression for Literal {
    fn accept_printer(&self, visitor: &dyn Visitor<String>) -> String {
        visitor.visit_literal(self)
    }
    fn accept_interpreter(&self, visitor: &dyn Visitor<Result<Literal, String>>) -> Result<Literal, String> {
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
