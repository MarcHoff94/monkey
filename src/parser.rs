use std::collections::HashMap;

use crate::lexer::Lexer;
use crate::ast::{Programm, Statement};
use crate::token::{Token, TokenType, LetStatement, Identifier, MonkeyExpression, ReturnStatement, ExpressionStatement};

pub struct Parser<'a> {
    lexer: &'a mut Lexer,
    curr_token: Token,
    peek_token: Token,
    prefix_parse_fns: HashMap<TokenType, fn(&mut Parser<'a>) -> Result<MonkeyExpression, &'static str>>,
    infix_parse_fns: HashMap<TokenType, fn(expression: MonkeyExpression) -> Result<MonkeyExpression, &'static str>>
}
impl<'a> Parser <'a> {
    pub fn new(lexer: &'a mut Lexer) -> Parser<'a> {
        let token_1 = lexer.next_token();
        let token_2 = lexer.next_token();
        let mut p = Parser{
            lexer: lexer,
            curr_token: token_1,
            peek_token: token_2,
            prefix_parse_fns: HashMap::new(),
            infix_parse_fns: HashMap::new(),
        };
        p.register_prefix_fn(TokenType::IDENT, Parser::parse_identifier);
        p
    }
    fn register_prefix_fn(&mut self, tok_type: TokenType, parse_func: fn(&mut Parser<'a>) -> Result<MonkeyExpression, &'static str>) {
        self.prefix_parse_fns.insert(tok_type, parse_func);
    }

    fn register_infix_fn(&mut self, tok_type: TokenType, parse_func: fn(expr: MonkeyExpression) -> Result<MonkeyExpression, &'static str>) {
        self.infix_parse_fns.insert(tok_type, parse_func);
    }
    //alot of cloning going on here :/ -> needs to be fixed
    pub fn parse_programm(&mut self) -> Result<Programm, &'static str> {
        let mut programm = Programm {
            statements: Vec::new()
        };
        let mut parsed_statement: Statement;
        loop {
            
            match self.curr_token.tokentype {
                TokenType::LET => {
                    parsed_statement = match self.parse_let_statement() {
                        Ok(x) => x,
                        Err(err) => panic!("Error parsing let statement: {}",err),
                    };
                },
                TokenType::RETURN => {
                    parsed_statement = match self.parse_return_statement() {
                        Ok(x) => x,
                        Err(err) => panic!("Error parsing return statment: {}", err),
                    }
                }
                TokenType::EOF => break,
                _ => {
                    parsed_statement = match self.parse_expression_statement() {
                        Ok(x) => x,
                        Err(err) => panic!("Error parsing return statment: {}", err)
                    }
                },
            }

            println!("{:#?}", parsed_statement);
            programm.statements.push(parsed_statement);
            let _ = &self.next_token();
        }
        if programm.statements.len() > 0 {
            Ok(programm)
        } else {
            Err("failed to parse any statements")
        }
    }

    fn next_token(&mut self) {
        self.curr_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn parse_let_statement(&mut self) -> Result<Statement, &'static str> {
        let statement_token = self.curr_token.clone();
        let statement_name: Identifier;
        match self.peek_token.tokentype {
            TokenType::IDENT => {
                self.next_token();
                statement_name = Identifier{token: self.curr_token.clone(), value: self.curr_token.literal.clone()};
            },
            _ => return Err("the name of the variable contains illegal characters or keywords"),
        }

        match self.peek_token.tokentype {
            TokenType::ASSIGN => {self.next_token();},
            _ => return Err("for assigning values to a variable a \"=\" is required "),
            
        }

        //expression parsing still missing
        
        while self.curr_token.tokentype != TokenType::SEMICOLON {
            self.next_token();
        }

        Ok(Statement::LET(LetStatement::new(
            Token::new(statement_token.tokentype.clone(), statement_token.literal.clone()), statement_name, 
            MonkeyExpression { token: statement_token.clone(), value: statement_token.literal})))
    }

    fn parse_return_statement(&mut self) -> Result<Statement, &'static str> {
        let statement_token = self.curr_token.clone();
        //expression parsing is still missing 
        while self.curr_token.tokentype != TokenType::SEMICOLON {
            self.next_token();
        }
        Ok(Statement::RETURN(ReturnStatement::new(statement_token, MonkeyExpression { token: self.curr_token.clone(), value: self.curr_token.literal.clone() })))
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, &'static str> {
        let expression = match self.parse_expression(Precedence::LOWEST) {
            Ok(x) => x,
            Err(err) => panic!("was not able to parse expression of statement. Token: {:?}, Error: {}", self.curr_token, err),
        };
        if self.peektoken_is(TokenType::SEMICOLON) {
            self.next_token();
        }
        Ok(Statement::EXPRESSION(ExpressionStatement::new(self.curr_token.clone(), expression)))
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Result<MonkeyExpression, &'static str> {
        match self.curr_token.tokentype {
            TokenType::IDENT => self.parse_identifier(),
            _ => Err("mep")
        }
    }

    pub fn parse_identifier(&mut self) -> Result<MonkeyExpression, &'static str> {
        Ok(MonkeyExpression { token: self.curr_token.clone(), value: self.curr_token.literal.clone() })
    }

    fn expect_peek(&mut self, tok_type: TokenType) -> bool {
        if self.peektoken_is(tok_type) {
            self.next_token();
            return true
        } else {
            return false
        }
    }
    fn peektoken_is(&self, tok_type: TokenType) -> bool {
        return self.peek_token.tokentype == tok_type
    }

    fn currtoken_is(&self, tok_type: TokenType) -> bool {
        return self.curr_token.tokentype == tok_type
    }
}

pub enum Precedence {
    LOWEST,
    EQUAL,
    LESSGREATER,
    SUM,
    PRODUCT,
    PREFIX,
    CALL,
}
impl Precedence {
    pub fn into_i32(&self) -> i32 {
        match self {
            Self::LOWEST => 1,
            Self::EQUAL => 2,
            Self::LESSGREATER => 3,
            Self::SUM => 4,
            Self::PRODUCT => 5,
            Self::PREFIX => 6,
            Self::CALL => 7,
        }
    }
    pub fn from_i32(int: i32) -> Option<Precedence> {
        match int {
            1 => Some(Precedence::LOWEST),
            2 => Some(Precedence::EQUAL),
            3 => Some(Precedence::LESSGREATER),
            4 => Some(Precedence::SUM),
            5 => Some(Precedence::PRODUCT),
            6 => Some(Precedence::PREFIX),
            7 => Some(Precedence::CALL),
            _ => None
        }
    }
}