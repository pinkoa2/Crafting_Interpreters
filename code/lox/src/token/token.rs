use super::token_type::Token_Type;

/*

   Notes:

   So the token type is suppose to be the type
   The lexem is the string that we get for example "=!"
   The literal is for type. So we have a string: then literal should be the value of the string
   line is the line

*/

#[allow(dead_code)]
pub struct Token {
    token_type: Token_Type,
    lexem: String,
    literal: String, // This might need to be an object
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
