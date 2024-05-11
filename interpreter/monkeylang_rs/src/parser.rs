use crate::{
    ast,
    lexer::{
        self,
        token::{Token, TokenType},
        Lexer,
    },
    parser,
};

pub struct Parser<'a> {
    lexer: Lexer<'a>,

    curr_token: Option<Token>,
    peek_token: Option<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(l: Lexer) -> Parser {
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
        self.curr_token = self.peek_token;
        self.peek_token = Some(self.lexer.next_token());
    }
}
