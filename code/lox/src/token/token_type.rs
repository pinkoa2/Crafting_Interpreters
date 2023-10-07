use core::fmt;

#[allow(non_camel_case_types)]
#[allow(dead_code)]
#[derive(PartialEq, Eq)]
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
            Token_Type::BANG => write!(f, "!"),
            Token_Type::BANG_EQUAL => write!(f, "!="),
            Token_Type::EQUAL => write!(f, "="),
            Token_Type::EQUAL_EQUAL => write!(f, "=="),
            Token_Type::GREATER => write!(f, ">"),
            Token_Type::GREATER_EQUAL => write!(f, ">="),
            Token_Type::LESS => write!(f, "<"),
            Token_Type::LESS_EQUAL => write!(f, "<="),
            Token_Type::STRING => write!(f, "STRING"),
            Token_Type::EOF => write!(f, "EOF"),
            // TODO: add the remaining
            _ => write!(f, "UNKNOWN"),
        }
    }
}
