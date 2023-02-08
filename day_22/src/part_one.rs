use std::collections::HashMap;
use std::error::Error;
use std::io;

pub fn main_p1() -> Result<(), Box<dyn Error>> {
    let mut lines = io::stdin().lines();

    let mut grid = HashMap::<Point, PointType>::new();
    let mut moves = Vec::new();
    let mut row = 1;

    let mut line = lines.next().unwrap().unwrap();

    // 1. Parse the Grid
    while !line.is_empty() {
        for (idx, c) in line.chars().enumerate() {
            if let Ok(point_type) = PointType::try_from(c) {
                grid.insert(Point::new(idx as u32, row), point_type);
            }
        }

        row += 1;
        line = lines.next().unwrap().unwrap();
    }

    line = lines.next().unwrap().unwrap();

    let moves_parsed: Vec<&str> = line.split_inclusive(&['L', 'R'][..]).collect();

    // 2. Parse the Moves
    for m in moves_parsed {
        let number = &m[..m.len() - 1];
        let turn = &m[m.len() - 1..];

        println!("Parsing number {} and turn {}", number, turn);

        if turn == "L" || turn == "R" {
            moves.push(Move::Moving(number.parse().unwrap()));
            moves.push(Move::Turning(Direction::from(turn)));
        } else {
            moves.push(Move::Moving(m.parse().unwrap()));
        }
    }

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    fn x(&self) -> u32 {
        self.x
    }

    fn y(&self) -> u32 {
        self.y
    }
}

#[derive(Debug, Clone, Copy)]
enum PointType {
    Wall,
    Floor,
}

impl TryFrom<char> for PointType {
    type Error = io::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(PointType::Floor),
            '#' => Ok(PointType::Wall),
            _ => Err(io::Error::new(io::ErrorKind::Unsupported, "nop")),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Facing {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Clockwise,
    CounterClockwise,
}

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value.chars().nth(0).unwrap() {
            'L' => Direction::CounterClockwise,
            'R' => Direction::Clockwise,
            a => panic!("Direction {} not recognized", a),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Move {
    Moving(u32),
    Turning(Direction),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {}
}
