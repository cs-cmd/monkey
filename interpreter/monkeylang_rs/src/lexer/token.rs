use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    ILLEGAL,
    EOF,

    IDENT,
    INT,

    ASSIGN,
    PLUS,

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
