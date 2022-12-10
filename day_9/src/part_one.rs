use std::collections::HashSet;
use std::error::Error;
use std::io;

pub fn main_p1() -> Result<(), Box<dyn Error>> {
    let lines = io::stdin().lines();

    let mut head = Position::default();
    let mut tail = Position::default();

    let mut visited = HashSet::<Position>::new();
    visited.insert(tail.clone());

    for line in lines {
        let line = line.unwrap();

        let command: Vec<&str> = line.split(' ').collect();
        assert_eq!(command.len(), 2);

        let dir = Direction::from(command[0].chars().nth(0).unwrap());
        let steps: u32 = command[1].parse()?;

        for _ in 0..steps {
            head.mv(&dir);

            if head.distance(&tail) > 1 {
                if head.y == tail.y || head.x == tail.x {
                    // Still horizontally/vertically aligned
                    // Do the same movement as the head
                    tail.mv(&dir);
                } else {
                    // We need to make 2 moves to catch up to the head
                    // 1. Perform the same move as the head
                    tail.mv(&dir);

                    // Now we need to align the tail with the head on one dimension
                    match dir {
                        Direction::Up | Direction::Down => {
                            if head.x < tail.x {
                                tail.mv(&Direction::Left);
                            } else {
                                tail.mv(&Direction::Right);
                            }
                        }
                        Direction::Left | Direction::Right => {
                            if head.y < tail.y {
                                tail.mv(&Direction::Down);
                            } else {
                                tail.mv(&Direction::Up);
                            }
                        }
                    }
                }

                visited.insert(tail.clone());
            }

            assert_eq!(head.distance(&tail), 1);
        }
    }

    println!("Number of position visited: {}", visited.len());

    Ok(())
}

#[derive(Debug, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }

    fn distance(&self, p: &Self) -> usize {
        let mut distance = 0;

        if self.x != p.x {
            distance = (self.x - p.x).abs() as usize;
        }

        if self.y != p.y {
            distance = distance.max((self.y - p.y).abs() as usize);
        }

        distance
    }

    fn mv(&mut self, dir: &Direction) {
        match dir {
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }
}

impl Default for Position {
    fn default() -> Self {
        Position { x: 0, y: 0 }
    }
}

impl Clone for Position {
    fn clone(&self) -> Self {
        Position {
            x: self.x,
            y: self.y,
        }
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Position {}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Char not recognized."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position_distance_zero() {
        let a = Position::default();
        let b = Position::default();

        assert_eq!(a.distance(&b), 0);
    }

    #[test]
    fn position_distance_one_left() {
        let a = Position::new(2, 2);
        let b = Position::new(1, 2);

        assert_eq!(a.distance(&b), 1);
    }

    #[test]
    fn position_distance_one_right() {
        let a = Position::new(2, 2);
        let b = Position::new(3, 2);

        assert_eq!(a.distance(&b), 1);
    }

    #[test]
    fn position_distance_one_down() {
        let a = Position::new(2, 2);
        let b = Position::new(2, 1);

        assert_eq!(a.distance(&b), 1);
    }

    #[test]
    fn position_distance_one_up() {
        let a = Position::new(2, 2);
        let b = Position::new(2, 3);

        assert_eq!(a.distance(&b), 1);
    }

    #[test]
    fn position_distance_two_left() {
        let a = Position::new(2, 2);
        let b = Position::new(0, 2);

        assert_eq!(a.distance(&b), 2);
    }

    #[test]
    fn position_distance_two_right() {
        let a = Position::new(2, 2);
        let b = Position::new(4, 2);

        assert_eq!(a.distance(&b), 2);
    }

    #[test]
    fn position_distance_two_down() {
        let a = Position::new(2, 2);
        let b = Position::new(2, 0);

        assert_eq!(a.distance(&b), 2);
    }

    #[test]
    fn position_distance_two_up() {
        let a = Position::new(2, 2);
        let b = Position::new(2, 4);

        assert_eq!(a.distance(&b), 2);
    }

    #[test]
    fn position_distance_one_left_up() {
        let a = Position::new(2, 2);
        let b = Position::new(1, 3);

        assert_eq!(a.distance(&b), 1);
    }

    #[test]
    fn position_distance_one_right_up() {
        let a = Position::new(2, 2);
        let b = Position::new(3, 3);

        assert_eq!(a.distance(&b), 1);
    }

    #[test]
    fn position_distance_one_left_down() {
        let a = Position::new(2, 2);
        let b = Position::new(1, 1);

        assert_eq!(a.distance(&b), 1);
    }

    #[test]
    fn position_distance_one_right_down() {
        let a = Position::new(2, 2);
        let b = Position::new(3, 1);

        assert_eq!(a.distance(&b), 1);
    }

    #[test]
    fn position_distance_two_left_up() {
        let a = Position::new(2, 2);
        let b = Position::new(0, 4);

        assert_eq!(a.distance(&b), 2);
    }

    #[test]
    fn position_distance_two_right_up() {
        let a = Position::new(2, 2);
        let b = Position::new(4, 4);

        assert_eq!(a.distance(&b), 2);
    }

    #[test]
    fn position_distance_two_left_down() {
        let a = Position::new(2, 2);
        let b = Position::new(0, 0);

        assert_eq!(a.distance(&b), 2);
    }

    #[test]
    fn position_distance_two_right_down() {
        let a = Position::new(2, 2);
        let b = Position::new(4, 0);

        assert_eq!(a.distance(&b), 2);
    }
}
