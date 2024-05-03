#[derive(Debug, PartialEq)]
pub enum TokenType {
    ILLEGAL,
    EOF,
    EOL,

    IDENT,
    INT,

    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,
    BSLASH,

    EQUAL,
    NEQUAL,

    LTHAN,
    RTHAN,

    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(tt: TokenType, tl: String) -> Token {
        return Token {
            token_type: tt,
            literal: tl,
        };
    }

    pub fn new_with_ref(tt: TokenType, tl: &String) -> Token {
        return Self::new(tt, String::from(tl));
    }

    pub fn new_with_char(tt: TokenType, tc: &char) -> Token {
        return Token {
            token_type: tt,
            literal: String::from(*tc),
        };
    }

    // found it difficult to implement a static `HashMap` for the `Token` impl
    // block - substituted the `HashMap` for a simple `match` statement
    pub fn lookup_ident(ident: &str) -> TokenType {
        return match ident {
            "let" => TokenType::LET,
            "fn" => TokenType::FUNCTION,
            "true" => TokenType::TRUE,
            "false" => TokenType::FALSE,
            "if" => TokenType::IF,
            "else" => TokenType::ELSE,
            "return" => TokenType::RETURN,
            _ => TokenType::IDENT,
        };
    }
}
