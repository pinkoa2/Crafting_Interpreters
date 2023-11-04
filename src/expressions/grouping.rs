use super::{expression::Expression, visitor::Visitor, literal::Literal};

#[allow(dead_code)]
pub struct Grouping {
    pub exp: Box<dyn Expression>,
}

impl Expression for Grouping {
    fn accept_printer(&self, visitor: &dyn super::visitor::Visitor<String>) -> String {
        visitor.visit_grouping(self)
    }
    fn accept_interpreter(&self, visitor: &dyn Visitor<Result<Literal, String>>) -> Result<Literal, String> {
        visitor.visit_grouping(self)
    }
}
#[allow(dead_code)]
impl Grouping {
    pub fn new(exp: Box<dyn Expression>) -> Grouping {
        Grouping { exp }
    }
}
