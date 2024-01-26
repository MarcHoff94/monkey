use crate::token::{ExpressionStatement, Identifier, IntegerLiteral, LetStatement, ReturnStatement};


pub trait Node {
    fn token_literal(&self) -> Option<&String>;
}
pub trait Expression {
    fn expression_node(&self);
}

#[derive(Debug)]
pub enum MonkeyExpression {
    IDENT(Identifier),
    INTEGERLITERAL(IntegerLiteral)
}
impl MonkeyExpression {
    pub fn into_expr(self) -> Box<dyn Expression> {
        match self {
            Self::IDENT(x) => Box::new(x),
            Self::INTEGERLITERAL(x) => Box::new(x),
        }
    }
}
impl Node for MonkeyExpression {
    fn token_literal(&self) -> Option<&String> {
        match &self {
            Self::IDENT(expr) => expr.token_literal(),
            Self::INTEGERLITERAL(expr) =>expr.token_literal(),
        }
    }
}
impl Expression for MonkeyExpression {
    fn expression_node(&self) {
        
    }
}
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

