use std::collections::HashMap;
use std::error::Error;
use std::io;

pub fn main_p1() -> Result<(), Box<dyn Error>> {
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

    let result = eval_monkey(String::from("root"), &monkeys);
    println!("Result: {}", result);

    Ok(())
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

#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq)]
enum Expression {
    Operation(String, Operator, String),
    Number(i64),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {}
}
