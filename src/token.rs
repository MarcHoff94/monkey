use crate::ast::Expression;
use crate::ast::MonkeyExpr;
use crate::ast::MonkeyExpression;
use crate::ast::MonkeyStatement;
use crate::ast::Node;
use crate::ast::Statement;
use crate::ast::NodeType;


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
        let result = match keyword.to_lowercase().as_str() {
            "fn" => TokenType::FUNCTION,
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
impl MonkeyStatement for LetStatement {}

impl Node for LetStatement {
    fn token_literal(&self) -> Option<&String> {
        if &self.token.literal != "" {
            Some(&self.token.literal)
        } else {
            None
        }
    }
    fn node_type(&self) -> NodeType {
        NodeType::STATEMENT
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
impl MonkeyStatement for ReturnStatement {}

impl Node for ReturnStatement {
    fn token_literal(&self) -> Option<&String> {
        if &self.token.literal != "" {
            Some(&self.token.literal)
        } else {
            None   
        }
    }
    fn node_type(&self) -> NodeType {
        NodeType::STATEMENT
    }
}


#[derive(Debug)]
pub struct ExpressionStatement {
    token: Token,
    pub expression: MonkeyExpression
}

impl ExpressionStatement {
    pub fn new(tok:Token, expr: MonkeyExpression) -> ExpressionStatement {
        ExpressionStatement { token: tok, expression: expr }
    }
}
impl MonkeyStatement for ExpressionStatement {}
// impl MonkeyExpr for ExpressionStatement {}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> Option<&String> {
        if &self.token.literal != "" {
            Some(&self.token.literal)
        } else {
            None
        }
    }
    fn node_type(&self) -> NodeType {
        NodeType::STATEMENT
    }
}
// impl Expression for ExpressionStatement {
//     fn expression_node(&self) {
        
//     }
// }

#[derive(Debug)]
pub struct BlockStatement {
    token: Token,
    pub statements: Vec<Statement>,
}
impl BlockStatement {
    pub fn new(tok: Token, statements: Vec<Statement>) -> BlockStatement {
        BlockStatement {token: tok, statements: statements}
    }
}
impl MonkeyExpr for BlockStatement {}

impl Node for BlockStatement {
    fn token_literal(&self) -> Option<&String> {
        if &self.token.literal != "" {
            Some(&self.token.literal)
        } else {
            None
        }
    }
    fn node_type(&self) -> NodeType {
        NodeType::BLOCKSTATEMENT
    }
}
impl Expression for BlockStatement {
    fn expression_node(&self) {}
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
impl MonkeyExpr for Identifier {}

impl Node for Identifier {
    fn token_literal(&self) -> Option<&String> {
        if &self.token.literal != "" {
            Some(&self.token.literal)
        } else {
            None
        }
    }
    fn node_type(&self) -> NodeType {
        NodeType::EXPRESSION
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
impl MonkeyExpr for IntegerLiteral {}

impl Node for IntegerLiteral {
    fn token_literal(&self) -> Option<&String> {
        if &self.token.literal != "" {
            Some(&self.token.literal)
        } else {
            None
        }
    }
    fn node_type(&self) -> NodeType {
        NodeType::EXPRESSION
    }
}
impl Expression for IntegerLiteral {
    fn expression_node(&self) {}
}

#[derive(Debug)]
pub struct Boolean {
    pub token: Token,
    pub value: bool,
}
impl Boolean {
    pub fn new(tok: Token, val: bool) -> Boolean {
        Boolean { token: tok, value: val }
    }
}
impl MonkeyExpr for Boolean {}

impl Node for Boolean {
    fn token_literal(&self) -> Option<&String> {
        if &self.token.literal != "" {
            Some(&self.token.literal)
        } else {
            None
        }
    }
    fn node_type(&self) -> NodeType {
        NodeType::EXPRESSION
    }
}
impl Expression for Boolean {
    fn expression_node(&self) {}
}
#[derive(Debug)]
pub struct PrefixExpression {
    pub token: Token,
    pub operator: String,
    pub right: Box<dyn MonkeyExpr>,
}
impl PrefixExpression {
    pub fn new(tok: Token, op: String, right: MonkeyExpression) -> PrefixExpression {
        PrefixExpression { token: tok, operator: op, right: Box::new(right) }
    }
}
impl MonkeyExpr for PrefixExpression {}

impl Node for PrefixExpression {
    fn token_literal(&self) -> Option<&String> {
        if &self.token.literal != "" {
            Some(&self.token.literal)
        } else {
            None
        }
    }
    fn node_type(&self) -> NodeType {
        NodeType::EXPRESSION
    }
}
impl Expression for PrefixExpression {
    fn expression_node(&self) {
        
    }
}

#[derive(Debug)]
pub struct InfixExpression {
    pub token: Token,
    pub operator: String,
    pub left: Box<dyn MonkeyExpr>,
    pub right: Box<dyn MonkeyExpr>,
}

impl InfixExpression {
    pub fn new(operator: String, tok: Token, left: Box<dyn MonkeyExpr>, right: Box<dyn MonkeyExpr>) -> InfixExpression {
        InfixExpression { operator: operator, token: tok,  left: left, right: right }
    }
}

impl MonkeyExpr for InfixExpression {}

impl Node for InfixExpression {
    fn token_literal(&self) -> Option<&String> {
        if &self.token.literal != "" {
            Some(&self.token.literal)
        } else {
            None
        }
    }
    fn node_type(&self) -> NodeType {
        NodeType::EXPRESSION
    }
}
impl Expression for InfixExpression {
    fn expression_node(&self) {
        
    }
}

#[derive(Debug)]
pub struct FunctionLiteral {
    token: Token,
    parameters: Vec<Identifier>,
    blockstatment: BlockStatement,
}
impl FunctionLiteral {
    pub fn new(tok: Token, params: Vec<Identifier>, blockstatement: BlockStatement) -> FunctionLiteral {
        FunctionLiteral { token: tok, parameters: params, blockstatment: blockstatement }
    } 
}
impl MonkeyExpr for FunctionLiteral {}

impl Node for FunctionLiteral {
    fn token_literal(&self) -> Option<&String> {
        if &self.token.literal != "" {
            Some(&self.token.literal)
        } else {
            None
        }
    }
    fn node_type(&self) -> NodeType {
        NodeType::EXPRESSION
    }
}
impl Expression for FunctionLiteral {
    fn expression_node(&self) {}
}
#[derive(Debug)]
pub struct IfExpression {
    tok: Token,
    condition: Box<dyn MonkeyExpr>,
    consequence: BlockStatement,
    alternative: Option<BlockStatement>,
}
impl IfExpression {
    pub fn new(tok: Token, condition: Box<dyn MonkeyExpr>, consequence: BlockStatement, alternative: Option<BlockStatement>) -> IfExpression {
        IfExpression { tok:tok, condition: condition, consequence: consequence, alternative: alternative }
    }
}
impl MonkeyExpr for IfExpression {}

impl Node for IfExpression {
    fn token_literal(&self) -> Option<&String> {
        self.condition.token_literal()
    }
    fn node_type(&self) -> NodeType {
        NodeType::EXPRESSION
    }
}
impl Expression for IfExpression {
    fn expression_node(&self) {}
}

#[derive(Debug)]
pub struct CallExpression {
    token: Token,
    function: Box<dyn MonkeyExpr>,
    arguments: Option<Vec<Box<dyn MonkeyExpr>>>,
}

impl  CallExpression {
    pub fn new(tok: Token, function: Box<dyn MonkeyExpr>, arguments: Option<Vec<Box<dyn MonkeyExpr>>>) -> CallExpression {
        CallExpression { token: tok, function, arguments }
    }
}

impl MonkeyExpr for CallExpression {}

impl Node for CallExpression {
    fn token_literal(&self) -> Option<&String> {
        if &self.token.literal != "" {
            Some(&self.token.literal)
        } else {
            None
        }       
    }
    fn node_type(&self) -> NodeType {
        NodeType::EXPRESSION
    }
}
impl Expression for CallExpression {
    fn expression_node(&self) {}
}