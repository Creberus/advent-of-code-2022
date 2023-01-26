use std::collections::HashSet;
use std::error::Error;
use std::io;

// Position are as followed
// x right
// y up
// z back

pub fn main_p1() -> Result<(), Box<dyn Error>> {
    let lines = io::stdin().lines();

    let mut positions = HashSet::<Position>::new();

    for line in lines {
        let line = line.unwrap();

        let pos = Position::from(line.as_str());
        positions.insert(pos);
    }

    let mut surface_area: u32 = 0;

    for pos in &positions {
        let top = Position::new(pos.x(), pos.y() + 1, pos.z());
        let bot = Position::new(pos.x(), pos.y() - 1, pos.z());

        let right = Position::new(pos.x() + 1, pos.y(), pos.z());
        let left = Position::new(pos.x() - 1, pos.y(), pos.z());

        let back = Position::new(pos.x(), pos.y(), pos.z() + 1);
        let front = Position::new(pos.x(), pos.y(), pos.z() - 1);

        let neigbhors = vec![top, bot, right, left, back, front];

        for neigbhor in neigbhors {
            if !positions.contains(&neigbhor) {
                surface_area += 1;
            }
        }
    }

    println!("Total Surface Area: {}", surface_area);

    Ok(())
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    x: i8,
    y: i8,
    z: i8,
}

impl Position {
    fn new(x: i8, y: i8, z: i8) -> Self {
        Self { x, y, z }
    }

    fn x(&self) -> i8 {
        self.x
    }

    fn y(&self) -> i8 {
        self.y
    }

    fn z(&self) -> i8 {
        self.z
    }
}

impl From<&str> for Position {
    fn from(value: &str) -> Self {
        let values: Vec<&str> = value.split(',').collect();

        assert_eq!(values.len(), 3);

        let x = values[0].parse().unwrap();
        let y = values[1].parse().unwrap();
        let z = values[2].parse().unwrap();

        Position::new(x, y, z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_one_cube() {
        let input = "5,2,12";

        let pos = Position::from(input);

        assert_eq!(pos, Position::new(5, 2, 12));
    }
}
