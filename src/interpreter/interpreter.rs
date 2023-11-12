use crate::{
    expressions::{
        binary::Binary,
        expression::Expression,
        grouping::Grouping,
        literal::{Literal, LiteralEnum},
        unary::Unary,
        visitor::Visitor,
    },
    lox::Lox,
    token::{token::Token, token_type::Token_Type},
};

pub struct Interpreter {}

impl Visitor<Result<Literal, String>> for Interpreter {
    fn visit_literal(&self, element: &Literal) -> Result<Literal, String> {
        Ok(element.clone())
    }

    fn visit_grouping(&self, element: &Grouping) -> Result<Literal, String> {
        self.evaluate(&element.exp)
    }

    fn visit_unary(&self, element: &Unary) -> Result<Literal, String> {
        let operator: &Token_Type = &element.operator.token_type;
        let right = &element.right;
        let expression: Literal = match self.evaluate(right) {
            Ok(literal) => literal,
            Err(m) => return Err(m),
        };

        match operator {
            Token_Type::BANG => {
                let new_operand = !Interpreter::is_truthy(&expression);
                return Ok(Literal::new(Box::new(new_operand), LiteralEnum::BOOLEAN));
            }
            Token_Type::MINUS => {
                // Check that it's a number
                if expression.literal_type != LiteralEnum::NUMBER {
                    return Interpreter::runtime_error(
                        &element.operator,
                        "Operand must be a number.",
                    );
                }
                let value = expression.value.downcast_ref::<f64>().unwrap().clone() * -1.0;
                return Ok(Literal::new(Box::new(value), LiteralEnum::NUMBER));
            }
            _ => Interpreter::runtime_error(
                &element.operator,
                "Invalid Unary, shoud not have occured",
            ),
        }
    }

    fn visit_binary(&self, element: &Binary) -> Result<Literal, String> {
        let operator: &Token = &element.operator;
        let op: &Token_Type = &operator.token_type;
        let right: Literal = match self.evaluate(&element.right) {
            Ok(literal) => literal,
            Err(m) => return Err(m),
        };
        let left: Literal = match self.evaluate(&element.left) {
            Ok(literal) => literal,
            Err(m) => return Err(m),
        };

        match op {
            Token_Type::MINUS => Interpreter::handle_both_numbers(&left, operator, &right),
            Token_Type::SLASH => Interpreter::handle_both_numbers(&left, operator, &right),
            Token_Type::STAR => Interpreter::handle_both_numbers(&left, operator, &right),
            Token_Type::GREATER => Interpreter::handle_both_numbers(&left, operator, &right),
            Token_Type::GREATER_EQUAL => Interpreter::handle_both_numbers(&left, operator, &right),
            Token_Type::LESS => Interpreter::handle_both_numbers(&left, operator, &right),
            Token_Type::LESS_EQUAL => Interpreter::handle_both_numbers(&left, operator, &right),
            Token_Type::EQUAL_EQUAL => Interpreter::handle_comparison_case(&left, operator, &right),
            Token_Type::BANG_EQUAL => Interpreter::handle_comparison_case(&left, operator, &right),
            Token_Type::PLUS => Interpreter::handle_plus_case(&left, operator, &right),
            _ => Interpreter::runtime_error(operator, "Unexpected token type in binary operation"),
        }
    }
}

impl Interpreter {
    pub fn evaluate(&self, expression: &Box<dyn Expression>) -> Result<Literal, String> {
        expression.accept_interpreter(self)
    }

    fn runtime_error(token: &Token, message: &str) -> Result<Literal, String> {
        Lox::error_token(token, message);
        Err(message.to_string())
    }

    fn is_truthy(literal: &Literal) -> bool {
        match literal.literal_type {
            LiteralEnum::NIL => false,
            LiteralEnum::BOOLEAN => literal.value.downcast_ref::<bool>().unwrap().clone(),
            _ => true,
        }
    }

