use std::collections::HashMap;

use crate::lexer::Lexer;
use crate::ast::{Programm, Statement, MonkeyExpression, MonkeyExpr};
use crate::token::{ExpressionStatement, Identifier, InfixExpression, IntegerLiteral, LetStatement, PrefixExpression, ReturnStatement, Token, TokenType};

pub struct Parser<'a> {
    lexer: &'a mut Lexer,
    curr_token: Token,
    peek_token: Token,
    prefix_parse_fns: HashMap<TokenType, fn(&mut Parser<'a>) -> Result<MonkeyExpression, &'static str>>,
    infix_parse_fns: HashMap<TokenType, fn(&mut Parser<'a>, left : MonkeyExpression) -> Result<MonkeyExpression, &'static str>>
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
        p.register_prefix_fn(TokenType::INT, Parser::parse_integer_literal);
        p.register_prefix_fn(TokenType::BANG, Parser::parse_prefix_expression);
        p.register_prefix_fn(TokenType::MINUS, Parser::parse_prefix_expression);

        p.register_infix_fn(TokenType::EQ, Parser::parse_infix_expression);
        p.register_infix_fn(TokenType::NOTEQ, Parser::parse_infix_expression);
        p.register_infix_fn(TokenType::LT, Parser::parse_infix_expression);
        p.register_infix_fn(TokenType::GT, Parser::parse_infix_expression);
        p.register_infix_fn(TokenType::PLUS, Parser::parse_infix_expression);
        p.register_infix_fn(TokenType::MINUS, Parser::parse_infix_expression);
        p.register_infix_fn(TokenType::ASTERISK, Parser::parse_infix_expression);
        p.register_infix_fn(TokenType::SLASH, Parser::parse_infix_expression);

        p
    }
    fn register_prefix_fn(&mut self, tok_type: TokenType, parse_func: fn(&mut Parser<'a>) -> Result<MonkeyExpression, &'static str>) {
        self.prefix_parse_fns.insert(tok_type, parse_func);
    }

    fn register_infix_fn(&mut self, tok_type: TokenType, parse_func: fn(&mut Parser<'a>, expr: MonkeyExpression) -> Result<MonkeyExpression, &'static str>) {
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
                statement_name = Identifier::new(self.curr_token.clone(), self.curr_token.literal.clone());
            },
            _ => return Err("the name of the variable contains illegal characters or keywords"),
        }

        match self.peek_token.tokentype {
            TokenType::ASSIGN => {self.next_token(); self.next_token();},
            _ => return Err("for assigning values to a variable a \"=\" is required "),
            
        }
        let value = self.parse_expression(Precedence::LOWEST);
        // let value = match self.curr_token.tokentype {
        //     TokenType::INT => self.parse_integer_literal(),
        //     TokenType::MINUS => self.parse_prefix_expression(),
        //     _ => Ok(MonkeyExpression::INTEGERLITERAL(IntegerLiteral::new(self.curr_token.clone(), 0)))
        // };
        self.next_token();
        
        while self.curr_token.tokentype != TokenType::SEMICOLON {
            self.next_token();
        }

        Ok(Statement::LET(
            LetStatement::new(
                Token::new(statement_token.tokentype, statement_token.literal), 
                statement_name,
                value.unwrap())
            )
        )
    }

    fn parse_return_statement(&mut self) -> Result<Statement, &'static str> {
        let statement_token = self.curr_token.clone();
        self.next_token();
        let expression = match self.parse_expression(Precedence::LOWEST) {
            Ok(x) =>x,
            Err(err) => panic!("was not able to parse expression of return statement. Token: {:?}, Error: {}", self.curr_token, err),
        };
        
        while self.curr_token.tokentype != TokenType::SEMICOLON {
            self.next_token();
        }
        return Ok(Statement::RETURN(ReturnStatement::new(statement_token, expression)))
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, &'static str> {
        let expression = match self.parse_expression(Precedence::LOWEST) {
            Ok(x) => x,
            Err(err) => panic!("was not able to parse expression of expression statement. Token: {:?}, Error: {}", self.curr_token, err),
        };
        if self.peektoken_is(TokenType::SEMICOLON) {
            self.next_token();
        }
        Ok(Statement::EXPRESSION(ExpressionStatement::new(self.curr_token.clone(), expression)))
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Result<MonkeyExpression, &'static str> {
        let mut left_expr: Result<MonkeyExpression, &'static str>;
        let prefix_tok = self.curr_token.clone();
        
        let prefix = match self.prefix_parse_fns.get(&prefix_tok.tokentype) {
            Some(func) => func,
            None => panic!("couldnt find prefix parsing function. Token: {:?}", prefix_tok),
        };

        left_expr = prefix(self);
        println!("{:#?}", left_expr);
        while self.peek_token.tokentype != TokenType::SEMICOLON && precedence.into_i32() < self.get_precedence(true).into_i32() {
            let infix = match self.infix_parse_fns.get(&self.peek_token.tokentype) {
                Some(func) => func,
                None => panic!("couldnt find infix parsing function. Token: {:?}", self.curr_token),
            };
            self.next_token();
            left_expr = infix(self, left_expr.unwrap());
            println!("{:#?}", left_expr);
        }
        left_expr
        // match self.curr_token.tokentype {
        //     TokenType::IDENT => self.parse_identifier(),
        //     TokenType::INT => self.parse_integer_literal(),
        //     TokenType::BANG | TokenType::MINUS => self.parse_prefix_expression(),
        //     _ => Err("mep")
        // }
    }

    fn parse_identifier(&mut self) -> Result<MonkeyExpression, &'static str> {
        Ok(
            MonkeyExpression::IDENT(
                Identifier::new(self.curr_token.clone(), self.curr_token.literal.clone())
            )
        )
    }

    fn parse_integer_literal(&mut self) -> Result<MonkeyExpression, &'static str> {
        let int_val = match self.curr_token.literal.parse::<i64>() {
            Ok(x) => x,
            Err(err) => panic!("Parse Error! could not parse integer value: {}", err),
        };
        Ok(
            MonkeyExpression::INTEGERLITERAL(
                IntegerLiteral::new(self.curr_token.clone(), int_val)
            )
        )
    }

    fn parse_prefix_expression(&mut self) -> Result<MonkeyExpression, &'static str> {
        let tok = self.curr_token.clone();
        let op = self.curr_token.literal.clone();
        self.next_token();
        let right = self.parse_expression(Precedence::PREFIX).unwrap();
        Ok(MonkeyExpression::PREFIX(PrefixExpression::new(tok, op, right)))

    }

    fn parse_infix_expression(&mut self, left: MonkeyExpression) -> Result<MonkeyExpression, &'static str> {
        let token = self.curr_token.clone();
        let precedence = self.get_precedence(false);
        self.next_token();
        Ok(MonkeyExpression::INFIX(
            InfixExpression::new(
                token.literal.clone(),
                token,
                left.into_expr(),
                self.parse_expression(precedence).unwrap().into_expr()
            )   
        ))
    }

    fn get_precedence(&self, peek: bool) -> Precedence {
        let token = if peek {&self.peek_token} else {&self.curr_token};
        match token.tokentype {
            TokenType::EQ | TokenType::NOTEQ => Precedence::EQUAL,
            TokenType::GT | TokenType::LT => Precedence::LESSGREATER,
            TokenType::PLUS | TokenType::MINUS => Precedence::SUM,
            TokenType::ASTERISK | TokenType::SLASH => Precedence::PRODUCT,
            _ => Precedence::LOWEST
        }
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