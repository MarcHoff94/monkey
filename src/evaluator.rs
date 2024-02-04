use crate::ast::{MonkeyStatement, Node, NodeType, Programm, Statement};
use crate::object::Object;
pub fn eval(program: Programm) -> Vec<Box<dyn Object>> {
    let mut object: Box<dyn Object>;
    let mut results: Vec<Box<dyn Object>> = Vec::new();
    for node in program.statements {
        object = match node {
            Statement::LET(stmt) => eval_statement(stmt).unwrap(),
            Statement::RETURN(stmt) => eval_statement(stmt).unwrap(),
            Statement::EXPRESSION(stmt) => eval_statement(stmt).unwrap(),
            Statement::BLOCK(stmt) => eval_block_statement(stmt.statements).unwrap(),
        };
        results.push(object);
    }
    results

}

fn eval_statement<T: MonkeyStatement>(node: T) -> Option<Box<dyn Object>> {
    None
}

fn eval_block_statement(stmts: Vec<Statement>) -> Option<Box<dyn Object>> {
    None
}