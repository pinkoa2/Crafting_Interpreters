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
        while self.current >= self.source.len() {
            self.start = self.current;
            self.scan_token();
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

    fn scan_token(&mut self) -> Token {
        //
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
}
