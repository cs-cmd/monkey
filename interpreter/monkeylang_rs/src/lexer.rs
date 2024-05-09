use core::{iter::Peekable, str::Chars};
use token::{Token, TokenType};

// TODO: Add documentation to this file

pub mod token;

#[derive(Debug)]
pub struct Lexer<'a> {
    input: &'a str,
    chars: Peekable<Chars<'a>>,
    line_number: usize,
    position: i32,
    ch: char,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &str) -> Lexer {
        let mut l = Lexer {
            input,
            chars: input.chars().peekable(),
            line_number: 1,
            position: 0,
            ch: '\0',
        };

        l.read_char();

        return l;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.ch {
            '=' => self.read_op_multichar(TokenType::ASSIGN),
            ';' => Token::new_with_char(TokenType::SEMICOLON, self.ch),
            '(' => Token::new_with_char(TokenType::LPAREN, self.ch),
            ')' => Token::new_with_char(TokenType::RPAREN, self.ch),
            ',' => Token::new_with_char(TokenType::COMMA, self.ch),
            '+' => Token::new_with_char(TokenType::PLUS, self.ch),
            '{' => Token::new_with_char(TokenType::LBRACE, self.ch),
            '}' => Token::new_with_char(TokenType::RBRACE, self.ch),
            '-' => Token::new_with_char(TokenType::MINUS, self.ch),
            '!' => self.read_op_multichar(TokenType::BANG),
            '*' => Token::new_with_char(TokenType::ASTERISK, self.ch),
            '/' => Token::new_with_char(TokenType::SLASH, self.ch),
            '\\' => Token::new_with_char(TokenType::BSLASH, self.ch),
            '<' => Token::new_with_char(TokenType::LTHAN, self.ch),
            '>' => Token::new_with_char(TokenType::RTHAN, self.ch),
            _ => {
                if Self::is_letter(&self.ch) {
                    let lit = self.read(Self::is_letter);
                    let token_type = Token::lookup_keyword(&lit);
                    return Token::new(token_type, lit);
                } else if Self::is_number(&self.ch) {
                    let lit = self.read(Self::is_number);
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

    fn read(&mut self, comp: fn(&char) -> bool) -> String {
        let mut s = String::new();

        while comp(&self.ch) {
            s.push(self.ch);
            self.read_char();
        }

        return s;
    }

    // checks next character to see if it is a valid operator
    fn read_op_multichar(&mut self, default: TokenType) -> Token {
        let mut s = String::from(self.ch);
        s.push(*self.peek_next());

        match Token::lookup_operator(&s) {
            Some(t_type) => {
                let t = Token::new(t_type, s);
                self.read_char();
                t
            }
            _ => Token::new_with_char(default, self.ch),
        }
    }

    fn peek_next(&mut self) -> &char {
        return self.chars.peek().unwrap_or(&'\0');
    }

    fn skip_whitespace(&mut self) -> () {
        while Self::is_whitespace(&self.ch) {
            if self.ch == '\n' {
                self.position = 0;
                self.line_number += 1;
            }

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
            ' ' | '\t' | '\r' | '\n' => true,
            _ => false,
        };
    }

    pub fn get_current_location(&self) -> String {
        return format!("Line {}: column: {}", self.line_number, self.position);
    }
}
