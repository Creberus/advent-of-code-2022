use std::collections::VecDeque;
use std::error::Error;
use std::fmt::Display;
use std::io;

pub fn main_p2() -> Result<(), Box<dyn Error>> {
    let mut lines = io::stdin().lines();

    let mut id: usize = 0;
    let mut monkeys = Vec::<Monkey>::new();

    loop {
        let line = lines.next();
        if line.is_none() || line.unwrap().is_err() {
            break; // EOF
        }

        // 1. Create the monkey
        let mut monkey = Monkey::new(id);

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
        let test: u64 = test.split(' ').last().unwrap().parse().unwrap();
        monkey.divisible(test);

        // 5. Set monkey to throw when true
        let line = lines.next().unwrap().unwrap();
        let throw = line.split(':').last().unwrap();
        let throw: u32 = throw.split(' ').last().unwrap().parse().unwrap();
        *monkey.true_monkey_mut() = throw;

        // 5. Set monkey to throw when false
        let line = lines.next().unwrap().unwrap();
        let throw = line.split(':').last().unwrap();
        let throw: u32 = throw.split(' ').last().unwrap().parse().unwrap();
        *monkey.false_monkey_mut() = throw;

        monkeys.push(monkey);

        lines.next();

        id += 1;
    }

    let divisor = monkeys.iter().fold(1, |acc, m| acc * m.divisor());

    for round in 0..10000 {
        println!("Starting round {}", round);

        for index in 0..monkeys.len() {
            let mut to_throw = Vec::<(u32, u64)>::new();
            let monkey = monkeys.get_mut(index).unwrap();

            // 1. Perform the monkey steps for all the items
            while monkey.has_item() {
                monkey.inspect();
                monkey.bored();
                let monkey_to_throw = if monkey.is_divisible() {
                    monkey.true_monkey()
                } else {
                    monkey.false_monkey()
                };
                let item = monkey.throw() % divisor;

                // 2. We need to store the actions to do in order to avoid 2 mutable borrows
                to_throw.push((monkey_to_throw, item));
            }

            // 3. We finally throw all the items at once
            for throw in to_throw {
                monkeys[throw.0 as usize].push(throw.1);
            }
        }

        //monkeys.iter().for_each(|m| println!("{}", m));
    }

    monkeys.sort_by_key(|m| m.inspected());
    let mut max = monkeys.iter().rev();

    let monkey_business = max.next().unwrap().inspected() * max.next().unwrap().inspected();

    println!("Monkey business: {}", monkey_business);

    Ok(())
}

#[derive(Debug)]
struct Monkey {
    id: usize,
    items: VecDeque<u64>,
    operation: Operation,
    divisible: u64,
    true_monkey: u32,
    false_monkey: u32,
    inspected: u64,
}

impl Monkey {
    fn new(id: usize) -> Self {
        Monkey {
            id,
            items: VecDeque::new(),
            operation: Operation::default(),
            divisible: 0,
            true_monkey: 0,
            false_monkey: 0,
            inspected: 0,
        }
    }

    fn id(&self) -> usize {
        self.id
    }

    fn push(&mut self, item: u64) {
        self.items.push_back(item);
    }

    fn operation(&mut self, operation: Operation) {
        self.operation = operation;
    }

    fn divisible(&mut self, divisible: u64) {
        self.divisible = divisible;
    }

    fn true_monkey(&self) -> u32 {
        self.true_monkey
    }

    fn true_monkey_mut(&mut self) -> &mut u32 {
        &mut self.true_monkey
    }

    fn false_monkey(&self) -> u32 {
        self.false_monkey
    }

    fn false_monkey_mut(&mut self) -> &mut u32 {
        &mut self.false_monkey
    }

    fn has_item(&self) -> bool {
        !self.items.is_empty()
    }

    fn inspect(&mut self) {
        self.inspected += 1;

        match self.items.front_mut() {
            Some(item) => {
                let value = if let Value::Number(n) = self.operation.value() {
                    n
                } else {
                    *item
                };

                match self.operation.op() {
                    Operator::Add => *item += value,
                    Operator::Mul => *item *= value,
                }
            }
            None => panic!("Monkey has no items left."),
        }
    }

    fn bored(&mut self) {
        match self.items.front_mut() {
            Some(item) => {
                *item = *item;
            }
            None => panic!("Monkey has no items left."),
        }
    }

    fn is_divisible(&self) -> bool {
        self.items.front().unwrap() % self.divisible == 0
    }

    fn throw(&mut self) -> u64 {
        self.items.pop_front().unwrap()
    }

    fn inspected(&self) -> u64 {
        self.inspected
    }

    fn divisor(&self) -> u64 {
        self.divisible
    }
}

impl Display for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Monkey {}: {:?}", self.id, self.items)
    }
}

#[derive(Debug, Clone, Copy)]
struct Operation {
    op: Operator,
    val: Value,
}

impl Operation {
    fn new(op: Operator, val: Value) -> Self {
        Operation { op, val }
    }

    fn op(&self) -> Operator {
        self.op
    }

    fn value(&self) -> Value {
        self.val
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

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
enum Value {
    Number(u64),
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
