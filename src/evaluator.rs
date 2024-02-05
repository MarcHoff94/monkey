use crate::ast::*;
use crate::object::*;
use crate::token::*;
pub fn eval(program: Programm) -> Vec<Box<dyn Object>> {
    let mut object: Box<dyn Object>;
    let mut results: Vec<Box<dyn Object>> = Vec::new();
    for node in program.statements {
        object = match node {
            Statement::LET(stmt) => eval_let_statement(stmt),
            Statement::RETURN(stmt) => eval_return_statement(stmt),
            Statement::EXPRESSION(stmt) => eval_expr_statement(stmt),
            Statement::BLOCK(stmt) => eval_block_statement(stmt.statements),
        };
        results.push(object);
    }
    results

}
fn eval_let_statement(node: LetStatement) -> Box<dyn Object> {
    Box::new(Null{})
}
fn eval_return_statement(node: ReturnStatement) -> Box<dyn Object> {
    Box::new(Null{})
}
fn eval_expr_statement(node: ExpressionStatement) -> Box<dyn Object> {
    eval_expr(node.expression)
}
fn eval_block_statement(stmts: Vec<Statement>) -> Box<dyn Object> {
    Box::new(Null{})
}
fn eval_expr(expr: MonkeyExpression) -> Box<dyn Object> {
    match expr {
        MonkeyExpression::INTEGERLITERAL(x) => eval_integer_literal(x),
        MonkeyExpression::BOOLEAN(x) => eval_bool(x),
        _ => Box::new(Null{}),
    }
}

fn eval_integer_literal(int_lit: IntegerLiteral) -> Box<dyn Object> {
    Box::new(Integer::new(int_lit.value))
}
fn eval_bool(bool_lit: Boolean) -> Box<dyn Object> {
    Box::new(Bool::new(bool_lit.value))
}