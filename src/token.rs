use crate::ast::Expression;
use crate::ast::Statement;
use crate::ast::Node;


#[derive(Debug)]
pub struct Token{
    pub tokentype: TokenType,
    pub literal: String,
}
impl Token {
    pub fn new(tok_type: TokenType, literal: String) -> Token {
        Token {
            tokentype: tok_type,
            literal: literal,
        }
    }
}
#[derive(Debug)]
pub enum TokenType {
    ILLEGAL,
    EOF,

    //Identifiers + literals
    IDENT,
    INT,

    //Operators
    ASSIGN,
    PLUS,
    PLUSSELF,
    MINUS,
    MINUSSELF,
    POWER,
    BANG,
    ASTERISK,
    SLASH,

    LT,
    LTEQ,
    GT,
    GTEQ,

    EQ,
    NOTEQ,

    //Delimiters

    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    //Keywords

    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
}
impl TokenType {
    pub fn lookup_keyword(keyword: &str) -> TokenType {
        let result = match keyword {
            "function" => TokenType::FUNCTION,
            "let" => TokenType::LET,
            "true" => TokenType::TRUE,
            "false" => TokenType::FALSE,
            "if" => TokenType::IF,
            "else" => TokenType::ELSE,
            "return" => TokenType::RETURN,
            _ => TokenType::IDENT,
        };
        return result
    }
}

pub struct LetStatement {
    token: Token,
    name: Identifier,
    value: MonkeyExperssion,

}

impl Node for LetStatement {
    fn token_literal(&self) -> Option<&String> {
        if &self.token.literal != "" {
            Some(&self.token.literal)
        } else {
            None
        }
    }
}
impl Statement for LetStatement {
    fn statement_node(&self) {

    }
}

pub struct Identifier {
    token: Token,
    value: String, 
}
impl Node for Identifier {
    fn token_literal(&self) -> Option<&String> {
        if &self.token.literal != "" {
            Some(&self.token.literal)
        } else {
            None
        }
    } 
}
impl Expression for Identifier {
    fn expression_node(&self) {
        
    }
}

pub struct MonkeyExperssion {

}
impl Node for MonkeyExperssion {
    fn token_literal(&self) -> Option<&String> {
        None
    }
}
impl  Expression for MonkeyExperssion {
    fn expression_node(&self) {
    }
}