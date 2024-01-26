
use crate::ast::Expression;
use crate::ast::MonkeyExpression;
use crate::ast::Node;


#[derive(Debug, Clone)]
pub struct Token{
    pub tokentype: TokenType,
    pub literal: String,
}
impl Token {
    pub fn new(tok_type: TokenType, literal: String) -> Token {
        Token {tokentype: tok_type, literal: literal}
    }
}
#[derive(Debug, PartialEq, Clone, Eq, Hash)]
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
#[derive(Debug)]
pub struct LetStatement {
    token: Token,
    name: Identifier,
    value: MonkeyExpression,

}
impl LetStatement {
    pub fn new(tok: Token, name: Identifier, value: MonkeyExpression) -> LetStatement {
        LetStatement{token: tok, name: name, value: value}
    }
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

#[derive(Debug)]
pub struct ReturnStatement {
    token: Token,
    return_value: MonkeyExpression,
}
impl ReturnStatement {
    pub fn new(tok: Token, expr: MonkeyExpression) -> ReturnStatement {
        ReturnStatement {token: tok, return_value: expr}
    }
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> Option<&String> {
        if &self.token.literal != "" {
            Some(&self.token.literal)
        } else {
            None   
        }
    }
}


#[derive(Debug)]
pub struct ExpressionStatement {
    token: Token,
    expression: MonkeyExpression
}

impl ExpressionStatement {
    pub fn new(tok:Token, expr: MonkeyExpression) -> ExpressionStatement {
        ExpressionStatement { token: tok, expression: expr }
    }
}
impl Node for ExpressionStatement {
    fn token_literal(&self) -> Option<&String> {
        if &self.token.literal != "" {
            Some(&self.token.literal)
        } else {
            None
        }
    }
}
impl Expression for ExpressionStatement {
    fn expression_node(&self) {
        
    }
}

#[derive(Debug)]
pub struct Identifier {
    pub token: Token,
    pub value: String, 
}
impl Identifier {
    pub fn new(tok: Token, val: String) -> Identifier {
        Identifier { token: tok, value: val }
    }
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

#[derive(Debug)]
pub struct IntegerLiteral {
    pub token: Token,
    pub value: i64,
}
impl IntegerLiteral {
    pub fn new(tok: Token, val: i64) -> IntegerLiteral {
        IntegerLiteral { token: tok, value: val }
    }
}
impl Node for IntegerLiteral {
    fn token_literal(&self) -> Option<&String> {
        if &self.token.literal != "" {
            Some(&self.token.literal)
        } else {
            None
        }
    }
}
impl Expression for IntegerLiteral {
    fn expression_node(&self) {
        
    }
}

