use super::binary::Binary;

pub trait Visitor {
    fn visit_binary(&self, element: &Binary);
}