    fn is_equal(left_literal: &Literal, right_literal: &Literal) -> bool {
        let left_type = &left_literal.literal_type;
        let right_type = &right_literal.literal_type;

        // If not the same type, then not equal
        if left_type != right_type {
            return false;
        }

        // If both are nil, they are equal
        if left_type == right_type && left_type == &LiteralEnum::NIL {
            return true;
        }

        if left_type == right_type && left_type == &LiteralEnum::BOOLEAN {
            let left_bool = left_literal.value.downcast_ref::<bool>().unwrap();
            let right_bool = right_literal.value.downcast_ref::<bool>().unwrap();
            return left_bool == right_bool;
        }

        if left_type == right_type && left_type == &LiteralEnum::NUMBER {
            let left_num = left_literal.value.downcast_ref::<f64>().unwrap();
            let right_num = right_literal.value.downcast_ref::<f64>().unwrap();
            return left_num == right_num;
        }

        if left_type == right_type && left_type == &LiteralEnum::STRING {
            let left_string = left_literal.value.downcast_ref::<String>().unwrap();
            let right_string = right_literal.value.downcast_ref::<String>().unwrap();
            return left_string == right_string;
        }

        false
    }

    fn handle_both_numbers(
        left: &Literal,
        token: &Token,
        right: &Literal,
    ) -> Result<Literal, String> {
        if left.literal_type != LiteralEnum::NUMBER && right.literal_type != LiteralEnum::NUMBER {
            return Interpreter::runtime_error(token, "Both operands must be a number");
        }
        let left_value = left.value.downcast_ref::<f64>().unwrap().clone();
        let right_value = right.value.downcast_ref::<f64>().unwrap().clone();
        let op = &token.token_type;

        match op {
            Token_Type::MINUS => Ok(Literal::new(
                Box::new(left_value - right_value),
                LiteralEnum::NUMBER,
            )),
            Token_Type::STAR => Ok(Literal::new(
                Box::new(left_value * right_value),
                LiteralEnum::NUMBER,
            )),
            Token_Type::SLASH => Ok(Literal::new(
                Box::new(left_value / right_value),
                LiteralEnum::NUMBER,
            )),
            Token_Type::GREATER => Ok(Literal::new(
                Box::new(left_value > right_value),
                LiteralEnum::BOOLEAN,
            )),
            Token_Type::GREATER_EQUAL => Ok(Literal::new(
                Box::new(left_value >= right_value),
                LiteralEnum::BOOLEAN,
            )),
            Token_Type::LESS => Ok(Literal::new(
                Box::new(left_value < right_value),
                LiteralEnum::BOOLEAN,
            )),
            Token_Type::LESS_EQUAL => Ok(Literal::new(
                Box::new(left_value <= right_value),
                LiteralEnum::BOOLEAN,
            )),
            _ => Interpreter::runtime_error(
                token,
                "Unexpected conditions operation with two numbers",
            ),
        }
    }

    fn handle_comparison_case(
        left_literal: &Literal,
        token: &Token,
        right_literal: &Literal,
    ) -> Result<Literal, String> {
        let result_of_comparison = Interpreter::is_equal(left_literal, right_literal);

        let op = &token.token_type;
        match op {
            Token_Type::EQUAL_EQUAL => Ok(Literal::new(
                Box::new(result_of_comparison),
                LiteralEnum::BOOLEAN,
            )),
            Token_Type::BANG_EQUAL => Ok(Literal::new(
                Box::new(!result_of_comparison),
                LiteralEnum::BOOLEAN,
            )),
            _ => {
                Interpreter::runtime_error(token, "Unexpected comparision with non comparison type")
            }
        }
    }

    fn handle_plus_case(
        left_literal: &Literal,
        token: &Token,
        right_literal: &Literal,
    ) -> Result<Literal, String> {
        let left_type = &left_literal.literal_type;
        let right_type = &right_literal.literal_type;

        if left_type != &LiteralEnum::STRING && left_type != &LiteralEnum::NUMBER {
            return Interpreter::runtime_error(token, "Operand must be either a string or number");
        }

        if right_type != &LiteralEnum::STRING && right_type != &LiteralEnum::NUMBER {
            return Interpreter::runtime_error(token, "Operand must be either a string or number");
        }

        if left_type == right_type && left_type == &LiteralEnum::NUMBER {
            let left_num = left_literal.value.downcast_ref::<f64>().unwrap();
            let right_num = right_literal.value.downcast_ref::<f64>().unwrap();
            return Ok(Literal::new(
                Box::new(left_num + right_num),
                LiteralEnum::NUMBER,
            ));
        }

        if left_type == right_type && left_type == &LiteralEnum::STRING {
            let left_string = left_literal.value.downcast_ref::<String>().unwrap();
            let right_string = right_literal.value.downcast_ref::<String>().unwrap();
            return Ok(Literal::new(
                Box::new(left_string.to_string() + right_string),
                LiteralEnum::STRING,
            ));
        }

        Interpreter::runtime_error(token, "Unexpected case for addition")
    }
}
