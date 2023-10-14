use super::expression::Expression;

#[allow(dead_code)]
pub struct Grouping<'a> {
    pub exp: &'a dyn Expression,
}

impl<'a> Expression for Grouping<'a> {
    fn accept(&self, visitor: &dyn super::printer::Visitor) -> String {
        visitor.visit_grouping(self)
    }
}
#[allow(dead_code)]
impl<'a> Grouping<'a> {
    pub fn new(exp: &'a dyn Expression) -> Grouping<'a> {
        Grouping { exp }
    }
}
