use super::{binary::Binary, grouping::Grouping, literal::Literal, unary::Unary};

pub trait Visitor<ReturnType> {
    fn visit_binary(&self, element: &Binary) -> ReturnType;
    fn visit_literal(&self, element: &Literal) -> ReturnType;
    fn visit_unary(&self, element: &Unary) -> ReturnType;
    fn visit_grouping(&self, element: &Grouping) -> ReturnType;
}

/*
 
Is Dr. Asnis taking new patients and would like to make an appointment for my back


Do you take my insurance 

    Blue Cross Blue Shield Anthem

    866-211-6588


    617-726-7500

    8236044

    1175 Cambridge Street Boston 
    165kj
*/
