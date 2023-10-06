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
        tokens.push(Token::new(
            // TODO: inaccurate
            Token_Type::EOF,
            "".to_string(),
            "".to_string(),
            0,
        ));
        tokens
    }

    fn scan_token(&mut self) -> Option<Token> {
        let current_char: char = self.advance();
        match current_char {
            // Single Use Characters can refactor nicely
            '(' => Some(self.generate_token(Token_Type::LEFT_PAREN, None, None, self.line)),
            ')' => Some(self.generate_token(Token_Type::RIGHT_PAREN, None, None, self.line)),
            '{' => Some(self.generate_token(Token_Type::LEFT_BRACE, None, None, self.line)),
            '}' => Some(self.generate_token(Token_Type::RIGHT_BRACE, None, None, self.line)),
            ',' => Some(self.generate_token(Token_Type::COMMA, None, None, self.line)),
            '.' => Some(self.generate_token(Token_Type::DOT, None, None, self.line)),
            '-' => Some(self.generate_token(Token_Type::MINUS, None, None, self.line)),
            '+' => Some(self.generate_token(Token_Type::PLUS, None, None, self.line)),
            ';' => Some(self.generate_token(Token_Type::SEMICOLON, None, None, self.line)),
            '/' => Some(self.generate_token(Token_Type::SLASH, None, None, self.line)),
            '*' => Some(self.generate_token(Token_Type::SLASH, None, None, self.line)),
            // Double char examples: can refactor
            '!' => match self.match_next(&'=') {
                true => Some(self.generate_token(Token_Type::BANG_EQUAL, None, None, self.line)),
                false => Some(self.generate_token(Token_Type::BANG, None, None, self.line)),
            },
            '=' => match self.match_next(&'=') {
                true => Some(self.generate_token(Token_Type::EQUAL_EQUAL, None, None, self.line)),
                false => Some(self.generate_token(Token_Type::EQUAL, None, None, self.line)),
            },
            '<' => match self.match_next(&'=') {
                true => Some(self.generate_token(Token_Type::LESS_EQUAL, None, None, self.line)),
                false => Some(self.generate_token(Token_Type::LESS, None, None, self.line)),
            },
            '>' => match self.match_next(&'=') {
                true => Some(self.generate_token(Token_Type::GREATER_EQUAL, None, None, self.line)),
                false => Some(self.generate_token(Token_Type::GREATER, None, None, self.line)),
            },
            '@' => None,
            _ => panic!("TODO"),
        }
    }

    fn generate_token(
        &mut self,
        token_type: Token_Type,
        lexem: Option<String>,
        literal: Option<String>,
        line: usize,
    ) -> Token {
        let lexem_ = match lexem {
            Some(lexem) => lexem,
            None => "".to_string(),
        };
        let literal_ = match literal {
            Some(lexem) => lexem,
            None => "".to_string(),
        };
        Token::new(token_type, lexem_, literal_, line)
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

    fn is_at_end(&mut self) -> bool {
        self.current >= self.source.len()
    }
}
