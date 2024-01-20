pub trait Node {
    fn token_literal(&self) -> Option<&String>;
}

pub trait Statement: Node {
    fn statement_node(&self);
}

pub trait Expression: Node {
    fn expression_node(&self);
}

pub struct Programm {
    statements: Vec<Box<dyn Statement>>,
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