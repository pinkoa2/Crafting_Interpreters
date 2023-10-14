use super::visitor::Visitor;

pub struct ExpressionPrinter {}
#[allow(dead_code)]

impl Visitor for ExpressionPrinter {
    fn visit_binary(&self, element: &super::binary::Binary) {
        panic!("Not implemented")
    }
}
impl ExpressionPrinter {
    pub fn print() {}
}
