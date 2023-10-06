use super::super::token::token::Token;
use super::super::token::token_type::Token_Type;

struct Scanner {
    source: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
}

#[allow(dead_code)]
impl Scanner {
    pub fn new(source: &String) -> Scanner {
        Scanner {
            source: source.chars().collect(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn scan_tokens(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        while self.is_at_end() {
            self.start = self.current;
            match self.scan_token() {
                Some(token) => tokens.push(token),
                None => continue,
            }
        }
        tokens.push(self.generate_token(Token_Type::EOF, None));
        tokens
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
            _ => panic!("TODO"),
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
        let current_char: char = match self.source.get(self.current) {
            Some(c) => c.clone(),
            None => panic!("Error in advance, must have a miss index"),
        };
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

    fn string(&mut self) -> Token {}

    fn is_at_end(&mut self) -> bool {
        self.current >= self.source.len()
    }
}

/*
Some rando tests
*/
