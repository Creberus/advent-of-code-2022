use std::error::Error;
use std::fmt::{Debug, Display};
use std::io;
use std::str::Chars;

pub fn main_p2() -> Result<(), Box<dyn Error>> {
    let mut lines = io::stdin().lines();

    let mut packets = Vec::<(Packet, Packet)>::new();

    loop {
        let mut line = lines.next();
        if line.is_none() {
            break; // EOF
        }

        let packet_one = line.unwrap().unwrap();
        line = lines.next();
        let packet_two = line.unwrap().unwrap();

        packets.push((Packet::parse(packet_one), Packet::parse(packet_two)));

        lines.next(); // Empty line between 2 packets
    }

    let mut results = Vec::<usize>::new();
    let mut index = 1;

    for pair in packets {
        println!("{}", pair.0);
        println!("{}", pair.1);
        let result = pair.0.compare(&pair.1);
        println!("{}", result);
        if result == -1 {
            results.push(index);
        }

        index += 1;
    }

    println!("Sum: {}", results.iter().sum::<usize>());

    Ok(())
}

#[derive(Debug)]
enum Node {
    Number(u32),
    Array(Vec<Node>),
}

impl Node {
    fn parse(input: &mut Chars) -> Self {
        let mut childs = Vec::new();

        loop {
            let c = input.next();
            if c.is_none() {
                break; // End of Input
            }

            let mut c = c.unwrap();

            if c == '[' {
                let child = Node::parse(input);
                childs.push(child);
            } else if c == ',' {
                continue;
            } else if c == ']' {
                break;
            } else {
                //TODO: While iterator not empty, ',', ']', on graille le chiffre
                let mut number = String::new();
                while c.is_digit(10) {
                    number.push(c);
                    c = input.next().unwrap();
                }
                childs.push(Node::Number(number.parse().unwrap()));
            }
        }

        Node::Array(childs)
    }
}

fn compare(a: &Node, b: &Node) -> i32 {
    match (a, b) {
        (Node::Number(a), Node::Number(b)) => {
            if a == b {
                0
            } else if a < b {
                -1
            } else {
                1
            }
        }
        (Node::Array(a), Node::Array(b)) => {
            let mut index = 0;
            let mut result = 0;

            while result == 0 && index < a.len() && index < b.len() {
                result = compare(&a[index], &b[index]);
                index += 1;
            }

            if result == 0 {
                if a.len() > b.len() {
                    result = 1;
                } else if a.len() < b.len() {
                    result = -1;
                }
            }

            result
        }
        (a, Node::Number(b)) => compare(a, &Node::Array(vec![Node::Number(*b)])),
        (Node::Number(a), b) => compare(&Node::Array(vec![Node::Number(*a)]), b),
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Number(a) => write!(f, "{}", a),
            Node::Array(arr) => {
                write!(f, "[")?;

                if !arr.is_empty() {
                    write!(f, "{}", arr[0])?;

                    for i in 1..arr.len() {
                        write!(f, ",{}", arr[i])?;
                    }
                }

                write!(f, "]")
            }
        }
    }
}

#[derive(Debug)]
struct Packet {
    root: Node,
}

impl Packet {
    fn parse(input: String) -> Self {
        let mut it = input[1..input.len()].chars();
        let root = Node::parse(&mut it);
        Packet { root }
    }

    fn compare(&self, other: &Self) -> i32 {
        compare(&self.root, &other.root)
    }
}

impl Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.root)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packets_empty() {
        let packet_one = String::from("[]");
        let packet_two = String::from("[]");

        let packet_one = Packet::parse(packet_one);
        let packet_two = Packet::parse(packet_two);

        assert_eq!(packet_one.compare(&packet_two), 0);
    }

    #[test]
    fn packets_small_left() {
        let packet_one = String::from("[4]");
        let packet_two = String::from("[12]");

        let packet_one = Packet::parse(packet_one);
        let packet_two = Packet::parse(packet_two);

        assert_eq!(packet_one.compare(&packet_two), -1);
    }

    #[test]
    fn packets_small_right() {
        let packet_one = String::from("[42]");
        let packet_two = String::from("[12]");

        let packet_one = Packet::parse(packet_one);
        let packet_two = Packet::parse(packet_two);

        assert_eq!(packet_one.compare(&packet_two), 1);
    }

    #[test]
    fn right_order_left_smaller() {
        let packet_one = String::from("[1,1,3,1,1]");
        let packet_two = String::from("[1,1,5,1,1]");

        let packet_one = Packet::parse(packet_one);
        let packet_two = Packet::parse(packet_two);

        assert_eq!(packet_one.compare(&packet_two), -1);
    }

    #[test]
    fn example_2() {
        let packet_one = String::from("[[1],[2,3,4]]");
        let packet_two = String::from("[[1],4]");

        let packet_one = Packet::parse(packet_one);
        let packet_two = Packet::parse(packet_two);

        assert_eq!(packet_one.compare(&packet_two), -1);
    }

    #[test]
    fn example_3() {
        let packet_one = String::from("[9]");
        let packet_two = String::from("[[8,7,6]]");

        let packet_one = Packet::parse(packet_one);
        let packet_two = Packet::parse(packet_two);

        assert_eq!(packet_one.compare(&packet_two), 1);
    }

    #[test]
    fn example_4() {
        let packet_one = String::from("[[4,4],4,4]");
        let packet_two = String::from("[[4,4],4,4,4]");

        let packet_one = Packet::parse(packet_one);
        let packet_two = Packet::parse(packet_two);

        assert_eq!(packet_one.compare(&packet_two), -1);
    }

    #[test]
    fn example_5() {
        let packet_one = String::from("[7,7,7,7]");
        let packet_two = String::from("[7,7,7]");

        let packet_one = Packet::parse(packet_one);
        let packet_two = Packet::parse(packet_two);

        assert_eq!(packet_one.compare(&packet_two), 1);
    }

    #[test]
    fn example_6() {
        let packet_one = String::from("[]");
        let packet_two = String::from("[3]");

        let packet_one = Packet::parse(packet_one);
        let packet_two = Packet::parse(packet_two);

        assert_eq!(packet_one.compare(&packet_two), -1);
    }

    #[test]
    fn example_7() {
        let packet_one = String::from("[[[]]]");
        let packet_two = String::from("[[]]");

        let packet_one = Packet::parse(packet_one);
        let packet_two = Packet::parse(packet_two);

        assert_eq!(packet_one.compare(&packet_two), 1);
    }

    #[test]
    fn example_8() {
        let packet_one = String::from("[1,[2,[3,[4,[5,6,7]]]],8,9]");
        let packet_two = String::from("[1,[2,[3,[4,[5,6,0]]]],8,9]");

        let packet_one = Packet::parse(packet_one);
        let packet_two = Packet::parse(packet_two);

        assert_eq!(packet_one.compare(&packet_two), 1);
    }

    #[test]
    fn example_9() {
        let packet_one = String::from("[[],[[[5,5,6,0,4],[6,0,8,2]],4],[[10,3,2,3]],[[[6]]],[[]]]");
        let packet_two = String::from("[[],[[7,1,[],[2,1],7],[[9],0,5],[10,[7,6,3,7],[9,3],9]],[],[[[9,4,9,4],[5,1,1,5]]],[10,5,7,0,[[3,8],[],2]]]");

        let packet_one = Packet::parse(packet_one);
        let packet_two = Packet::parse(packet_two);

        assert_eq!(packet_one.compare(&packet_two), -1);
    }
}
