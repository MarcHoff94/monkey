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
    eval_expr(node.expression).into_obj()
}
fn eval_block_statement(stmts: Vec<Statement>) -> Box<dyn Object> {
    Box::new(Null{})
}
fn eval_expr(expr: MonkeyExpression) -> MonkeyObject {
    match expr {
        MonkeyExpression::INTEGERLITERAL(x) => eval_integer_literal(x),
        MonkeyExpression::BOOLEAN(x) => eval_bool(x),
        MonkeyExpression::PREFIX(x) => eval_prefix_expr(&x.operator, *x.right),
        MonkeyExpression::INFIX(x) => eval_infix_expr(x.operator.as_str(), *x.left, *x.right).unwrap(),
        _ => MonkeyObject::NULL(Null{}),
    }
}

fn eval_integer_literal(int_lit: IntegerLiteral) -> MonkeyObject {
    MonkeyObject::INTEGER(Integer::new(int_lit.value))
}

fn eval_bool(bool_lit: Boolean) -> MonkeyObject {
    MonkeyObject::BOOLEAN(Bool::new(bool_lit.value)) 
}

fn eval_prefix_expr(operator: &String, right_expr: MonkeyExpression) -> MonkeyObject {
    let right = eval_expr(right_expr);
    match operator.as_str() {
        "!" => eval_bang_operator_expr(right),
        "-" => eval_minus_operator_expr(right),
        _ => MonkeyObject::NULL(Null{})
    }
}

fn eval_bang_operator_expr(right: MonkeyObject) -> MonkeyObject {
    match right {
        MonkeyObject::BOOLEAN(x) => { match x.value {
            true => MonkeyObject::BOOLEAN(Bool { value: false }),
            false => MonkeyObject::BOOLEAN(Bool { value: true })
            }
        }
        MonkeyObject::NULL(x) => MonkeyObject::BOOLEAN(Bool { value: true }),
        _ => MonkeyObject::BOOLEAN(Bool { value: false })
    }
}
fn eval_minus_operator_expr(right: MonkeyObject) -> MonkeyObject {
    match right {
        MonkeyObject::INTEGER(mut x) => {
            x.value = -x.value; 
            MonkeyObject::INTEGER(x)
        },
        _ => MonkeyObject::NULL(Null{}) 
    }
    
}

fn eval_infix_expr(operator: &str, left_expr: MonkeyExpression, right_expr: MonkeyExpression) -> Result<MonkeyObject, &'static str> {
    let left = eval_expr(left_expr);
    let right = eval_expr(right_expr);
    match (left, right) {
        (MonkeyObject::INTEGER(l), MonkeyObject::INTEGER(r)) => eval_integer_infix_expr(operator, &l, &r),
        (MonkeyObject::BOOLEAN(l), MonkeyObject::BOOLEAN(r)) => eval_bool_infix_expr(operator, &l, &r),
        _ => panic!("illegal infix expr")
    }
}
fn eval_integer_infix_expr(operator: &str, left: &Integer, right: &Integer) -> Result<MonkeyObject, &'static str> {
    let result = match operator {
        "+" => MonkeyObject::INTEGER(Integer { value: left.value + right.value }),
        "-" => MonkeyObject::INTEGER(Integer { value: left.value - right.value }),
        "*" => MonkeyObject::INTEGER(Integer { value: left.value * right.value }),
        "/" => MonkeyObject::INTEGER(Integer { value: left.value / right.value }),
        "^" => MonkeyObject::INTEGER(Integer { value: left.value.pow(right.value.try_into().unwrap()) }),
        "<" => MonkeyObject::BOOLEAN(Bool { value: left.value < right.value }),
        ">" => MonkeyObject::BOOLEAN(Bool { value: left.value > right.value }),
        "<=" => MonkeyObject::BOOLEAN(Bool { value: left.value <= right.value }),
        ">=" => MonkeyObject::BOOLEAN(Bool { value: left.value >= right.value }),
        "!=" => MonkeyObject::BOOLEAN(Bool { value: left.value != right.value }),
        "==" => MonkeyObject::BOOLEAN(Bool { value: left.value == right.value }),
        _ => panic!("Illegal operator in integer Infixexpression: {}", operator)
    };
    Ok(result)
}

fn eval_bool_infix_expr(operator: &str, left: &Bool, right: &Bool) -> Result<MonkeyObject, &'static str> {
    let result = match operator {
        "==" => MonkeyObject::BOOLEAN(Bool { value: left.value == right.value }),
        "!=" => MonkeyObject::BOOLEAN(Bool { value: left.value != right.value }),
        _ => panic!("illegal operator for boolean comparison: {}", operator),
    };
    Ok(result)
}