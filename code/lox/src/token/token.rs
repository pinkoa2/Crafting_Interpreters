use super::token_type::Token_Type;

#[allow(dead_code)]
pub struct Token {
    token_type: Token_Type,
    lexem: String,
    literal: String, // This might need to be an object
    lint: usize,
}

#[allow(dead_code)]
impl Token {
    pub fn new(token_type: Token_Type, lexem: String, literal: String, lint: usize) -> Token {
        Token {
            token_type,
            lexem,
            literal,
            lint,
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
