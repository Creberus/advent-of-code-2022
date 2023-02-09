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
                grid.insert(Point::new((idx + 1) as u32, row), point_type);
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

    let (starting_point, sp_type) = grid
        .iter()
        .filter(|p| p.0.y() == 1)
        .min_by_key(|p| p.0.x())
        .unwrap();

    assert_eq!(*sp_type, PointType::Floor);

    let mut facing = Facing::Right;
    let mut current_point = *starting_point;

    for m in moves {
        match m {
            Move::Moving(mut value) => {
                while value > 0 {
                    let mut next_position = match facing {
                        Facing::Up => Point::new(current_point.x(), current_point.y() - 1),
                        Facing::Right => Point::new(current_point.x() + 1, current_point.y()),
                        Facing::Down => Point::new(current_point.x(), current_point.y() + 1),
                        Facing::Left => Point::new(current_point.x() - 1, current_point.y()),
                    };

                    if let Some(next_point) = grid.get(&next_position) {
                        match next_point {
                            PointType::Floor => (),
                            PointType::Wall => break,
                        }
                    } else {
                        // Find the point to the opposite side
                        let opposite_pos = match facing {
                            Facing::Up => {
                                let mut back_pos =
                                    Point::new(current_point.x(), current_point.y() + 1);
                                while let Some(_) = grid.get(&back_pos) {
                                    back_pos = Point::new(back_pos.x(), back_pos.y() + 1);
                                }

                                Point::new(back_pos.x(), back_pos.y() - 1)
                            }
                            Facing::Right => {
                                let mut back_pos =
                                    Point::new(current_point.x() - 1, current_point.y());
                                while let Some(_) = grid.get(&back_pos) {
                                    back_pos = Point::new(back_pos.x() - 1, back_pos.y());
                                }

                                Point::new(back_pos.x() + 1, back_pos.y())
                            }
                            Facing::Down => {
                                let mut back_pos =
                                    Point::new(current_point.x(), current_point.y() - 1);
                                while let Some(_) = grid.get(&back_pos) {
                                    back_pos = Point::new(back_pos.x(), back_pos.y() - 1);
                                }
                                Point::new(back_pos.x(), back_pos.y() + 1)
                            }
                            Facing::Left => {
                                let mut back_pos =
                                    Point::new(current_point.x() + 1, current_point.y());
                                while let Some(_) = grid.get(&back_pos) {
                                    back_pos = Point::new(back_pos.x() + 1, back_pos.y());
                                }
                                Point::new(back_pos.x() - 1, back_pos.y())
                            }
                        };

                        let pt = *grid.get(&opposite_pos).unwrap();

                        if pt == PointType::Floor {
                            next_position = opposite_pos;
                        } else {
                            break;
                        }
                    }

                    current_point = next_position;
                    value -= 1;
                }
            }
            Move::Turning(dir) => {
                facing = match dir {
                    Direction::Clockwise => match facing {
                        Facing::Up => Facing::Right,
                        Facing::Right => Facing::Down,
                        Facing::Down => Facing::Left,
                        Facing::Left => Facing::Up,
                    },
                    Direction::CounterClockwise => match facing {
                        Facing::Up => Facing::Left,
                        Facing::Right => Facing::Up,
                        Facing::Down => Facing::Right,
                        Facing::Left => Facing::Down,
                    },
                }
            }
        }
    }

    println!("Facing {:?} at {:?}", facing, current_point);

    println!(
        "Final Password: {}",
        1000 * current_point.y() + 4 * current_point.x() + facing as u32
    );

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
