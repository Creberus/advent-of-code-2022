use std::error::Error;
use std::fmt::Debug;
use std::io;
use std::str::Chars;

pub fn main_p1() -> Result<(), Box<dyn Error>> {
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

    println!("{:?}", packets);

    Ok(())
}

#[derive(Debug)]
enum Node {
    Number(u32),
    Array(Vec<Node>),
}

impl Node {
    fn parse(input: &str) -> Self {
        let mut childs = Vec::new();

        let mut chars = input.chars().enumerate();

        loop {
            let c = chars.next();
            if c.is_none() {
                break; // End of Input
            }

            let (idx, c) = c.unwrap();

            if c == '[' {
                let child = Node::parse(&input[idx + 1..]);
                childs.push(child);
            } else if c == ',' {
                continue;
            } else if c == ']' {
                break;
            } else {
                let end = chars.position(|c| c.1 == ',' || c.1 == ']').unwrap();
                let child = input[idx..idx + end + 1].parse().unwrap();
                childs.push(Node::Number(child));
            }
        }

        Node::Array(childs)
    }
}

#[derive(Debug)]
struct Packet {
    root: Node,
}

impl Packet {
    fn parse(input: String) -> Self {
        let root = Node::parse(&input[1..input.len()]);
        Packet { root }
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
    }

    #[test]
    fn packets_small() {
        let packet_one = String::from("[42]");
        let packet_two = String::from("[12]");

        let packet_one = Packet::parse(packet_one);
        let packet_two = Packet::parse(packet_two);
    }

    #[test]
    fn right_order_left_smaller() {
        let packet_one = String::from("[1,1,3,1,1]");
        let packet_two = String::from("[1,1,5,1,1]");

        let packet_one = Packet::parse(packet_one);
        let packet_two = Packet::parse(packet_two);
    }
}
