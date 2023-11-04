use super::{literal::Literal, visitor::Visitor};

pub trait Expression {
    fn accept_printer(&self, visitor: &dyn Visitor<String>) -> String;
    fn accept_interpreter(&self, visitor: &dyn Visitor<Result<Literal, String>>) -> Result<Literal, String>;
}
