mod parse;
mod tokenize;

use std::collections::HashMap;

use parse::{parse, AssignmentImpl, BinaryOpImpl, Expr, FunCallImpl, NumberImpl, VariableImpl};
use tokenize::{tokenize, TokenType::*};

type Env = HashMap<String, i32>;

fn interprete(exprs: &Vec<parse::Expr>, env: &mut Env) {
    for expr in exprs {
        evaluate(expr, env);
    }
}

// Expressions yield values; statements do not
fn evaluate(expr: &Expr, env: &mut Env) -> i32 {
    match expr {
        Expr::Assignment(AssignmentImpl { target, value }) => {
            let value = evaluate(value, env);
            env.insert(target.name.lexeme.clone(), value);
            value
        }
        Expr::FunCall(FunCallImpl { name, arg }) => {
            if name.name.lexeme == "print" {
                let arg = evaluate(arg, env);
                println!("{}", arg);
                arg
            } else {
                panic!("Undefined function {:?}", name.name.lexeme);
            }
        }

        Expr::BinaryOperation(BinaryOpImpl {
            lhs,
            operation,
            rhs,
        }) => {
            let lhs = evaluate(lhs, env);
            let rhs = evaluate(rhs, env);
            match operation.token_type {
                Plus => lhs + rhs,
                Minus => lhs - rhs,
                Star => lhs * rhs,
                Slash => lhs / rhs,
                _ => panic!("Invalid binary operation {:?}", operation),
            }
        }
        Expr::Number(NumberImpl { value, .. }) => *value,
        Expr::Variable(VariableImpl { name }) => {
            if let Some(value) = env.get(&name.lexeme) {
                *value
            } else {
                panic!("Variable {:?} is not defined", name.lexeme);
            }
        }
    }
}

fn main() {
    let src = "a=b=2 * 3\nc=print(b+3)\nprint(c/a)";
    let tokens = tokenize(src);
    let ast = parse(tokens);
    let mut env = HashMap::new();
    interprete(&ast, &mut env);
}
