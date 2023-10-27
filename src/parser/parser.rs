#[allow(unused_imports)]
use crate::expressions::binary::Binary;
use crate::expressions::expression::Expression;
use crate::expressions::literal::{Literal, LiteralEnum};
#[allow(unused_imports)]
use crate::expressions::unary::Unary;
use crate::token::token::Token;
use crate::token::token_type::Token_Type;

#[allow(dead_code)]
pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    current: usize,
}

#[allow(dead_code)]
impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Parser<'a> {
        Parser { tokens, current: 0 }
    }

    // equality       → comparison ( ( "!=" | "==" ) comparison )* ;
    fn expression(&mut self) -> Box<dyn Expression> {
        let mut expr: Box<dyn Expression> = self.comparison();

        let expression_vector = vec![Token_Type::BANG_EQUAL, Token_Type::EQUAL_EQUAL];
        while self.match_token(&expression_vector) {
            let operator = self.previous().clone();
            let right = self.comparison();
            expr = Box::new(Binary::new(expr, operator, right));
        }
        expr
    }

    // comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    fn comparison(&mut self) -> Box<dyn Expression> {
        let mut expr: Box<dyn Expression> = self.term();

        let comparison_vector = vec![
            Token_Type::GREATER,
            Token_Type::GREATER_EQUAL,
            Token_Type::LESS,
            Token_Type::LESS_EQUAL,
        ];
        while self.match_token(&comparison_vector) {
            let operator = self.previous().clone();
            let right = self.term();
            expr = Box::new(Binary::new(expr, operator, right))
        }
        expr
    }

    // term           → factor ( ( "-" | "+" ) factor )* ;
    fn term(&mut self) -> Box<dyn Expression> {
        let mut expr: Box<dyn Expression> = self.factor();

        let term_vector = vec![Token_Type::MINUS, Token_Type::PLUS];
        while self.match_token(&term_vector) {
            let operator = self.previous().clone();
            let right = self.factor();
            expr = Box::new(Binary::new(expr, operator, right))
        }
        expr
    }

    // factor         → unary ( ( "/" | "*" ) unary )* ;
    fn factor(&mut self) -> Box<dyn Expression> {
        let mut expr = self.unary();

        let factor_vector = vec![Token_Type::SLASH, Token_Type::STAR];
        while self.match_token(&factor_vector) {
            let operator = self.previous().clone();
            let right = self.unary();
            expr = Box::new(Binary::new(expr, operator, right));
        }
        expr
    }

    // unary          → ( "!" | "-" ) unary | primary
    fn unary(&mut self) -> Box<dyn Expression> {
        let unary_vector = vec![Token_Type::BANG, Token_Type::MINUS];
        if self.match_token(&unary_vector) {
            let operator = self.previous().clone();
            let right = self.unary();
            return Box::new(Unary::new(operator, right));
        }
        self.primary()
    }

    // → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")"
    fn primary(&mut self) -> Box<dyn Expression> {
        if self.match_token(&vec![Token_Type::TRUE]) {
            return Box::new(Literal::new(Box::new(true), LiteralEnum::BOOLEAN));
        }
        if self.match_token(&vec![Token_Type::FALSE]) {
            return Box::new(Literal::new(Box::new(false), LiteralEnum::BOOLEAN));
        }
        if self.match_token(&vec![Token_Type::NIL]) {
            return Box::new(Literal::new(Box::new(0), LiteralEnum::NIL));
        }
        if self.match_token(&vec![Token_Type::NUMBER]) {
            return Box::new(Literal::new(Box::new(self.previous().literal.parse::<f64>()), LiteralEnum::NUMBER));
        }
        if self.match_token(&vec![Token_Type::STRING]) {
            return Box::new(Literal::new(Box::new(self.previous().literal.clone()), LiteralEnum::STRING));
        }

        panic!("Not Implemented")
    }

    fn match_token(&mut self, token_types: &Vec<Token_Type>) -> bool {
        for token_type in token_types {
            // We look at the current token, if it matches, then we advance by 1
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: &Token_Type) -> bool {
        if self.is_at_end() {
            return false;
        }
        return &self.peek().token_type == token_type;
    }

    // Advances the position by 1 and returns the token it was at when called
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1
        }
        return self.previous();
    }

    fn is_at_end(&self) -> bool {
        return self.peek().token_type == Token_Type::EOF;
    }

    fn peek(&self) -> &Token {
        return self.tokens.get(self.current).unwrap();
    }

    fn previous(&self) -> &Token {
        return self.tokens.get(self.current - 1).unwrap();
    }
}
