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

    println!("Tree Expr (before reduction): {}", tree_expr);

    let tree_expr = reduce_tree(tree_expr);

    println!("Tree Expr (after reduction): {}", tree_expr);

    let (value, path) = match tree_expr {
        TreeExpr::Root(l, r) => match (*l, *r) {
            (expr, TreeExpr::Number(value)) | (TreeExpr::Number(value), expr) => (value, expr),
            (_, _) => panic!("Error: Root malformed"),
        },
        _ => panic!("Root is malformed"),
    };

    let result = compute_variable(path, value);

    println!("The variable should be: {}", result);

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

fn reduce_tree(expr: TreeExpr) -> TreeExpr {
    match expr {
        TreeExpr::Operation(l, op, r) => {
            let l = reduce_tree(*l);
            let r = reduce_tree(*r);

            match (l, r) {
                (TreeExpr::Number(lvalue), TreeExpr::Number(rvalue)) => match op {
                    Operator::Plus => TreeExpr::Number(lvalue + rvalue),
                    Operator::Minus => TreeExpr::Number(lvalue - rvalue),
                    Operator::Mul => TreeExpr::Number(lvalue * rvalue),
                    Operator::Div => TreeExpr::Number(lvalue / rvalue),
                },
                (l, r) => TreeExpr::Operation(Box::new(l), op, Box::new(r)),
            }
        }
        TreeExpr::Root(l, r) => {
            let l = reduce_tree(*l);
            let r = reduce_tree(*r);

            TreeExpr::Root(Box::new(l), Box::new(r))
        }
        expr => expr,
    }
}

fn compute_variable(expr: TreeExpr, mut value: i64) -> i64 {
    match expr {
        TreeExpr::Operation(l, op, r) => match op {
            Operator::Plus => match (*l, *r) {
                (TreeExpr::Number(v), expr) | (expr, TreeExpr::Number(v)) => {
                    value -= v;
                    compute_variable(expr, value)
                }
                (_, _) => panic!(),
            },
            Operator::Minus => match (*l, *r) {
                (TreeExpr::Number(v), expr) => {
                    value = -(value - v);
                    compute_variable(expr, value)
                }
                (expr, TreeExpr::Number(v)) => {
                    value += v;
                    compute_variable(expr, value)
                }
                (_, _) => panic!(),
            },
            Operator::Mul => match (*l, *r) {
                (TreeExpr::Number(v), expr) | (expr, TreeExpr::Number(v)) => {
                    value /= v;
                    compute_variable(expr, value)
                }
                (_, _) => panic!(),
            },
            Operator::Div => match (*l, *r) {
                (TreeExpr::Number(v), expr) => {
                    value /= v;
                    compute_variable(expr, value)
                }
                (expr, TreeExpr::Number(v)) => {
                    value *= v;
                    compute_variable(expr, value)
                }
                (_, _) => panic!(),
            },
        },
        TreeExpr::Number(_) => value,
        TreeExpr::Variable() => value,
        _ => panic!("Tree Expr shouldn't be reached"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addition_left() {
        let tree = TreeExpr::Operation(
            Box::new(TreeExpr::Variable()),
            Operator::Plus,
            Box::new(TreeExpr::Number(15)),
        );

        let result = compute_variable(tree, 150);

        assert_eq!(result, 135);
    }

    #[test]
    fn addition_right() {
        let tree = TreeExpr::Operation(
            Box::new(TreeExpr::Number(20)),
            Operator::Plus,
            Box::new(TreeExpr::Variable()),
        );

        let result = compute_variable(tree, 150);

        assert_eq!(result, 130);
    }

    #[test]
    fn additions() {
        let tree = TreeExpr::Operation(
            Box::new(TreeExpr::Operation(
                Box::new(TreeExpr::Number(10)),
                Operator::Plus,
                Box::new(TreeExpr::Variable()),
            )),
            Operator::Plus,
            Box::new(TreeExpr::Number(10)),
        );

        let result = compute_variable(tree, 150);

        assert_eq!(result, 130);
    }

    #[test]
    fn multiplication_left() {
        let tree = TreeExpr::Operation(
            Box::new(TreeExpr::Variable()),
            Operator::Mul,
            Box::new(TreeExpr::Number(15)),
        );

        let result = compute_variable(tree, 150);

        assert_eq!(result, 10);
    }

    #[test]
    fn multiplication_right() {
        let tree = TreeExpr::Operation(
            Box::new(TreeExpr::Number(15)),
            Operator::Mul,
            Box::new(TreeExpr::Variable()),
        );

        let result = compute_variable(tree, 150);

        assert_eq!(result, 10);
    }

    #[test]
    fn multiplications() {
        let tree = TreeExpr::Operation(
            Box::new(TreeExpr::Operation(
                Box::new(TreeExpr::Number(10)),
                Operator::Mul,
                Box::new(TreeExpr::Variable()),
            )),
            Operator::Mul,
            Box::new(TreeExpr::Number(10)),
        );

        let result = compute_variable(tree, 200);

        assert_eq!(result, 2);
    }

    #[test]
    fn substraction_left() {
        let tree = TreeExpr::Operation(
            Box::new(TreeExpr::Variable()),
            Operator::Minus,
            Box::new(TreeExpr::Number(15)),
        );

        let result = compute_variable(tree, 150);

        assert_eq!(result, 165);
    }

    #[test]
    fn substraction_right() {
        let tree = TreeExpr::Operation(
            Box::new(TreeExpr::Number(15)),
            Operator::Minus,
            Box::new(TreeExpr::Variable()),
        );

        let result = compute_variable(tree, 150);

        assert_eq!(result, -135);
    }

    #[test]
    fn substractions() {
        let tree = TreeExpr::Operation(
            Box::new(TreeExpr::Operation(
                Box::new(TreeExpr::Number(10)),
                Operator::Minus,
                Box::new(TreeExpr::Variable()),
            )),
            Operator::Minus,
            Box::new(TreeExpr::Number(10)),
        );

        let result = compute_variable(tree, 200);

        assert_eq!(result, -200);
    }
}
