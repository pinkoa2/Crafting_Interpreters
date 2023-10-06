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
    UNKNOWN,
}

#[allow(dead_code)]
impl Token_Type {
    pub fn identify_token(token: &char) -> Option<Token_Type> {
        match token {
            '(' => Some(Token_Type::LEFT_PAREN),
            ')' => Some(Token_Type::RIGHT_PAREN),
            '{' => Some(Token_Type::LEFT_BRACE),
            '}' => Some(Token_Type::RIGHT_BRACE),
            ',' => Some(Token_Type::COMMA),
            '.' => Some(Token_Type::DOT),
            '-' => Some(Token_Type::MINUS),
            '+' => Some(Token_Type::PLUS),
            ';' => Some(Token_Type::SEMICOLON),
            '*' => Some(Token_Type::STAR),
            '!' => Some(Token_Type::BANG),
            '=' => Some(Token_Type::EQUAL),
            '>' => Some(Token_Type::GREATER),
            '<' => Some(Token_Type::LESS),
            // TODO: add the remaining
            _ => None,
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
            Token_Type::BANG => write!(f, "!"),
            Token_Type::BANG_EQUAL => write!(f, "!="),
            Token_Type::EQUAL => write!(f, "="),
            Token_Type::EQUAL_EQUAL => write!(f, "=="),
            Token_Type::GREATER => write!(f, ">"),
            Token_Type::GREATER_EQUAL => write!(f, ">="),
            Token_Type::LESS => write!(f, "<"),
            Token_Type::LESS_EQUAL => write!(f, "<="),
            // TODO: add the remaining
            _ => write!(f, "UNKNOWN"),
        }
    }
}
