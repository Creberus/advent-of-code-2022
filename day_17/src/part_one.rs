use std::collections::HashSet;
use std::error::Error;
use std::io;
use std::ops::Add;

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

    let mut rocks = HashSet::<Position>::new();
    let mut map = Map::new();
    let shapes_order = vec![
        ShapeType::HorizontalLine,
        ShapeType::Cross,
        ShapeType::ReverseL,
        ShapeType::VerticalLine,
        ShapeType::Square,
    ];

    // Add the ground
    for x in 1..=7 {
        rocks.insert(Position::new(x, 0));
    }

    let mut movement_index = 0;

    for shape_index in 0..2022 {
        print!("Shape n°{}/2021\r", shape_index);

        // Get the current shape
        let shape_type = shapes_order[shape_index % shapes_order.len()];
        let mut shape = Shape::from(shape_type);

        // Spawn the shape at the good position
        let mut highest_position = map.highest_position();
        highest_position += 4;

        *shape.global_pos_mut().x_mut() += 3;
        *shape.global_pos_mut().y_mut() = highest_position;

        let mut come_to_rest = false;

        // Start the logic loop
        while !come_to_rest {
            // Apply jet of hot gas
            let jet_hot_gas = movements[movement_index % movements.len()];
            movement_index += 1;

            match jet_hot_gas {
                Movement::Left => {
                    if can_push_left(&shape, &map, &rocks) {
                        *shape.global_pos_mut().x_mut() -= 1
                    }
                }
                Movement::Right => {
                    if can_push_right(&shape, &map, &rocks) {
                        *shape.global_pos_mut().x_mut() += 1
                    }
                }
            }

            // Fall one unit down
            if can_fall_one(&shape, &map, &rocks) {
                *shape.global_pos_mut().y_mut() -= 1;
            } else {
                // Add shape to rocks
                rocks.extend(shape.rocks().iter());
                come_to_rest = true;
                let highest_position = shape.rocks().iter().max_by_key(|r| r.y()).unwrap().y();
                if highest_position > map.highest_position() {
                    *map.highest_position_mut() = highest_position;
                }
            }
        }
    }

    //println!("{:?}", rocks);
    println!("Highest position: {}", map.highest_position());

    Ok(())
}

fn can_push_left(shape: &Shape, map: &Map, rocks: &HashSet<Position>) -> bool {
    if shape.global_pos().x() - 1 == map.x_min() {
        false
    } else if rocks.intersection(&shape.left_collision()).count() > 0 {
        false
    } else {
        true
    }
}

fn can_push_right(shape: &Shape, map: &Map, rocks: &HashSet<Position>) -> bool {
    if (shape.global_pos().x() + shape.max_x()) + 1 == map.x_max() {
        false
    } else if rocks.intersection(&shape.right_collision()).count() > 0 {
        false
    } else {
        true
    }
}

fn can_fall_one(shape: &Shape, map: &Map, rocks: &HashSet<Position>) -> bool {
    rocks.intersection(&shape.down_collision()).count() == 0
}

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
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

    fn x_mut(&mut self) -> &mut i32 {
        &mut self.x
    }

    fn y(&self) -> i32 {
        self.y
    }

    fn y_mut(&mut self) -> &mut i32 {
        &mut self.y
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Position::new(self.x + rhs.x, self.y + rhs.y)
    }
}

#[derive(Debug, Clone, Copy)]
enum ShapeType {
    HorizontalLine,
    VerticalLine,
    Cross,
    ReverseL,
    Square,
}

/// Shape describing our rocks falling in the cave.
///
/// We have 5 possible shapes which are represented as such:
/// (.: Nothing, #: Element, <: Hitbox Left, >: Hitbox Right, v: Hitbox Down)
/// (Hitboxes can overlap)
///
/// Horizontal Line:
/// <####>
/// .vvvv.
///
/// Cross:
/// .<#>.
/// <###>
/// .<#>. | .v#v.
/// ..v..
///
/// Reverse L:
/// ..<#>
/// ..<#>
/// <###>
/// .vvv.
///
/// Vertical Line:
/// <#>
/// <#>
/// <#>
/// <#>
/// .v.
///
/// Square:
/// <##>
/// <##>
/// .vv.
///
/// The Left/Right hitbox will be used when the element is pushed by a jet of hot gas.
/// The Down hitbox will be used to detect if we reached a ground or another element when falling.
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
    left_collision: HashSet<Position>,
    right_collision: HashSet<Position>,
    down_collision: HashSet<Position>,
}

