use crate::token::{ExpressionStatement, Identifier, InfixExpression, IntegerLiteral, LetStatement, PrefixExpression, ReturnStatement};
use std::fmt::Debug;

pub trait MonkeyExpr: Expression + Node + Debug {}
pub trait Node {
    fn token_literal(&self) -> Option<&String>;
}

pub trait Expression {
    fn expression_node(&self);
}

#[derive(Debug)]
pub enum MonkeyExpression {
    IDENT(Identifier),
    INTEGERLITERAL(IntegerLiteral),
    PREFIX(PrefixExpression),
    INFIX(InfixExpression),
}
impl MonkeyExpression {
    pub fn into_expr(self) -> Box<dyn MonkeyExpr> {
        match self {
            Self::IDENT(x) => Box::new(x),
            Self::INTEGERLITERAL(x) => Box::new(x),
            Self::PREFIX(x) => Box::new(x),
            Self::INFIX(x) => Box::new(x),
        }
    }
}
impl Node for MonkeyExpression {
    fn token_literal(&self) -> Option<&String> {
        match &self {
            Self::IDENT(expr) => expr.token_literal(),
            Self::INTEGERLITERAL(expr) =>expr.token_literal(),
            Self::PREFIX(expr) => expr.token_literal(),
            Self::INFIX(expr) => expr.token_literal(),
        }
    }
}
impl Expression for MonkeyExpression {
    fn expression_node(&self) {
        
    }
}
impl MonkeyExpr for MonkeyExpression {}

#[derive(Debug)]
pub enum Statement {
    LET(LetStatement),
    RETURN(ReturnStatement),
    EXPRESSION(ExpressionStatement),
}
impl Node for Statement {
    fn token_literal(&self) -> Option<&String> {
        match &self {
            Self::LET(statement) => statement.token_literal(),
            Self::RETURN(statement) => statement.token_literal(),
            Self::EXPRESSION(statement) => statement.token_literal(),
        }
    }
}

pub struct Programm {
    pub statements: Vec<Statement>,
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
}

