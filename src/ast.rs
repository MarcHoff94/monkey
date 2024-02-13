use crate::token::*;
use std::fmt::Debug;

pub trait MonkeyExpr: Expression + Node + Debug + Clone {}

pub trait Node {
    fn token_literal(&self) -> Option<&String>;
    fn node_type(&self) -> NodeType;
}
pub trait MonkeyStatement: Node {}

pub trait Expression {
    fn expression_node(&self);
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub enum NodeType {
    PROGRAM,
    STATEMENT,
    BLOCKSTATEMENT,
    EXPRESSION,
}

#[derive(Debug, PartialEq, Clone)]
pub enum MonkeyExpression {
    IDENT(Identifier),
    INTEGERLITERAL(IntegerLiteral),
    BOOLEAN(Boolean),
    PREFIX(PrefixExpression),
    INFIX(InfixExpression),
    IF(IfExpression),
    FUNCTIONLITERAL(FunctionLiteral),
    CALL(CallExpression),
}
impl MonkeyExpression {
    // pub fn into_expr(self) -> Box<dyn MonkeyExpr> {
    //     match self {
    //         Self::IDENT(x) => Box::new(x),
    //         Self::INTEGERLITERAL(x) => Box::new(x),
    //         Self::BOOLEAN(x) => Box::new(x),
    //         Self::PREFIX(x) => Box::new(x),
    //         Self::INFIX(x) => Box::new(x),
    //         Self::IF(x) => Box::new(x),
    //         Self::FUNCTIONLITERAL(x) => Box::new(x),
    //         Self::CALL(x) => Box::new(x),
    //     }
    // }
}
impl Node for MonkeyExpression {
    fn token_literal(&self) -> Option<&String> {
        match &self {
            Self::IDENT(expr) => expr.token_literal(),
            Self::INTEGERLITERAL(expr) =>expr.token_literal(),
            Self::BOOLEAN(expr) => expr.token_literal(),
            Self::PREFIX(expr) => expr.token_literal(),
            Self::INFIX(expr) => expr.token_literal(),
            Self::IF(expr) => expr.token_literal(),
            Self::FUNCTIONLITERAL(expr) => expr.token_literal(),
            Self::CALL(expr) => expr.token_literal(),
        }
    }
    fn node_type(&self) -> NodeType {
        NodeType::EXPRESSION
    }
}
impl Expression for MonkeyExpression {
    fn expression_node(&self) {
        
    }
}
impl MonkeyExpr for MonkeyExpression {}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    LET(LetStatement),
    RETURN(ReturnStatement),
    EXPRESSION(ExpressionStatement),
    BLOCK(BlockStatement)
}
impl Node for Statement {
    fn token_literal(&self) -> Option<&String> {
        match &self {
            Self::LET(statement) => statement.token_literal(),
            Self::RETURN(statement) => statement.token_literal(),
            Self::EXPRESSION(statement) => statement.token_literal(),
            Self::BLOCK(statement) => statement.token_literal(),
        }
    }
    fn node_type(&self) -> NodeType {
        NodeType::STATEMENT
    }
}

#[derive(Debug)]
pub struct Programm {
    pub statements: Vec<Statement>,
}
impl Programm {
    fn print_program(&self) {
        let mut result = String::new();
        for stmt in &self.statements{
            result.push_str(stmt.token_literal().unwrap())
        }
        println!("{}", result);
    }
}
impl Node for Programm {
    fn token_literal(&self) -> Option<&String> {
        if self.statements.len() > 0 {
            match &self.statements[0].token_literal() {
                Some(x) => return Some(x),
                None => return None,
            }
        } else {
            None
        }
    }
    fn node_type(&self) -> NodeType {
        NodeType::PROGRAM
    }
}

