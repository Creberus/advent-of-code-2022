use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;
use std::io;

pub fn main_p1() -> Result<(), Box<dyn Error>> {
    let lines = io::stdin().lines();

    let mut elves = HashSet::<Point>::new();

    let mut row = 0;

    for line in lines {
        let line = line.unwrap();

        for (idx, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    elves.insert(Point::new(idx as i32, row));
                    ()
                }
                _ => continue,
            }
        }

        row += 1;
    }

    let mut directions = VecDeque::from(vec![
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ]);

    for round in 0..10 {
        let mut propositions = HashMap::<Point, i32>::new();
        let mut moves = HashMap::<Point, Point>::new();

        // 1st half
        for elve in &elves {
            if !is_alone(&elves, elve) {
                for dir in &directions {
                    if can_move(&elves, elve, *dir) {
                        let position = match dir {
                            Direction::North => Point::new(elve.x(), elve.y() - 1),
                            Direction::South => Point::new(elve.x(), elve.y() + 1),
                            Direction::West => Point::new(elve.x() - 1, elve.y()),
                            Direction::East => Point::new(elve.x() + 1, elve.y()),
                        };

                        moves.insert(*elve, position);

                        let value = if let Some(value) = propositions.get(&position) {
                            *value
                        } else {
                            0
                        };

                        propositions.insert(position, value + 1);

                        break;
                    }
                }
            }
        }

        // 2nd half
        for m in moves {
            if let Some(value) = propositions.get(&m.1) {
                if *value == 1 {
                    elves.remove(&m.0);
                    elves.insert(m.1);
                }
            }
        }

        // Cycle the directions
        let dir = directions.pop_front().unwrap();
        directions.push_back(dir);
    }

    let x_min = elves.iter().min_by_key(|p| p.x()).unwrap().x();
    let x_max = elves.iter().max_by_key(|p| p.x()).unwrap().x();

    let y_min = elves.iter().min_by_key(|p| p.y()).unwrap().y();
    let y_max = elves.iter().max_by_key(|p| p.y()).unwrap().y();

    let mut empty_tile = 0;

    for y in y_min..=y_max {
        for x in x_min..=x_max {
            if let None = elves.get(&Point::new(x, y)) {
                empty_tile += 1;
            }
        }
    }

    println!("Empty tiles: {}", empty_tile);

    Ok(())
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    North,
    South,
    West,
    East,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn x(&self) -> i32 {
        self.x
    }

    fn y(&self) -> i32 {
        self.y
    }
}

fn is_alone(elves: &HashSet<Point>, elve: &Point) -> bool {
    let up = Point::new(elve.x(), elve.y() - 1);
    let down = Point::new(elve.x(), elve.y() + 1);

    let left = Point::new(elve.x() - 1, elve.y());
    let right = Point::new(elve.x() + 1, elve.y());

    let up_left = Point::new(left.x(), up.y());
    let up_right = Point::new(right.x(), up.y());

    let down_left = Point::new(left.x(), down.y());
    let down_right = Point::new(right.x(), down.y());

    let positions = vec![
        up, down, left, right, up_left, up_right, down_left, down_right,
    ];

    !positions.iter().any(|p| elves.contains(p))
}

fn can_move(elves: &HashSet<Point>, elve: &Point, dir: Direction) -> bool {
    let up = Point::new(elve.x(), elve.y() - 1);
    let down = Point::new(elve.x(), elve.y() + 1);

    let left = Point::new(elve.x() - 1, elve.y());
    let right = Point::new(elve.x() + 1, elve.y());

    let up_left = Point::new(left.x(), up.y());
    let up_right = Point::new(right.x(), up.y());

    let down_left = Point::new(left.x(), down.y());
    let down_right = Point::new(right.x(), down.y());

    let positions = match dir {
        Direction::North => vec![up, up_left, up_right],
        Direction::South => vec![down, down_left, down_right],
        Direction::West => vec![left, up_left, down_left],
        Direction::East => vec![right, up_right, down_right],
    };

    !positions.iter().any(|p| elves.contains(p))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {}
}
