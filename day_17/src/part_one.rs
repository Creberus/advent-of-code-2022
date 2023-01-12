use std::collections::HashSet;
use std::error::Error;
use std::io;

//IDEA: One idea is to have each line represented as an u8: 0b10000001
// Where 0 is air and 1 is a rock/wall/ground
// This will take less memory overall but is harder to implement

pub fn main_p1() -> Result<(), Box<dyn Error>> {
    let lines = io::stdin().lines();

    let mut movements = Vec::new();

    for line in lines {
        let line = line.unwrap();

        let chars = line.chars();

        chars.for_each(|c| movements.push(Movement::from(c)))
    }

    let mut map = HashSet::<Position>::new();

    // Add the ground
    for x in 1..=7 {
        map.insert(Position::new(x, 0));
    }

    Ok(())
}

#[derive(Debug)]
enum Movement {
    Left,
    Right,
}

impl From<char> for Movement {
    fn from(value: char) -> Self {
        match value {
            '<' => Movement::Left,
            '>' => Movement::Right,
            _ => panic!("Unrecognized input."),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }

    fn x(&self) -> i32 {
        self.x
    }

    fn y(&self) -> i32 {
        self.y
    }
}

/// Shape describing our rocks falling in the cave.
#[derive(Debug)]
struct Shape {
    /// The current global position of our shape
    /// The down_left element is taken for the global position
    global_pos: Position,

    /// The right most elements of our shape
    max_x: i32,

    /// The elements of our shape
    locals_pos: Vec<Position>,

    /// Cache for local position that can occur a collision
    left_collision: Vec<Position>,
    right_collision: Vec<Position>,
    down_collision: Vec<Position>,
}

impl Shape {
    fn new(global_pos: Position, locals_pos: Vec<Position>) -> Self {
        let max_x = locals_pos.iter().max_by_key(|p| p.x()).unwrap().x();

        let mut left_collision = Vec::new();
        let mut right_collision = Vec::new();
        let mut down_collision = Vec::new();

        // Add hitbox for left collisions
        for local in &locals_pos {
            let left = Position::new(local.x() - 1, local.y());
            if !locals_pos.contains(&left) {
                left_collision.push(left);
            }
        }

        Shape {
            global_pos,
            max_x,
            locals_pos,
            left_collision,
            right_collision,
            down_collision,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {}
}
