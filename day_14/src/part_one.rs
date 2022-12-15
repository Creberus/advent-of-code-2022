use std::collections::HashMap;
use std::error::Error;
use std::io;
use std::num::ParseIntError;

pub fn main_p1() -> Result<(), Box<dyn Error>> {
    let mut lines = io::stdin().lines();

    loop {
        let line = lines.next();
        if line.is_none() {
            break; // EOF
        }
    }

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Element {
    Rock,
    Air,
    Sand,
    Void,
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }

    fn parse(input: &str) -> Result<Self, ParseIntError> {
        let values: Vec<&str> = input.split(',').collect();

        //TODO: Add check if we have 2 elements

        let x = values[0].parse::<usize>()?;
        let y = values[1].parse::<usize>()?;

        Ok(Point { x, y })
    }
}

struct Map {
    map: HashMap<Point, Element>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_zero_zero() {
        let input = "0,0";

        let point = Point::parse(input);

        assert_eq!(point, Ok(Point::new(0, 0)));
    }

    #[test]
    fn point_simple() {
        let input = "5,8";

        let point = Point::parse(input);

        assert_eq!(point, Ok(Point::new(5, 8)));
    }

    #[test]
    fn point_hard() {
        let input = "488,164";

        let point = Point::parse(input);

        assert_eq!(point, Ok(Point::new(488, 164)));
    }

    #[test]
    fn point_failed_no_comma() {
        let input = "488164";

        let point = Point::parse(input);

        assert!(point.is_err());
    }

    #[test]
    fn point_failed_empty() {
        let input = "";

        let point = Point::parse(input);

        assert!(point.is_err());
    }

    #[test]
    fn point_failed_left_empty() {
        let input = ",45";

        let point = Point::parse(input);

        assert!(point.is_err());
    }

    #[test]
    fn point_failed_right_empty() {
        let input = "45,";

        let point = Point::parse(input);

        assert!(point.is_err());
    }
}
