use crate::{
    ast,
    lexer::{self, Lexer},
    parser,
    token::{Token, TokenType},
};
use std::mem;

pub struct Parser<'a> {
    lexer: &'a mut Lexer<'a>,
    curr_token: Option<Token>,
    peek_token: Option<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(l: &'a mut Lexer<'a>) -> Parser<'a> {
        let mut p = Parser {
            lexer: l,
            curr_token: None,
            peek_token: None,
        };

        // read tokens to get parser to starting position
        p.next_token();

        p.next_token();

        return p;
    }

    pub fn next_token(&mut self) -> () {
        self.curr_token = mem::replace(&mut self.peek_token, Some(self.lexer.next_token()));
    }
}
