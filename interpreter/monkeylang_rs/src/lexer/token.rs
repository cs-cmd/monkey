//! # Token
//!
//! `token.rs` contains types and associated functions needed to represent
//! tokens in the Monkey programming language

/// Represents the type of token
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

/// The actual token type, containing the type and the string literal
#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    /// basic token declaration with type and string literal
    ///
    /// # Example:
    ///
    /// ```
    /// use monkeylang_rs::lexer::token::{TokenType, Token};
    /// let tt = TokenType::INT;
    /// let val_str = "15";
    /// let t = Token::new(tt, val_str.to_owned());
    /// ```
    pub fn new(tt: TokenType, tl: String) -> Token {
        return Token {
            token_type: tt,
            literal: tl,
        };
    }

    /// New token with a string reference. Useful if moving will happen
    /// later in function
    /// Creates a new string reference based on given reference
    ///
    /// # Example:
    ///
    /// ```
    /// use monkeylang_rs::lexer::token::{Token, TokenType};
    /// let tt = TokenType::BANG;
    /// let s_ref = String::from("!");
    /// let t_ref = Token::new_with_ref(tt, &s_ref);
    /// println!("{}", s_ref);
    /// ```
    pub fn new_with_ref(tt: TokenType, tl: &String) -> Token {
        return Self::new(tt, String::from(tl));
    }

    /// New token from character. Used for single-character tokens
    ///
    /// # Example:
    ///
    /// ```
    /// use monkeylang_rs::lexer::token::{Token, TokenType};
    /// let c = '=';
    /// let tt = TokenType::ASSIGN;
    /// let t = Token::new_with_char(tt, c);
    /// ```
    pub fn new_with_char(tt: TokenType, tc: char) -> Token {
        return Token {
            token_type: tt,
            literal: String::from(tc),
        };
    }

    /// Looks up identifiers based on string and returns a TokenType.
    /// Returns `TokenType::IDENT` if the
    ///
    /// # Example:
    ///
    /// ```
    /// use monkeylang_rs::lexer::token::{Token, TokenType};
    /// let s_1 = Token::lookup_keyword("let");
    /// assert_eq!(TokenType::LET, s_1);
    /// let s_2 = Token::lookup_keyword("easdadsf");
    /// assert_eq!(TokenType::IDENT, s_2);
    /// ```
    /// # Comments
    /// 1. During development, I initially wanted to use a hashmap. Using
    /// a static hashmap as an associative variable proved difficult, so
    /// I opted for a function that uses a match statement
    pub fn lookup_keyword(keyword: &str) -> TokenType {
        return match keyword {
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

    /// Looks up multicharacter operators based on a match statement. Returns
    /// an `Option`.
    ///
    /// # Example
    ///
    /// ```
    /// use monkeylang_rs::lexer::token::{Token, TokenType};
    /// let s_1 = Token::lookup_operator("!=");
    /// assert_eq!(Some(TokenType::NEQUAL), s_1);
    /// let s_2 = Token::lookup_operator("==");
    /// assert_eq!(Some(TokenType::EQUAL), s_2);
    /// let s_3 = Token::lookup_operator("sss");
    /// assert_eq!(None, s_3);
    /// ```
    pub fn lookup_operator(operator: &str) -> Option<TokenType> {
        return match operator {
            "!=" => Some(TokenType::NEQUAL),
            "==" => Some(TokenType::EQUAL),
            _ => None,
        };
    }
}
