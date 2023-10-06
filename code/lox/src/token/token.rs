use super::token_type::Token_Type;

#[allow(dead_code)]
pub struct Token {
    token_type: Token_Type,
    lexem: String,
    literal: String,
    line: usize,
}

#[allow(dead_code)]
impl Token {
    pub fn new(token_type: Token_Type, lexem: String, literal: String, line: usize) -> Token {
        Token {
            token_type,
            lexem,
            literal,
            line,
        }
    }

    pub fn to_string(&self) -> String {
        return format!(
            "{} {} {}",
            self.token_type.to_string(),
            self.lexem,
            self.literal
        );
    }
}
