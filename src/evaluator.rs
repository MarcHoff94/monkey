use crate::ast::*;
use crate::object::*;
use crate::token::*;

pub fn eval(program: Vec<Statement>) -> Vec<MonkeyObject> {
    let mut object: MonkeyObject;
    let mut results: Vec<MonkeyObject> = Vec::new();
    for node in program {
        object = match node {
            Statement::LET(stmt) => eval_let_statement(stmt),
            Statement::RETURN(stmt) => eval_return_statement(stmt),
            Statement::EXPRESSION(stmt) => eval_expr_statement(stmt),
            Statement::BLOCK(stmt) => {
                let block_result = eval(stmt.statements);
                if check_for_return_statement(&block_result) {
                    return block_result
                } else {
                    MonkeyObject::BLOCK(Block { statements: block_result })
                }
            },
        };
        match object {
            MonkeyObject::RETURN(x) => return vec![MonkeyObject::RETURN(x)],
            _ => {results.push(object);}
        }
    }
    results

}
fn eval_let_statement(node: LetStatement) -> MonkeyObject {
    MonkeyObject::NULL(Null{})
}
fn eval_return_statement(node: ReturnStatement) -> MonkeyObject {
    MonkeyObject::RETURN(ReturnValue::new(Box::new(eval_expr(node.return_value).unwrap())))
}
fn check_for_return_statement(block_result: &Vec<MonkeyObject>) -> bool {
    match block_result.get(0).unwrap() {
        MonkeyObject::RETURN(x) => true,
        _ => false,
    }
}
fn eval_expr_statement(node: ExpressionStatement) -> MonkeyObject {
    eval_expr(node.expression).unwrap()
}

fn eval_expr(expr: MonkeyExpression) -> Result<MonkeyObject, &'static str> {
    match expr {
        MonkeyExpression::INTEGERLITERAL(x) => Ok(eval_integer_literal(x)),
        MonkeyExpression::BOOLEAN(x) => Ok(eval_bool(x)),
        MonkeyExpression::PREFIX(x) => eval_prefix_expr(&x.operator, *x.right),
        MonkeyExpression::INFIX(x) => Ok(eval_infix_expr(x.operator.as_str(), *x.left, *x.right).unwrap()),
        MonkeyExpression::IF(x) => Ok(eval_if_expr(x).unwrap()),
        _ => panic!("Unknown Expression {:#?}", expr),
    }
}

fn eval_integer_literal(int_lit: IntegerLiteral) -> MonkeyObject {
    MonkeyObject::INTEGER(Integer::new(int_lit.value))
}

fn eval_bool(bool_lit: Boolean) -> MonkeyObject {
    MonkeyObject::BOOLEAN(Bool::new(bool_lit.value)) 
}

fn eval_prefix_expr(operator: &String, right_expr: MonkeyExpression) -> Result<MonkeyObject, &'static str> {
    let right = eval_expr(right_expr).unwrap();
    match operator.as_str() {
        "!" => Ok(eval_bang_operator_expr(right)),
        "-" => eval_minus_operator_expr(right),
        _ => panic!("Illegal Prefix operator: {}", operator),
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
fn eval_minus_operator_expr(right: MonkeyObject) -> Result<MonkeyObject, &'static str> {
    match right {
        MonkeyObject::INTEGER(mut x) => {
            x.value = -x.value; 
            Ok(MonkeyObject::INTEGER(x))
        },
        _ => panic!("The minus operator can only be used for numeric values. you tried to use it on: {:#?}", right)
    }
    
}

fn eval_infix_expr(operator: &str, left_expr: MonkeyExpression, right_expr: MonkeyExpression) -> Result<MonkeyObject, &'static str> {
    let left = eval_expr(left_expr).unwrap();
    let right = eval_expr(right_expr).unwrap();
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

fn eval_if_expr(if_expr: IfExpression) -> Result<MonkeyObject, &'static str> {
    let condition = eval_expr(*if_expr.condition).unwrap();
    let result = match condition {
        MonkeyObject::BOOLEAN(x) => x,
        _ => panic!("Could not evaluate condition: result of condition was no Bool"),
    };
    if result.value {
        let consequence = eval(if_expr.consequence.statements);
        Ok(MonkeyObject::BLOCK(Block{statements: consequence}))
    } else {
        let alternative = match if_expr.alternative {
            Some(x) => eval(x.statements),
            None => Vec::new(),
        };
        if check_for_return_statement(&alternative) {
            return Ok(alternative[0])
        } else {
            Ok(MonkeyObject::BLOCK(Block { statements: alternative }))
        }
    }
}
