use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Write};
use std::io;

//TODO: Reduce the tree to have a node with : humn + <number>
// Then, swap the nodes to the other side of the tree, to compute humn

pub fn main_p2() -> Result<(), Box<dyn Error>> {
    let lines = io::stdin().lines();

    let mut monkeys = HashMap::<String, Expression>::new();

    for line in lines {
        let line = line.unwrap();

        let (monkey, expr) = line.split_once(": ").unwrap();

        let expr: Vec<&str> = expr.split(' ').collect();

        if expr.len() == 1 {
            monkeys.insert(
                monkey.to_string(),
                Expression::Number(expr[0].parse().unwrap()),
            );
        } else if expr.len() == 3 {
            monkeys.insert(
                monkey.to_string(),
                Expression::Operation(
                    expr[0].to_string(),
                    Operator::from(expr[1]),
                    expr[2].to_string(),
                ),
            );
        } else {
            panic!("Expression not recognized '{:?}'", expr);
        }
    }

    let tree_expr = build_expr_tree(String::from("root"), &monkeys);

    println!("Tree Expr: {}", tree_expr);

    Ok(())
}
fn eval_root(monkeys: &HashMap<String, Expression>) -> i64 {
    let root = monkeys.get(&String::from("root")).unwrap();

    match root {
        Expression::Operation(l, _, r) => {
            let lmonkey = eval(monkeys.get(l).unwrap(), monkeys);
            let rmonkey = eval(monkeys.get(r).unwrap(), monkeys);

            rmonkey - lmonkey
        }
        _ => panic!("Root should be an Operation"),
    }
}

fn eval_monkey(monkey: String, monkeys: &HashMap<String, Expression>) -> i64 {
    eval(monkeys.get(&monkey).unwrap(), monkeys)
}

fn eval(expr: &Expression, monkeys: &HashMap<String, Expression>) -> i64 {
    match expr {
        Expression::Number(value) => *value,
        Expression::Operation(lmonkey, op, rmonkey) => {
            let left = eval(monkeys.get(lmonkey).unwrap(), monkeys);
            let right = eval(monkeys.get(rmonkey).unwrap(), monkeys);
            match op {
                Operator::Plus => left + right,
                Operator::Minus => left - right,
                Operator::Mul => left * right,
                Operator::Div => left / right,
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Operator {
    Plus,
    Minus,
    Mul,
    Div,
}

impl From<&str> for Operator {
    fn from(value: &str) -> Self {
        match value.chars().nth(0) {
            Some(value) => match value {
                '+' => Operator::Plus,
                '-' => Operator::Minus,
                '*' => Operator::Mul,
                '/' => Operator::Div,
                _ => panic!("Character {} invalid", value),
            },
            None => panic!("Number invalid"),
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Plus => f.write_str("+"),
            Operator::Minus => f.write_str("-"),
            Operator::Mul => f.write_str("*"),
            Operator::Div => f.write_str("/"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Expression {
    Operation(String, Operator, String),
    Number(i64),
}

#[derive(Debug, PartialEq, Eq)]
enum TreeExpr {
    Number(i64),
    Variable(),
    Operation(Box<TreeExpr>, Operator, Box<TreeExpr>),
    Root(Box<TreeExpr>, Box<TreeExpr>),
}

impl Display for TreeExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TreeExpr::Root(l, r) => f.write_fmt(format_args!("({}) == ({})", l, r)),
            TreeExpr::Number(value) => f.write_fmt(format_args!("{}", value)),
            TreeExpr::Variable() => f.write_str("x"),
            TreeExpr::Operation(l, op, r) => f.write_fmt(format_args!("({} {} {})", l, op, r)),
        }
    }
}

fn build_expr_tree(monkey: String, monkeys: &HashMap<String, Expression>) -> TreeExpr {
    let m = monkeys.get(&monkey).unwrap();

    if *monkey == String::from("root") {
        if let Expression::Operation(lm, _, rm) = m {
            let lexpr = build_expr_tree(lm.clone(), monkeys);
            let rexpr = build_expr_tree(rm.clone(), monkeys);

            TreeExpr::Root(Box::new(lexpr), Box::new(rexpr))
        } else {
            panic!("Root is malformed");
        }
    } else if *monkey == String::from("humn") {
        TreeExpr::Variable()
    } else {
        match m {
            Expression::Number(value) => TreeExpr::Number(*value),
            Expression::Operation(lm, op, rm) => {
                let lexpr = build_expr_tree(lm.clone(), monkeys);
                let rexpr = build_expr_tree(rm.clone(), monkeys);

                TreeExpr::Operation(Box::new(lexpr), *op, Box::new(rexpr))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {}
}
