use crate::expressions::binary::Binary;
use crate::expressions::expression::Expression;
use crate::expressions::grouping::Grouping;
use crate::expressions::literal::{Literal, LiteralEnum};
use crate::expressions::unary::Unary;
use crate::lox::Lox;
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

    pub fn parse(&mut self) -> Result<Box<dyn Expression>, String> {
        self.expression()
    }

    // equality       → comparison ( ( "!=" | "==" ) comparison )* ;
    fn expression(&mut self) -> Result<Box<dyn Expression>, String> {
        let mut expr: Box<dyn Expression> = match self.comparison() {
            Ok(expr) => expr,
            Err(m) => return Err(m),
        };

        let expression_vector = vec![Token_Type::BANG_EQUAL, Token_Type::EQUAL_EQUAL];
        while self.match_token(&expression_vector) {
            let operator = self.previous().clone();
            let right = match self.comparison() {
                Ok(expr) => expr,
                Err(m) => return Err(m),
            };
            expr = Box::new(Binary::new(expr, operator, right));
        }
        Ok(expr)
    }

    // comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    fn comparison(&mut self) -> Result<Box<dyn Expression>, String> {
        let mut expr: Box<dyn Expression> = match self.term() {
            Ok(expr) => expr,
            Err(m) => return Err(m),
        };

        let comparison_vector = vec![
            Token_Type::GREATER,
            Token_Type::GREATER_EQUAL,
            Token_Type::LESS,
            Token_Type::LESS_EQUAL,
        ];
        while self.match_token(&comparison_vector) {
            let operator = self.previous().clone();
            let right = match self.term() {
                Ok(expr) => expr,
                Err(m) => return Err(m),
            };
            expr = Box::new(Binary::new(expr, operator, right))
        }
        Ok(expr)
    }

    // term           → factor ( ( "-" | "+" ) factor )* ;
    fn term(&mut self) -> Result<Box<dyn Expression>, String> {
        let mut expr: Box<dyn Expression> = match self.factor() {
            Ok(expr) => expr,
            Err(m) => return Err(m),
        };

        let term_vector = vec![Token_Type::MINUS, Token_Type::PLUS];
        while self.match_token(&term_vector) {
            let operator = self.previous().clone();
            let right = match self.factor() {
                Ok(expr) => expr,
                Err(m) => return Err(m),
            };
            expr = Box::new(Binary::new(expr, operator, right))
        }
        Ok(expr)
    }

    // factor         → unary ( ( "/" | "*" ) unary )* ;
    fn factor(&mut self) -> Result<Box<dyn Expression>, String> {
        let mut expr: Box<dyn Expression> = match self.unary() {
            Ok(expr) => expr,
            Err(m) => return Err(m),
        };

        let factor_vector = vec![Token_Type::SLASH, Token_Type::STAR];
        while self.match_token(&factor_vector) {
            let operator = self.previous().clone();
            let right = match self.unary() {
                Ok(right) => right,
                Err(message) => return Err(message),
            };
            expr = Box::new(Binary::new(expr, operator, right));
        }
        Ok(expr)
    }

    // unary          → ( "!" | "-" ) unary | primary
    fn unary(&mut self) -> Result<Box<dyn Expression>, String> {
        let unary_vector = vec![Token_Type::BANG, Token_Type::MINUS];
        if self.match_token(&unary_vector) {
            let operator = self.previous().clone();
            let right = match self.unary() {
                Ok(right) => right,
                Err(message) => return Err(message),
            };
            return Ok(Box::new(Unary::new(operator, right)));
        }
        self.primary()
    }

    // → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")"
    fn primary(&mut self) -> Result<Box<dyn Expression>, String> {
        if self.match_token(&vec![Token_Type::TRUE]) {
            return Ok(Box::new(Literal::new(Box::new(true), LiteralEnum::BOOLEAN)));
        }
        if self.match_token(&vec![Token_Type::FALSE]) {
            return Ok(Box::new(Literal::new(
                Box::new(false),
                LiteralEnum::BOOLEAN,
            )));
        }
        if self.match_token(&vec![Token_Type::NIL]) {
            return Ok(Box::new(Literal::new(Box::new(0), LiteralEnum::NIL)));
        }
        if self.match_token(&vec![Token_Type::NUMBER]) {
            return Ok(Box::new(Literal::new(
                Box::new(
                    self.previous()
                        .literal
                        .parse::<f64>()
                        .expect("Failed to convert string to number"),
                ),
                LiteralEnum::NUMBER,
            )));
        }
        if self.match_token(&vec![Token_Type::STRING]) {
            return Ok(Box::new(Literal::new(
                Box::new(self.previous().literal.clone()),
                LiteralEnum::STRING,
            )));
        }

        // "(" expression ")"
        if self.match_token(&vec![Token_Type::LEFT_PAREN]) {
            let expr = match self.expression() {
                Ok(expr) => expr,
                Err(m) => return Err(m),
            };
            match self.consume(&Token_Type::RIGHT_PAREN, "Expect ')' after expression") {
                Ok(_) => {}
                Err(err) => return Err(err),
            };
            return Ok(Box::new(Grouping::new(expr)));
        }

        Err(self.parser_error(self.peek(), "Expected expression"))
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

    fn consume(&mut self, expected: &Token_Type, error_message: &str) -> Result<&Token, String> {
        if self.check(expected) {
            return Ok(self.advance());
        }
        Err(self.parser_error(self.peek(), error_message))
    }

    fn parser_error(&self, token: &Token, message: &str) -> String {
        Lox::error_token(token, message);
        message.to_string()
    }

    // From chapter 3, not set up currently
    // private void synchronize() {
    //   advance();
    //
    //   while (!isAtEnd()) {
    //     if (previous().type == SEMICOLON) return;
    //
    //     switch (peek().type) {
    //       case CLASS:
    //       case FUN:
    //       case VAR:
    //       case FOR:
    //       case IF:
    //       case WHILE:
    //       case PRINT:
    //       case RETURN:
    //         return;
    //     }
    //
    //     advance();
    //   }
    // }
}

