use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::io;

// Position are as followed
// x right
// y up
// z back

pub fn main_p2() -> Result<(), Box<dyn Error>> {
    let lines = io::stdin().lines();

    let mut positions = HashSet::<Position>::new();

    for line in lines {
        let line = line.unwrap();

        let pos = Position::from(line.as_str());
        positions.insert(pos);
    }

    // In order to compute the exterior surface area
    // We are simply using a flood fill algorithm to flood the outside of the lava droplet
    // This will give us a "shell" around the lava droplet

    let min_x = positions.iter().min_by_key(|p| p.x()).unwrap().x();
    let max_x = positions.iter().max_by_key(|p| p.x()).unwrap().x();

    let min_y = positions.iter().min_by_key(|p| p.y()).unwrap().y();
    let max_y = positions.iter().max_by_key(|p| p.y()).unwrap().y();

    let min_z = positions.iter().min_by_key(|p| p.z()).unwrap().z();
    let max_z = positions.iter().max_by_key(|p| p.z()).unwrap().z();

    let mut exteriors = HashSet::<Position>::new();

    let mut queue = VecDeque::<Position>::new();
    queue.push_back(Position::new(min_x - 1, min_y - 1, min_z - 1));
    exteriors.insert(Position::new(min_x - 1, min_y - 1, min_z - 1));

    let min = Position::new(min_x, min_y, min_z);
    let max = Position::new(max_x, max_y, max_z);

    while !queue.is_empty() {
        print!("Queue size: {}\r", queue.len());
        let pos = queue.pop_front().unwrap();

        let right = Position::new(pos.x() + 1, pos.y(), pos.z());
        let left = Position::new(pos.x() - 1, pos.y(), pos.z());
        let top = Position::new(pos.x(), pos.y() + 1, pos.z());
        let bot = Position::new(pos.x(), pos.y() - 1, pos.z());
        let back = Position::new(pos.x(), pos.y(), pos.z() + 1);
        let front = Position::new(pos.x(), pos.y(), pos.z() - 1);

        let neigbhors = vec![top, bot, right, left, back, front];

        for neigbhor in neigbhors {
            if !positions.contains(&neigbhor)
                && !exteriors.contains(&neigbhor)
                && !is_out_of_bound(neigbhor, min, max)
            {
                queue.push_back(neigbhor);
                exteriors.insert(neigbhor);
            }
        }
    }

    /*
    for z in min_z - 1..=max_z + 1 {
        println!("z: {}", z);
        for y in min_y - 1..=max_y + 1 {
            for x in min_x - 1..=max_x + 1 {
                if exteriors.contains(&Position::new(x, y, z)) {
                    print!("#");
                } else {
                    print!("*");
                }
            }
            println!("");
        }
        println!("");
        println!("");
    }*/

    // Compute surface area
    // But this time, take the air and chechks if a solid cube is near it because we only take care
    // of cubes that belongs to the shell of the lava droplet
    let mut surface_area: u32 = 0;

    for z in min_z - 1..=max_z + 1 {
        println!("z: {}", z);
        for y in min_y - 1..=max_y + 1 {
            for x in min_x - 1..=max_x + 1 {
                let pos = Position::new(x, y, z);

                if !exteriors.contains(&pos) {
                    let top = Position::new(pos.x(), pos.y() + 1, pos.z());
                    let bot = Position::new(pos.x(), pos.y() - 1, pos.z());

                    let right = Position::new(pos.x() + 1, pos.y(), pos.z());
                    let left = Position::new(pos.x() - 1, pos.y(), pos.z());

                    let back = Position::new(pos.x(), pos.y(), pos.z() + 1);
                    let front = Position::new(pos.x(), pos.y(), pos.z() - 1);

                    let neigbhors = vec![top, bot, right, left, back, front];

                    for neigbhor in neigbhors {
                        if exteriors.contains(&neigbhor) {
                            surface_area += 1;
                        }
                    }
                }
            }
        }
    }

    println!("Total Surface Area: {}", surface_area);

    Ok(())
}

fn is_out_of_bound(pos: Position, min: Position, max: Position) -> bool {
    pos.x() < min.x() - 1
        || pos.x() > max.x() + 1
        || pos.y() < min.y() - 1
        || pos.y() > max.y() + 1
        || pos.z() < min.z() - 1
        || pos.z() > max.z() + 1
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
