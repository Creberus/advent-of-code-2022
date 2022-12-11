use std::collections::VecDeque;
use std::error::Error;
use std::io;

pub fn main_p1() -> Result<(), Box<dyn Error>> {
    let mut lines = io::stdin().lines();

    let mut monkeys = Vec::<Monkey>::new();

    loop {
        let line = lines.next();
        if line.is_none() || line.unwrap().is_err() {
            break; // EOF
        }

        // 1. Create the monkey
        let mut monkey = Monkey::new();

        // 2. Add starting items to the monkey
        let line = lines.next().unwrap().unwrap();
        let starting_items = line.split(':').last().unwrap();
        for item in starting_items.split(',') {
            let (_, item) = item.split_at(1);
            monkey.push(item.parse().unwrap());
        }

        // 3. Add Operation to the monkey
        let line = lines.next().unwrap().unwrap();
        let operation = line.split(':').last().unwrap();
        let operation: Vec<&str> = operation.split(' ').collect();
        let op = Operator::from(operation[4]);
        let value = Value::from(operation[5]);
        let operation = Operation::new(op, value);
        monkey.operation(operation);

        // 4. Add test
        let line = lines.next().unwrap().unwrap();
        let test = line.split(':').last().unwrap();
        let test: u32 = test.split(' ').last().unwrap().parse().unwrap();
        monkey.divisible(test);

        // 5. Set monkey to throw when true
        let line = lines.next().unwrap().unwrap();
        let throw = line.split(':').last().unwrap();
        let throw: u32 = throw.split(' ').last().unwrap().parse().unwrap();
        monkey.true_monkey(throw);

        // 5. Set monkey to throw when false
        let line = lines.next().unwrap().unwrap();
        let throw = line.split(':').last().unwrap();
        let throw: u32 = throw.split(' ').last().unwrap().parse().unwrap();
        monkey.false_monkey(throw);

        monkeys.push(monkey);

        lines.next();
    }

    Ok(())
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<u32>,
    operation: Operation,
    divisible: u32,
    true_monkey: u32,
    false_monkey: u32,
}

impl Monkey {
    fn new() -> Self {
        Monkey {
            items: VecDeque::new(),
            operation: Operation::default(),
            divisible: 0,
            true_monkey: 0,
            false_monkey: 0,
        }
    }

    fn push(&mut self, item: u32) {
        self.items.push_back(item);
    }

    fn operation(&mut self, operation: Operation) {
        self.operation = operation;
    }

    fn divisible(&mut self, divisible: u32) {
        self.divisible = divisible;
    }

    fn true_monkey(&mut self, monkey: u32) {
        self.true_monkey = monkey;
    }

    fn false_monkey(&mut self, monkey: u32) {
        self.false_monkey = monkey;
    }

    fn throw(&mut self, monkey: &mut Self) {
        monkey.push(self.items.pop_front().unwrap());
    }
}

#[derive(Debug)]
struct Operation {
    op: Operator,
    val: Value,
}

impl Operation {
    fn new(op: Operator, val: Value) -> Self {
        Operation { op, val }
    }
}

impl Default for Operation {
    fn default() -> Self {
        Operation {
            op: Operator::Add,
            val: Value::Old,
        }
    }
}

#[derive(Debug)]
enum Operator {
    Add,
    Mul,
}

impl From<&str> for Operator {
    fn from(s: &str) -> Self {
        match s.chars().nth(0).unwrap() {
            '+' => Operator::Add,
            '*' => Operator::Mul,
            _ => panic!("Not an operator"),
        }
    }
}

#[derive(Debug)]
enum Value {
    Number(u32),
    Old,
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        if s == "old" {
            Value::Old
        } else {
            Value::Number(s.parse().unwrap())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {}
}
