use std::collections::HashSet;
use std::error::Error;
use std::io;

const HEAD: usize = 0;
const TAIL: usize = 9;

pub fn main_p2() -> Result<(), Box<dyn Error>> {
    let lines = io::stdin().lines();

    let mut rope: [Position; 10] = [Position::default(); 10];
    let mut rope_dirs: [Direction; 10] = [Direction::Undefined; 10];

    let mut visited = HashSet::<Position>::new();
    visited.insert(rope[TAIL]);

    for line in lines {
        let line = line.unwrap();

        let command: Vec<&str> = line.split(' ').collect();
        assert_eq!(command.len(), 2);

        let dir = Direction::from(command[0].chars().nth(0).unwrap());
        let steps: u32 = command[1].parse()?;

        for _ in 0..steps {
            // 1. Move the head
            rope[HEAD].mv(dir);
            rope_dirs[HEAD] = dir;

            // 2. Makes every child to follow
            for child in (HEAD + 1)..(TAIL + 1) {
                // 2.5 Every part of the rope can be seen as (head, tail)
                // Thus, we can apply the follow function to each of them
                // Each part will follow the part in front of her
                let head = rope[child - 1];
                let mut tail = rope[child];

                let child_dir = tail.follow(&head, rope_dirs[child - 1]);
                rope[child] = tail;
                rope_dirs[child] = child_dir;
            }

            // 3. Insert the position of the TAIL
            visited.insert(rope[TAIL]);

            print!("Rope: [");
            for part in rope {
                print!("{:?},", part);
            }
            println!("]\n");

            // 4. Check if all the parts are close to each other
            for part in HEAD..TAIL {
                assert!(rope[part].distance(&rope[part + 1]) < 2);
            }
        }
    }

    println!("Number of position visited: {}", visited.len());

    Ok(())
}

