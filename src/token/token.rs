use super::token_type::Token_Type;

#[allow(dead_code)]
pub struct Token {
    pub token_type: Token_Type,
    pub lexem: String,
    pub literal: String,
    pub line: usize,
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
            "{} {} {} {}",
            self.token_type.to_string(),
            self.lexem,
            self.literal,
            self.line
        );
    }

    pub fn test_string(&self) -> String {
        return format!(
            "'{}' '{}' '{}' '{}'",
            self.token_type.to_string(),
            self.lexem,
            self.literal,
            self.line
        );
    }
}

#[cfg(test)]
mod tests {
    use super::Token;
    use super::Token_Type;

    #[test]
    fn test_token_to_string() {
        let token = Token {
            token_type: Token_Type::LEFT_PAREN,
            lexem: "\"hello world\"".to_string(),
            literal: "hello world".to_string(),
            line: 18,
        };
        assert_eq!(token.to_string().trim(), "( \"hello world\" hello world 18")
    }
}