impl Shape {
    fn new(global_pos: Position, locals_pos: Vec<Position>) -> Self {
        let max_x = locals_pos.iter().max_by_key(|p| p.x()).unwrap().x();

        let mut left_collision = HashSet::new();
        let mut right_collision = HashSet::new();
        let mut down_collision = HashSet::new();

        // Add hitbox for left collisions
        for local in &locals_pos {
            let left = Position::new(local.x() - 1, local.y());
            if !locals_pos.contains(&left) {
                left_collision.insert(left);
            }
        }

        // Add hitbox for right collisions
        for local in &locals_pos {
            let right = Position::new(local.x() + 1, local.y());
            if !locals_pos.contains(&right) {
                right_collision.insert(right);
            }
        }

        // Add hitbox for down collisions
        for local in &locals_pos {
            let down = Position::new(local.x(), local.y() - 1);
            if !locals_pos.contains(&down) {
                down_collision.insert(down);
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

    fn global_pos(&self) -> &Position {
        &self.global_pos
    }

    fn global_pos_mut(&mut self) -> &mut Position {
        &mut self.global_pos
    }

    fn max_x(&self) -> i32 {
        self.max_x
    }

    fn left_collision(&self) -> HashSet<Position> {
        // Here we need to convert local position to global position.
        let mut global_left_collision = HashSet::new();

        for left_col in &self.left_collision {
            global_left_collision.insert(self.global_pos + *left_col);
        }

        global_left_collision
    }

    fn right_collision(&self) -> HashSet<Position> {
        // Here we need to convert local position to global position.
        let mut global_right_collision = HashSet::new();

        for right_col in &self.right_collision {
            global_right_collision.insert(self.global_pos + *right_col);
        }

        global_right_collision
    }

    fn down_collision(&self) -> HashSet<Position> {
        // Here we need to convert local position to global position.
        let mut global_down_collision = HashSet::new();

        for down_col in &self.down_collision {
            global_down_collision.insert(self.global_pos + *down_col);
        }

        global_down_collision
    }

    fn rocks(&self) -> HashSet<Position> {
        let mut global_positions = HashSet::new();

        for pos in &self.locals_pos {
            global_positions.insert(self.global_pos + *pos);
        }

        global_positions
    }
}

impl From<ShapeType> for Shape {
    fn from(value: ShapeType) -> Self {
        match value {
            ShapeType::HorizontalLine => {
                //  x 0123
                // y
                // 0  ####
                let mut locals_pos = Vec::new();
                locals_pos.push(Position::new(0, 0));
                locals_pos.push(Position::new(1, 0));
                locals_pos.push(Position::new(2, 0));
                locals_pos.push(Position::new(3, 0));
                Self::new(Position::new(0, 0), locals_pos)
            }
            ShapeType::VerticalLine => {
                //  x 0
                // y
                // 3  #
                // 2  #
                // 1  #
                // 0  #
                let mut locals_pos = Vec::new();
                locals_pos.push(Position::new(0, 0));
                locals_pos.push(Position::new(0, 1));
                locals_pos.push(Position::new(0, 2));
                locals_pos.push(Position::new(0, 3));
                Self::new(Position::new(0, 0), locals_pos)
            }
            ShapeType::Cross => {
                //  x 012
                // y
                // 2   #
                // 1  ###
                // 0   #
                let mut locals_pos = Vec::new();
                locals_pos.push(Position::new(1, 2));
                locals_pos.push(Position::new(0, 1));
                locals_pos.push(Position::new(1, 1));
                locals_pos.push(Position::new(2, 1));
                locals_pos.push(Position::new(1, 0));
                Self::new(Position::new(0, 0), locals_pos)
            }
            ShapeType::ReverseL => {
                //  x 012
                // y
                // 2    #
                // 1    #
                // 0  ###
                let mut locals_pos = Vec::new();
                locals_pos.push(Position::new(2, 2));
                locals_pos.push(Position::new(2, 1));
                locals_pos.push(Position::new(0, 0));
                locals_pos.push(Position::new(1, 0));
                locals_pos.push(Position::new(2, 0));
                Self::new(Position::new(0, 0), locals_pos)
            }
            ShapeType::Square => {
                //  x 01
                // y
                // 1  ##
                // 0  ##
                let mut locals_pos = Vec::new();
                locals_pos.push(Position::new(0, 0));
                locals_pos.push(Position::new(1, 0));
                locals_pos.push(Position::new(0, 1));
                locals_pos.push(Position::new(1, 1));
                Self::new(Position::new(0, 0), locals_pos)
            }
        }
    }
}

#[derive(Debug)]
struct Map {
    highest_position: i32,
    x_min: i32,
    x_max: i32,
}

impl Map {
    fn new() -> Self {
        Self {
            highest_position: 0,
            x_min: 0,
            x_max: 8,
        }
    }

    fn highest_position(&self) -> i32 {
        self.highest_position
    }

    fn highest_position_mut(&mut self) -> &mut i32 {
        &mut self.highest_position
    }

    fn x_min(&self) -> i32 {
        self.x_min
    }

    fn x_max(&self) -> i32 {
        self.x_max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct_horizontal_line() {}
}
