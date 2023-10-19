use super::expression::Expression;

#[allow(dead_code)]
pub struct Grouping {
    pub exp: Box<dyn Expression>,
}

impl Expression for Grouping {
    fn accept(&self, visitor: &dyn super::printer::Visitor) -> String {
        visitor.visit_grouping(self)
    }
}
#[allow(dead_code)]
impl Grouping {
    pub fn new(exp: Box<dyn Expression>) -> Grouping {
        Grouping { exp }
    }
}
