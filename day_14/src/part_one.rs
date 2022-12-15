use std::collections::HashMap;
use std::error::Error;
use std::io;
use std::num::ParseIntError;

pub fn main_p1() -> Result<(), Box<dyn Error>> {
    let lines = io::stdin().lines();

    let mut paths = Vec::<Path>::new();

    for line in lines {
        let line = line.unwrap();

        let path = Path::parse(&line)?;

        paths.push(path);
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

#[derive(Debug, PartialEq, Eq)]
struct Path {
    points: Vec<Point>,
}

impl Path {
    fn new() -> Self {
        Path { points: Vec::new() }
    }

    fn parse(input: &str) -> Result<Self, ParseIntError> {
        let mut path = Path::new();

        let points: Vec<&str> = input.split(" -> ").collect();

        for point in points {
            path.add_point(Point::parse(point)?);
        }

        Ok(path)
    }

    fn add_point(&mut self, p: Point) {
        self.points.push(p)
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

    #[test]
    fn path_simple() {
        let input = "0,0 -> 1,1";
        let mut expected = Path::new();
        expected.add_point(Point::new(0, 0));
        expected.add_point(Point::new(1, 1));

        let path = Path::parse(input);

        assert_eq!(path, Ok(expected));
    }

    #[test]
    fn path_hard() {
        let input = "498,4 -> 498,6 -> 496,6";
        let mut expected = Path::new();
        expected.add_point(Point::new(498, 4));
        expected.add_point(Point::new(498, 6));
        expected.add_point(Point::new(496, 6));

        let path = Path::parse(input);

        assert_eq!(path, Ok(expected));
    }
}
