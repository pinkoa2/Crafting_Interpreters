use super::visitor::Visitor;

pub trait Expression {
    fn accept(&self, visitor: &dyn Visitor) -> String;
    fn expression_shared_fn(&self) {
        println!("This is a shared")
    }
}
