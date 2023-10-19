// #[allow(unused_imports)]
// use crate::expressions::unary::Unary;
// #[allow(unused_imports)]
// use crate::expressions::binary::Binary;
// use crate::expressions::expression::Expression;
// use crate::token::token::Token;
// use crate::token::token_type::Token_Type;
//
// #[allow(dead_code)]
// pub struct Parser<'a> {
//     tokens: &'a Vec<Token>,
//     current: usize,
// }
//
// #[allow(dead_code)]
// impl<'a> Parser<'a> {
//     pub fn new(tokens: &'a Vec<Token>) -> Parser<'a> {
//         Parser { tokens, current: 0 }
//     }
//
//     // equality       → comparison ( ( "!=" | "==" ) comparison )* ;
//     // ex: 5 < 5 ( == 4 < 4 )
//     fn expression(&mut self) -> Box<dyn Expression> {
//         let mut expr: Box<dyn Expression> = self.comparison();
//
//         while self.match_token(&vec![Token_Type::BANG_EQUAL, Token_Type::EQUAL_EQUAL]) {
//             let operator = self.previous();
//             let right = self.comparison();
//             expr = Box::new(Binary::new(expr, operator.clone(), right));
//         }
//         expr
//     }
//
//     fn comparison(&mut self) -> Box<dyn Expression> {
//         panic!("Not implemented")
//     }
//
//     fn match_token(&mut self, token_types: &Vec<Token_Type>) -> bool {
//         for token_type in token_types {
//             // We look at the current token, if it matches, then we advance by 1
//             if self.check(token_type) {
//                 self.advance();
//                 return true;
//             }
//         }
//         false
//     }
//
//     fn check(&self, token_type: &Token_Type) -> bool {
//         if self.is_at_end() {
//             return false;
//         }
//         return &self.peek().token_type == token_type;
//     }
//
//     // Advances the position by 1 and returns the token it was at when called
//     fn advance(&mut self) -> &Token {
//         if !self.is_at_end() {
//             self.current += 1
//         }
//         return self.previous();
//     }
//
//     fn is_at_end(&self) -> bool {
//         return self.peek().token_type == Token_Type::EOF;
//     }
//
//     fn peek(&self) -> &Token {
//         return self.tokens.get(self.current).unwrap();
//     }
//
//     fn previous(&self) -> &Token {
//         return self.tokens.get(self.current - 1).unwrap();
//     }
// }
