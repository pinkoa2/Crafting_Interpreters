use super::{binary::Binary, literal::Literal};

pub trait Visitor {
    fn visit_binary(&self, element: &Binary) -> String;
    fn visit_literal(&self, element: &Literal) -> String;
}


