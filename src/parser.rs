use crate::lexer::Lexer;
use crate::ast::{Programm, Statement};
use crate::token::{Token, TokenType, LetStatement, Identifier, MonkeyExpression, ReturnStatement};

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
                        Err(err) => panic!("{}",err),
                    };
                },
                TokenType::RETURN => {
                    parsed_statement = match self.parse_return_statement() {
                        Ok(x) => x,
                        Err(err) => panic!("{}", err),
                    }
                }
                TokenType::EOF => break,
                _ => {let _ = &self.next_token(); continue},
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

        Ok(Statement::LET(LetStatement::new(Token::new(statement_token.tokentype, statement_token.literal), statement_name, MonkeyExpression {})))
    }

    fn parse_return_statement(&mut self) -> Result<Statement, &'static str> {
        let statement_token = self.curr_token.clone();
        //expression parsing is still missing 
        while self.curr_token.tokentype != TokenType::SEMICOLON {
            self.next_token();
        }
        Ok(Statement::RETURN(ReturnStatement::new(statement_token, MonkeyExpression {  })))
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