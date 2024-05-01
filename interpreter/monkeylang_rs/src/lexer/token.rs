#[derive(Debug, PartialEq)]
pub enum TokenType {
    ILLEGAL,
    EOF,

    IDENT,
    INT,

    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,
    BSLASH,

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

    pub fn new_with_char(tt: TokenType, tc: &char) -> Token {
        return Token {
            token_type: tt,
            literal: tc.to_string(),
        };
    }

    // found it difficult to implement a static `HashMap` for the `Token` impl
    // block - substituted the `HashMap` for a simple `match` statement
    pub fn lookup_ident(ident: &str) -> TokenType {
        return match ident {
            "let" => TokenType::LET,
            "fn" => TokenType::FUNCTION,
            _ => TokenType::IDENT,
        };
    }
}
