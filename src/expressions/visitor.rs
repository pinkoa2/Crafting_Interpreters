use super::{binary::Binary, grouping::Grouping, literal::Literal, unary::Unary};

pub trait Visitor<ReturnType> {
    fn visit_binary(&self, element: &Binary) -> ReturnType;
    fn visit_literal(&self, element: &Literal) -> ReturnType;
    fn visit_unary(&self, element: &Unary) -> ReturnType;
    fn visit_grouping(&self, element: &Grouping) -> ReturnType;
}
