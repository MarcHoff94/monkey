use crate::token::{LetStatement, ReturnStatement, ExpressionStatement};


pub trait Node {
    fn token_literal(&self) -> Option<&String>;
}

// pub trait Statement: Node {
//     fn statement_node(&self);
    
// }

pub trait Expression: Node {
    fn expression_node(&self);
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

