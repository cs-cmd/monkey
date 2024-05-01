pub mod token;

use core::str::Chars;
use token::{Token, TokenType};

pub struct Lexer<'a> {
    input: &'a str,
    chars: Chars<'a>,
    line_number: usize,
    position: i32,
    ch: char,
}

enum StringType {
    IDENTIFIER,
    NUMBER,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &str, line_number: usize) -> Lexer {
        let mut l = Lexer {
            input,
            chars: input.chars(),
            line_number,
            position: 0,
            ch: '\0',
        };

        l.read_char();

        return l;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.ch {
            '=' => Token::new_with_char(TokenType::ASSIGN, &self.ch),
            ';' => Token::new_with_char(TokenType::SEMICOLON, &self.ch),
            '(' => Token::new_with_char(TokenType::LPAREN, &self.ch),
            ')' => Token::new_with_char(TokenType::RPAREN, &self.ch),
            ',' => Token::new_with_char(TokenType::COMMA, &self.ch),
            '+' => Token::new_with_char(TokenType::PLUS, &self.ch),
            '{' => Token::new_with_char(TokenType::LBRACE, &self.ch),
            '}' => Token::new_with_char(TokenType::RBRACE, &self.ch),
            '-' => Token::new_with_char(TokenType::MINUS, &self.ch),
            '!' => Token::new_with_char(TokenType::BANG, &self.ch),
            '*' => Token::new_with_char(TokenType::ASTERISK, &self.ch),
            '/' => Token::new_with_char(TokenType::SLASH, &self.ch),
            '\\' => Token::new_with_char(TokenType::BSLASH, &self.ch),
            '<' => Token::new_with_char(TokenType::LTHAN, &self.ch),
            '>' => Token::new_with_char(TokenType::RTHAN, &self.ch),
            _ => {
                if Self::is_letter(&self.ch) {
                    let lit = self.read(StringType::IDENTIFIER);
                    let token_type = Token::lookup_ident(&lit);
                    return Token::new(token_type, lit);
                } else if Self::is_number(&self.ch) {
                    let lit = self.read(StringType::NUMBER);
                    return Token::new(TokenType::INT, lit);
                } else {
                    Token::new(TokenType::ILLEGAL, "".to_string())
                }
            }
        };

        self.read_char();

        return token;
    }

    fn read_char(&mut self) -> () {
        self.ch = self.chars.next().unwrap_or('\0');
        self.position += 1;
    }

    fn read(&mut self, t: StringType) -> String {
        let mut s = String::new();

        let comp = match t {
            StringType::IDENTIFIER => Self::is_letter,
            StringType::NUMBER => Self::is_number,
        };

        while comp(&self.ch) {
            s.push(self.ch);
            self.read_char();
        }

        return s;
    }

    fn skip_whitespace(&mut self) -> () {
        while Self::is_whitespace(&self.ch) {
            self.read_char();
        }
    }

    fn is_letter(c: &char) -> bool {
        return match c {
            'a'..='z' | 'A'..='Z' | '_' => true,
            _ => false,
        };
    }

    fn is_number(c: &char) -> bool {
        return match c {
            '0'..='9' => true,
            _ => false,
        };
    }

    fn is_whitespace(c: &char) -> bool {
        return match c {
            ' ' | '\t' | '\n' | '\r' => true,
            _ => false,
        };
    }
}
