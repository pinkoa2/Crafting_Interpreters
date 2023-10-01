use core::fmt;

#[allow(non_camel_case_types)]
#[allow(dead_code)]
pub enum Token_Type {
    // Single-character tokens.
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
}

#[allow(dead_code)]
impl Token_Type {
    pub fn identify_token(token: String) -> Token_Type {
        match token.trim() {
            "(" => Token_Type::LEFT_PAREN,
            ")" => Token_Type::RIGHT_PAREN,
            "{" => Token_Type::LEFT_BRACE,
            "}" => Token_Type::RIGHT_BRACE,
            "," => Token_Type::COMMA,
            "." => Token_Type::DOT,
            "-" => Token_Type::MINUS,
            "+" => Token_Type::PLUS,
            ";" => Token_Type::SEMICOLON,
            "*" => Token_Type::STAR,
            // TODO: add the remaining
            _ => panic!("Unidentified token given"),
        }
    }
}

impl fmt::Display for Token_Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // Single-character tokens
            Token_Type::LEFT_PAREN => write!(f, "("),
            Token_Type::RIGHT_PAREN => write!(f, ")"),
            Token_Type::LEFT_BRACE => write!(f, "{{"),
            Token_Type::RIGHT_BRACE => write!(f, "}}"),
            Token_Type::COMMA => write!(f, ","),
            Token_Type::DOT => write!(f, "."),
            Token_Type::MINUS => write!(f, "-"),
            Token_Type::PLUS => write!(f, "+"),
            Token_Type::SEMICOLON => write!(f, ";"),
            Token_Type::SLASH => write!(f, "/"),
            Token_Type::STAR => write!(f, "*"),
            // One or two character tokens
            Token_Type::BANG => write!(f, "!"),
            _ => write!(f, "UNKNOWN"),
        }
    }
}