#[derive(Debug, Hash, Copy)]
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

    fn mv(&mut self, dir: Direction) {
        match dir {
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
            Direction::Undefined => panic!("Direction is Undefined"),
        }
    }

    fn follow(&mut self, other: &Self, dir: Direction) -> Direction {
        if self.distance(&other) > 1 {
            if other.y == self.y {
                // Horizontally aligned
                if other.x > self.x {
                    self.mv(Direction::Right);
                    Direction::Right
                } else {
                    self.mv(Direction::Left);
                    Direction::Left
                }
            } else if other.x == self.x {
                // vertically aligned
                if other.y > self.y {
                    self.mv(Direction::Up);
                    Direction::Up
                } else {
                    self.mv(Direction::Down);
                    Direction::Down
                }
            } else {
                // We need to make 2 moves to catch up to the head
                // 1. Perform the same move as the head
                self.mv(dir);

                // Now we need to align the tail with the head on one dimension
                match dir {
                    Direction::Up | Direction::Down => {
                        if other.x < self.x {
                            self.mv(Direction::Left);
                            Direction::Left
                        } else {
                            self.mv(Direction::Right);
                            Direction::Right
                        }
                    }
                    Direction::Left | Direction::Right => {
                        if other.y < self.y {
                            self.mv(Direction::Down);
                            Direction::Down
                        } else {
                            self.mv(Direction::Up);
                            Direction::Up
                        }
                    }
                    Direction::Undefined => panic!("Direction is Undefined"),
                }
            }
        } else {
            Direction::Undefined
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Undefined,
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

    #[test]
    fn position_follow_up() {
        let head = Position::new(0, 2);
        let mut tail = Position::new(0, 0);
        let dir_expected = Direction::Up;

        let dir_actual = tail.follow(&head, dir_expected);

        assert_eq!(dir_actual, dir_expected);
        assert_eq!(head.distance(&tail), 1);
    }

    #[test]
    fn position_follow_down() {
        let head = Position::new(0, -2);
        let mut tail = Position::new(0, 0);
        let dir_expected = Direction::Down;

        let dir_actual = tail.follow(&head, dir_expected);

        assert_eq!(dir_actual, dir_expected);
        assert_eq!(head.distance(&tail), 1);
    }

    #[test]
    fn position_follow_left() {
        let head = Position::new(-2, 0);
        let mut tail = Position::new(0, 0);
        let dir_expected = Direction::Left;

        let dir_actual = tail.follow(&head, dir_expected);

        assert_eq!(dir_actual, dir_expected);
        assert_eq!(head.distance(&tail), 1);
    }

    #[test]
    fn position_follow_right() {
        let head = Position::new(2, 0);
        let mut tail = Position::new(0, 0);
        let dir_expected = Direction::Right;

        let dir_actual = tail.follow(&head, dir_expected);

        assert_eq!(dir_actual, dir_expected);
        assert_eq!(head.distance(&tail), 1);
    }

    #[test]
    fn position_not_follow() {
        let head = Position::new(0, 0);
        let mut tail = Position::new(0, 0);
        let dir_expected = Direction::Undefined;

        let dir_actual = tail.follow(&head, dir_expected);

        assert_eq!(dir_actual, dir_expected);
        assert_eq!(head.distance(&tail), 0);
    }

    #[test]
    fn position_not_follow_left() {
        let head = Position::new(-1, 0);
        let mut tail = Position::new(0, 0);
        let dir_expected = Direction::Undefined;

        let dir_actual = tail.follow(&head, Direction::Left);

        assert_eq!(dir_actual, dir_expected);
        assert_eq!(head.distance(&tail), 1);
    }

    #[test]
    fn position_not_follow_right() {
        let head = Position::new(1, 0);
        let mut tail = Position::new(0, 0);
        let dir_expected = Direction::Undefined;

        let dir_actual = tail.follow(&head, Direction::Right);

        assert_eq!(dir_actual, dir_expected);
        assert_eq!(head.distance(&tail), 1);
    }

    #[test]
    fn position_not_follow_up() {
        let head = Position::new(0, 1);
        let mut tail = Position::new(0, 0);
        let dir_expected = Direction::Undefined;

        let dir_actual = tail.follow(&head, Direction::Up);

        assert_eq!(dir_actual, dir_expected);
        assert_eq!(head.distance(&tail), 1);
    }

    #[test]
    fn position_not_follow_down() {
        let head = Position::new(0, -1);
        let mut tail = Position::new(0, 0);
        let dir_expected = Direction::Undefined;

        let dir_actual = tail.follow(&head, Direction::Down);

        assert_eq!(dir_actual, dir_expected);
        assert_eq!(head.distance(&tail), 1);
    }

    #[test]
    fn position_follow_left_up() {
        let head = Position::new(-2, 1);
        let mut tail = Position::new(0, 0);

        let dir = tail.follow(&head, Direction::Left);

        assert_eq!(dir, Direction::Up);
        assert_eq!(head.distance(&tail), 1);
    }

    #[test]
    fn position_follow_left_down() {
        let head = Position::new(-2, -1);
        let mut tail = Position::new(0, 0);

        let dir = tail.follow(&head, Direction::Left);

        assert_eq!(dir, Direction::Down);
        assert_eq!(head.distance(&tail), 1);
    }

    #[test]
    fn position_follow_right_up() {
        let head = Position::new(2, 1);
        let mut tail = Position::new(0, 0);

        let dir = tail.follow(&head, Direction::Right);

        assert_eq!(dir, Direction::Up);
        assert_eq!(head.distance(&tail), 1);
    }

    #[test]
    fn position_follow_right_down() {
        let head = Position::new(2, -1);
        let mut tail = Position::new(0, 0);

        let dir = tail.follow(&head, Direction::Right);

        assert_eq!(dir, Direction::Down);
        assert_eq!(head.distance(&tail), 1);
    }

    #[test]
    fn position_follow_up_left() {
        let head = Position::new(-1, 2);
        let mut tail = Position::new(0, 0);

        let dir = tail.follow(&head, Direction::Up);

        assert_eq!(dir, Direction::Left);
        assert_eq!(head.distance(&tail), 1);
    }

    #[test]
    fn position_follow_up_right() {
        let head = Position::new(1, 2);
        let mut tail = Position::new(0, 0);

        let dir = tail.follow(&head, Direction::Up);

        assert_eq!(dir, Direction::Right);
        assert_eq!(head.distance(&tail), 1);
    }

    #[test]
    fn position_follow_down_left() {
        let head = Position::new(-1, -2);
        let mut tail = Position::new(0, 0);

        let dir = tail.follow(&head, Direction::Down);

        assert_eq!(dir, Direction::Left);
        assert_eq!(head.distance(&tail), 1);
    }

    #[test]
    fn position_follow_down_right() {
        let head = Position::new(1, -2);
        let mut tail = Position::new(0, 0);

        let dir = tail.follow(&head, Direction::Down);

        assert_eq!(dir, Direction::Right);
        assert_eq!(head.distance(&tail), 1);
    }
}
