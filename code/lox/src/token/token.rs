use super::token_type::Token_Type;

#[allow(dead_code)]
struct Token {
    token_type: Token_Type,
    lexem: String,
    literal: String, // This might need to be an object
    lint: i32,
}

#[allow(dead_code)]
impl Token {
    pub fn new(token_type: Token_Type, lexem: String, literal: String, lint: i32) -> Token {
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
