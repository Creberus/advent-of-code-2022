use std::collections::HashSet;
use std::error::Error;
use std::io;

pub fn main_p1() -> Result<(), Box<dyn Error>> {
    let lines = io::stdin().lines();

    let mut positions = HashSet::<Position>::new();

    for line in lines {
        let line = line.unwrap();

        let pos = Position::from(line.as_str());
        positions.insert(pos);
    }

    Ok(())
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Position {
    x: u8,
    y: u8,
    z: u8,
}

impl Position {
    fn new(x: u8, y: u8, z: u8) -> Self {
        Self { x, y, z }
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