#[cfg(test)]
mod tests {
    use super::Parser;
    use crate::{expressions::printer::Printer, scanner::scanner::Scanner};

    fn compare_code_to_expression(code: &str, expected: &str) {
        let mut scanner = Scanner::new(&code.to_string());
        let tokens = scanner.scan_tokens().ok().unwrap();
        let mut parser = Parser::new(&tokens);
        let printer = Printer {};
        assert_eq!(printer.convert(parser.parse().ok().unwrap()), expected);
    }

    fn compare_code_to_err(code: &str, expected: &str) {
        let mut scanner = Scanner::new(&code.to_string());
        let tokens = scanner.scan_tokens().ok().unwrap();
        let mut parser = Parser::new(&tokens);
        assert_eq!(parser.parse().err().unwrap(), expected);
    }

    #[test]
    fn test_all_success_parser() {
        // Expression
        compare_code_to_expression(
            "true != ( (3-4*3+2) <= 6.23 )",
            "(!= true ((<= ((+ (- 3 (* 4 3)) 2)) 6.23)))",
        );

        // Equality
        compare_code_to_expression("4 == 3", "(== 4 3)");
        compare_code_to_expression("4 != 3", "(!= 4 3)");

        // Comparison
        compare_code_to_expression("4 < 3", "(< 4 3)");
        compare_code_to_expression("4 > 3", "(> 4 3)");
        compare_code_to_expression("4 >= 3", "(>= 4 3)");
        compare_code_to_expression("4 <= 3", "(<= 4 3)");
        compare_code_to_expression("4 <= 3 < 5", "(< (<= 4 3) 5)");
        compare_code_to_expression("4 <= (3 < 5)", "(<= 4 ((< 3 5)))");

        // Term
        compare_code_to_expression("5 + 5", "(+ 5 5)");
        compare_code_to_expression("5.1 - 5", "(- 5.1 5)");
        compare_code_to_expression("5 + 5 - 4 + 2 - 1", "(- (+ (- (+ 5 5) 4) 2) 1)");

        //Factor
        compare_code_to_expression("4 * 3", "(* 4 3)");
        compare_code_to_expression("4 * 3 * 2", "(* (* 4 3) 2)");
        compare_code_to_expression("4 / 3", "(/ 4 3)");
        compare_code_to_expression("4 / 3 * 2", "(* (/ 4 3) 2)");

        // Unary
        compare_code_to_expression("-4 * 3", "(* (- 4) 3)");
        compare_code_to_expression("! true", "(! true)");
        compare_code_to_expression("! false", "(! false)");

        // Primary
        compare_code_to_expression("\"Hello World\"", "Hello World");
        compare_code_to_expression("36434.23232", "36434.23232");
        compare_code_to_expression("true", "true");
        compare_code_to_expression("false", "false");
        compare_code_to_expression("nil", "nil");
        compare_code_to_expression("( nil )", "(nil)");
    }

    #[test]
    fn test_all_error_parser() {
        compare_code_to_err("( 4", "Expect ')' after expression");
        compare_code_to_err("", "Expected expression");
    }
}
