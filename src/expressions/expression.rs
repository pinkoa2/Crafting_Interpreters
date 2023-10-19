use super::printer::Visitor;

pub trait Expression {
    fn accept(&self, visitor: &dyn Visitor) -> String;
}
