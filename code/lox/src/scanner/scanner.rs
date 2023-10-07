use crate::lox::Lox;

use super::super::token::token::Token;
use super::super::token::token_type::Token_Type;

struct Scanner {
    source: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
    an_error_occured: bool,
}

#[allow(dead_code)]
impl Scanner {
    pub fn new(source: &String) -> Scanner {
        Scanner {
            source: source.chars().collect(),
            start: 0,
            current: 0,
            line: 1,
            an_error_occured: false,
        }
    }

    fn scan_tokens(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens: Vec<Token> = Vec::new();
        while self.is_at_end() {
            self.start = self.current;
            match self.scan_token() {
                Some(token) => tokens.push(token),
                None => continue,
            }
        }
        tokens.push(self.generate_token(Token_Type::EOF, None));
        if self.an_error_occured {
            return Err("Some error occured while parsing".to_string());
        }
        Ok(tokens)
    }

    fn scan_token(&mut self) -> Option<Token> {
        let current_char: char = self.advance();
        match current_char {
            // Single use char
            '(' => Some(self.generate_token(Token_Type::LEFT_PAREN, None)),
            ')' => Some(self.generate_token(Token_Type::RIGHT_PAREN, None)),
            '{' => Some(self.generate_token(Token_Type::LEFT_BRACE, None)),
            '}' => Some(self.generate_token(Token_Type::RIGHT_BRACE, None)),
            ',' => Some(self.generate_token(Token_Type::COMMA, None)),
            '.' => Some(self.generate_token(Token_Type::DOT, None)),
            '-' => Some(self.generate_token(Token_Type::MINUS, None)),
            '+' => Some(self.generate_token(Token_Type::PLUS, None)),
            ';' => Some(self.generate_token(Token_Type::SEMICOLON, None)),
            '*' => Some(self.generate_token(Token_Type::STAR, None)),
            // Double char
            '!' => match self.match_next(&'=') {
                true => Some(self.generate_token(Token_Type::BANG_EQUAL, None)),
                false => Some(self.generate_token(Token_Type::BANG, None)),
            },
            '=' => match self.match_next(&'=') {
                true => Some(self.generate_token(Token_Type::EQUAL_EQUAL, None)),
                false => Some(self.generate_token(Token_Type::EQUAL, None)),
            },
            '<' => match self.match_next(&'=') {
                true => Some(self.generate_token(Token_Type::LESS_EQUAL, None)),
                false => Some(self.generate_token(Token_Type::LESS, None)),
            },
            '>' => match self.match_next(&'=') {
                true => Some(self.generate_token(Token_Type::GREATER_EQUAL, None)),
                false => Some(self.generate_token(Token_Type::GREATER, None)),
            },
            // Comments ( like this :) )
            '/' => {
                if self.match_next(&'/') {
                    while self.peek_next() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                    return None;
                }
                Some(self.generate_token(Token_Type::SLASH, None))
            }
            // Ignore Whitespace
            ' ' => None,
            '\r' => None,
            '\t' => None,
            // Newline
            '\n' => {
                self.line += 1;
                None
            }
            // Strings
            '"' => Some(self.string()),
            // Catch remainder
            misc => {
                // Numbers
                if misc.is_digit(10) {
                    return Some(self.number());
                }
                // Reserved Key Words
                if misc.is_alphabetic() || misc == '_' {
                    return Some(self.identifier());
                }
                self.scanner_error(format!("Unknown char {} unable to be scanned", misc).trim());
                None
            }
        }
    }

    fn generate_token(&mut self, token_type: Token_Type, literal: Option<String>) -> Token {
        let literal_ = match literal {
            Some(lexem) => lexem,
            None => "".to_string(),
        };

        let lexem: String = self.source[self.start..self.current].iter().collect();
        Token::new(token_type, lexem, literal_, self.line)
    }

    // We grab the char at the current index and increment the current
    fn advance(&mut self) -> char {
        let current_char: char = self.source.get(self.current).unwrap().clone();
        self.current += 1;
        current_char
    }

    // We look at the text char and if we get what we expect return true and inc
    // otherwise return false
    fn match_next(&mut self, expected_match: &char) -> bool {
        if self.is_at_end() {
            return false;
        }
        let next_char = self.source.get(self.current).unwrap(); // We already incremented
        if next_char != expected_match {
            return false;
        }
        self.current += 1;
        true
    }

    fn peek_next(&mut self) -> char {
        self.source.get(self.current).unwrap_or(&'\0').clone()
    }

    fn peek_double_next(&mut self) -> char {
        self.source.get(self.current + 1).unwrap_or(&'\0').clone()
    }

    fn string(&mut self) -> Token {
        while self.peek_next() != '"' && !self.is_at_end() {
            if self.peek_next() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.scanner_error("Missing closing \" on string token")
        }

        self.advance(); // last " is needed

        let literal: String = self.source[self.start + 1..self.current - 1]
            .iter()
            .collect();

        return self.generate_token(Token_Type::STRING, Some(literal));
    }

    fn number(&mut self) -> Token {
        while self.peek_next().is_digit(10) {
            self.advance();
        }
        if self.peek_next() == '.' && self.peek_double_next().is_digit(10) {
            while self.peek_next().is_digit(10) {
                self.advance();
            }
        }
        let literal: String = self.source[self.start..self.current].iter().collect();

        return self.generate_token(Token_Type::NUMBER, Some(literal));
    }

    fn identifier(&mut self) -> Token {
        while self.peek_next().is_alphabetic() || self.peek_next() == '_' {
            self.advance();
        }

        let identifier: String = self.source[self.start..self.current].iter().collect();
        let token_type: Token_Type = match identifier.trim() {
            "and" => Token_Type::AND,
            "class" => Token_Type::CLASS,
            "else" => Token_Type::ELSE,
            "false" => Token_Type::FALSE,
            "fun" => Token_Type::FUN,
            "for" => Token_Type::FOR,
            "if" => Token_Type::IF,
            "nil" => Token_Type::NIL,
            "or" => Token_Type::OR,
            "print" => Token_Type::PRINT,
            "return" => Token_Type::RETURN,
            "super" => Token_Type::SUPER,
            "this" => Token_Type::THIS,
            "true" => Token_Type::TRUE,
            "var" => Token_Type::VAR,
            "while" => Token_Type::WHILE,
            _ => Token_Type::IDENTIFIER,
        };
        return self.generate_token(token_type, None);
    }

    fn is_at_end(&mut self) -> bool {
        self.current >= self.source.len()
    }

    fn scanner_error(&mut self, message: &str) {
        Lox::error(self.line, message.to_string());
        self.an_error_occured = true;
    }
}
