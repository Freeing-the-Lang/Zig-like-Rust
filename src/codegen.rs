use std::collections::HashMap;
use crate::ast::{Stmt, Expr};

pub fn run(stmts: Vec<Stmt>) {
    let mut env: HashMap<String, i64> = HashMap::new();

    for stmt in stmts {
        match stmt {
            Stmt::Const { name, value } => {
                let v = eval(&value, &env);
                env.insert(name, v);
            }
            Stmt::ExprStmt(e) => {
                let v = eval(&e, &env);
                println!("{}", v);
            }
        }
    }
}

fn eval(expr: &Expr, env: &HashMap<String, i64>) -> i64 {
    match expr {
        Expr::Number(n) => *n,
        Expr::Ident(s) => *env.get(s).unwrap_or(&0),
        Expr::Call { name, args } => match name.as_str() {
            "add" => eval(&args[0], env) + eval(&args[1], env),
            "sub" => eval(&args[0], env) - eval(&args[1], env),
            "mul" => eval(&args[0], env) * eval(&args[1], env),
            "div" => eval(&args[0], env) / eval(&args[1], env),
            "print" => {
                let v = eval(&args[0], env);
                println!("{}", v);
                v
            }
            _ => panic!("Unknown function {}", name),
        }
    }
}
