use std::collections::HashMap;

use crate::lexer::Lexer;
use crate::ast::{MonkeyExpr, MonkeyExpression, Programm, Statement};
use crate::token::{BlockStatement, Boolean, CallExpression, ExpressionStatement, FunctionLiteral, Identifier, IfExpression, InfixExpression, IntegerLiteral, LetStatement, PrefixExpression, ReturnStatement, Token, TokenType};

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
        p.register_prefix_fn(TokenType::FALSE, Parser::parse_boolean);
        p.register_prefix_fn(TokenType::TRUE, Parser::parse_boolean);
        p.register_prefix_fn(TokenType::LPAREN, Parser::parse_grouped_expression);
        p.register_prefix_fn(TokenType::IF, Parser::parse_if_expression);
        p.register_prefix_fn(TokenType::FUNCTION, Parser::parse_function_literal);

        p.register_infix_fn(TokenType::LPAREN, Parser::parse_call_expression);
        p.register_infix_fn(TokenType::EQ, Parser::parse_infix_expression);
        p.register_infix_fn(TokenType::NOTEQ, Parser::parse_infix_expression);
        p.register_infix_fn(TokenType::LT, Parser::parse_infix_expression);
        p.register_infix_fn(TokenType::LTEQ, Parser::parse_infix_expression);
        p.register_infix_fn(TokenType::GT, Parser::parse_infix_expression);
        p.register_infix_fn(TokenType::GTEQ, Parser::parse_infix_expression);
        p.register_infix_fn(TokenType::PLUS, Parser::parse_infix_expression);
        p.register_infix_fn(TokenType::MINUS, Parser::parse_infix_expression);
        p.register_infix_fn(TokenType::ASTERISK, Parser::parse_infix_expression);
        p.register_infix_fn(TokenType::SLASH, Parser::parse_infix_expression);
        p.register_infix_fn(TokenType::POWER, Parser::parse_infix_expression);

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
            
            parsed_statement = match self.parse_statement() {
                Some(x) => x,
                None => break,
            };

            // println!("{:#?}", parsed_statement);
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

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.curr_token.tokentype {
            TokenType::LET => {
                match self.parse_let_statement() {
                    Ok(x) => Some(x),
                    Err(err) => panic!("Error parsing let statement: {}",err),
                }
            },
            TokenType::RETURN => {
                match self.parse_return_statement() {
                    Ok(x) => Some(x),
                    Err(err) => panic!("Error parsing return statment: {}", err),
                }
            }
            TokenType::EOF => None,
            _ => {
                match self.parse_expression_statement() {
                    Ok(x) => Some(x),
                    Err(err) => panic!("Error parsing return statment: {}", err)
                }
            },
        }
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
        let value = self.parse_expression(Precedence::LOWEST.into_i32());

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
        let expression = match self.parse_expression(Precedence::LOWEST.into_i32()) {
            Ok(x) =>x,
            Err(err) => panic!("was not able to parse expression of return statement. Token: {:?}, Error: {}", self.curr_token, err),
        };
        
        while self.curr_token.tokentype != TokenType::SEMICOLON {
            self.next_token();
        }
        return Ok(Statement::RETURN(ReturnStatement::new(statement_token, expression)))
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, &'static str> {
        let expression = match self.parse_expression(Precedence::LOWEST.into_i32()) {
            Ok(x) => x,
            Err(err) => panic!("was not able to parse expression of expression statement. Token: {:?}, Error: {}", self.curr_token, err),
        };
        if self.peektoken_is(TokenType::SEMICOLON) {
            self.next_token();
        }
        Ok(Statement::EXPRESSION(ExpressionStatement::new(self.curr_token.clone(), expression)))
    }

    fn parse_block_statement(&mut self) -> Result<BlockStatement, &'static str> {
        let mut block_statement = BlockStatement::new(self.curr_token.clone(), Vec::new());
        self.next_token();
        while !self.currtoken_is(TokenType::RBRACE) && !self.currtoken_is(TokenType::EOF) {
            let statement = match self.parse_statement() {
                Some(x) => x,
                None => break,
            };
            block_statement.statements.push(statement);
            self.next_token();
        }
        Ok(block_statement)
    }

    fn parse_expression(&mut self, precedence: i32) -> Result<MonkeyExpression, &'static str> {
        let mut left_expr: Result<MonkeyExpression, &'static str>;
        
        //eeh sketchy, im not checking if curr_token.tokentype is a key in hashmap
        let prefix = self.prefix_parse_fns[&self.curr_token.tokentype];

        left_expr = prefix(self);

        while self.peek_token.tokentype != TokenType::SEMICOLON && precedence < self.get_precedence(true).into_i32() {
            let infix = self.infix_parse_fns[&self.peek_token.tokentype];
            self.next_token();
            left_expr = infix(self, left_expr.unwrap());
        }
        left_expr
    
    }

    fn parse_grouped_expression(&mut self) -> Result<MonkeyExpression, &'static str> {
        self.next_token();
        let expression = self.parse_expression(Precedence::LOWEST.into_i32());
        if !self.expect_peek(TokenType::RPAREN) {
            return Err("Error during parsing grouped expression. Did not find closing )")
        }

        expression

    }

    fn parse_function_literal(&mut self) -> Result<MonkeyExpression, &'static str> {
        let func_tok = self.curr_token.clone();

        if !self.expect_peek(TokenType::LPAREN) {
            panic!("Error parsing function: fn needs to be followed up with (). curr_token: {:#?}", self.curr_token)
        }

        let params = self.parse_function_parameters().unwrap();
    
        if !self.expect_peek(TokenType::LBRACE) {
            panic!("Error parsing function: missing opening bracket \"{{\" for blockstatement curr_tocken: {:#?}", self.curr_token)
        }

        let blockstatement = self.parse_block_statement().unwrap();

        Ok(MonkeyExpression::FUNCTIONLITERAL(
            FunctionLiteral::new(func_tok, params, blockstatement)
        ))
    }

    fn parse_function_parameters(&mut self) -> Result<Vec<Identifier>, &'static str> {
        let mut identifiers: Vec<Identifier> = Vec::new();
        if self.peektoken_is(TokenType::LPAREN) {
            self.next_token();
            return Ok(identifiers)
        }
        self.next_token();

        let mut ident = Identifier::new(self.curr_token.clone(), self.curr_token.literal.clone());
        identifiers.push(ident);

        while self.peektoken_is(TokenType::COMMA) {
            self.next_token();
            self.next_token();

            ident = Identifier::new(self.curr_token.clone(), self.curr_token.literal.clone());
            identifiers.push(ident);
        }

        if !self.expect_peek(TokenType::RPAREN) {
            panic!("Error parsing function parameters: missing ). curr_token: {:#?}", self.curr_token)
        }
        Ok(identifiers)
    }

    fn parse_if_expression(&mut self) -> Result<MonkeyExpression, &'static str> {
        let if_token = self.curr_token.clone();
        if !self.expect_peek(TokenType::LPAREN) {
           panic!("Opening braces missing. condition of If-expression needs to be in brackets => (<condition>)")
        }
        self.next_token();
        let condition = self.parse_expression(Precedence::LOWEST.into_i32()).unwrap();
        
        if !self.expect_peek(TokenType::RPAREN) {
            panic!("Closing braces missing. condition of If-expression needs to be in brackets => (<condition>)")
        }
        if !self.expect_peek(TokenType::LBRACE) {
            panic!("\"{{\" missing. the following blockstatement needs to be look like this => {{<BlockStatement>}}")
        }
        let consequence = self.parse_block_statement().unwrap();
        let alternative: Option<BlockStatement>;
        if self.peektoken_is(TokenType::ELSE) {
            self.next_token();
            if !self.expect_peek(TokenType::LBRACE) {
                panic!("\"{{\" missing. the following blockstatement needs to be look like this => {{<BlockStatement>}}")
            }
            alternative = Some(self.parse_block_statement().unwrap());
        } else {
            alternative = None;
        }

        Ok(MonkeyExpression::IF(
            IfExpression::new(if_token, condition.into_expr(), consequence, alternative)
        ))
        
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

    fn parse_boolean(&mut self) -> Result<MonkeyExpression, &'static str> {
        Ok(
            MonkeyExpression::BOOLEAN(
                Boolean::new(self.curr_token.clone(), self.currtoken_is(TokenType::TRUE))
            )
        )
        
    }

    fn parse_prefix_expression(&mut self) -> Result<MonkeyExpression, &'static str> {
        let tok = self.curr_token.clone();
        let op = self.curr_token.literal.clone();
        self.next_token();
        let right = self.parse_expression(Precedence::PREFIX.into_i32()).unwrap();
        Ok(MonkeyExpression::PREFIX(PrefixExpression::new(tok, op, right)))

    }

    fn parse_infix_expression(&mut self, left: MonkeyExpression) -> Result<MonkeyExpression, &'static str> {
        let token = self.curr_token.clone();
        let precedence = self.get_precedence(false).into_i32();
        self.next_token();
        Ok(MonkeyExpression::INFIX(
            InfixExpression::new(
                token.literal.clone(),
                token,
                left,
                self.parse_expression(precedence).unwrap()
            )
        ))
    }

    fn parse_call_expression(&mut self, function: MonkeyExpression) -> Result<MonkeyExpression, &'static str> {
        let tok = self.curr_token.clone();
        let args = self.parse_call_arguments();
        Ok(MonkeyExpression::CALL(CallExpression::new(tok, function, args)))
    }
    fn parse_call_arguments(&mut self) -> Option<Vec<Box<dyn MonkeyExpr>>> {

        if self.peektoken_is(TokenType::RPAREN) {
            self.next_token();
            return None
        }

        let mut args : Vec<Box<dyn MonkeyExpr>> = Vec::new();
        self.next_token();
        args.push(self.parse_expression(Precedence::LOWEST.into_i32()).unwrap().into_expr());

        while self.peektoken_is(TokenType::COMMA) {
            self.next_token();
            self.next_token();
            args.push(self.parse_expression(Precedence::LOWEST.into_i32()).unwrap().into_expr())
        }
        
        if !self.expect_peek(TokenType::RPAREN) {
            panic!("Error parsing arguments of function call: missing closing ). curr_token: {:#?}", self.curr_token)
        }
        Some(args)
    }
    fn get_precedence(&self, peek: bool) -> Precedence {
        let token = if peek {&self.peek_token} else {&self.curr_token};
        match token.tokentype {
            TokenType::EQ | TokenType::NOTEQ => Precedence::EQUAL,
            TokenType::GT | TokenType::LT => Precedence::LESSGREATER,
            TokenType::GTEQ | TokenType::LTEQ => Precedence::LESSGREATER,
            TokenType::PLUS | TokenType::MINUS => Precedence::SUM,
            TokenType::ASTERISK | TokenType::SLASH => Precedence::PRODUCT,
            TokenType::POWER => Precedence::POWER,
            TokenType::LPAREN => Precedence::CALL,
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
    POWER,
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
            Self::POWER => 6,
            Self::PREFIX => 7,
            Self::CALL => 8,
        }
    }
    pub fn from_i32(int: i32) -> Option<Precedence> {
        match int {
            1 => Some(Precedence::LOWEST),
            2 => Some(Precedence::EQUAL),
            3 => Some(Precedence::LESSGREATER),
            4 => Some(Precedence::SUM),
            5 => Some(Precedence::PRODUCT),
            6 => Some(Precedence::POWER),
            7 => Some(Precedence::PREFIX),
            8 => Some(Precedence::CALL),
            _ => None
        }
    }
}