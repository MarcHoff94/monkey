use crate::lexer::Lexer;
use crate::ast::{self, Programm};
use crate::token::Token;

pub struct Parser<'a> {
    lexer: &'a mut Lexer,
    curr_token: Token,
    peek_token: Token,
}
impl Parser <'_> {
    pub fn new(lexer: &mut Lexer) -> Parser {
        let token_1 = lexer.next_token();
        let token_2 = lexer.next_token();
        Parser{
            lexer: lexer,
            curr_token: token_1,
            peek_token: token_2,
        }

    }

    pub fn next_token(mut self) {
        self.curr_token = self.peek_token;
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_programm(&mut self) -> Result<Programm, &'static str> {
        Err("not implemented yet")
    }
}