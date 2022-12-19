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

    let mut map = Map::new();

    paths.iter().for_each(|p| map.apply_rock_path(p));

    let mut iterations = 0;

    while !map.fill() {
        iterations += 1;
    }

    //println!("{:?}", map);
    println!("It took {} iterations to fill the cave.", iterations);

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

        assert_eq!(values.len(), 2);

        let x = values[0].parse::<usize>()?;
        let y = values[1].parse::<usize>()?;

        Ok(Point { x, y })
    }

    fn x(&self) -> usize {
        self.x
    }

    fn y(&self) -> usize {
        self.y
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

    fn len(&self) -> usize {
        self.points.len()
    }

    fn iter(&self) -> PathIter {
        assert!(self.points.len() > 1);

        PathIter {
            path: self,
            index: 0,
        }
    }
}

impl<'a> IntoIterator for &'a Path {
    type Item = (Point, Point);
    type IntoIter = PathIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

struct PathIter<'a> {
    path: &'a Path,
    index: usize,
}

impl<'a> Iterator for PathIter<'a> {
    type Item = (Point, Point);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.path.len() - 1 {
            return None;
        }

        let item = (
            self.path.points[self.index],
            self.path.points[self.index + 1],
        );

        self.index += 1;

        Some(item)
    }
}

#[derive(Debug)]
struct Map {
    map: HashMap<Point, Element>,
    lowest_point: usize,
}

impl Map {
    fn new() -> Self {
        Map {
            map: HashMap::new(),
            lowest_point: 0,
        }
    }

    fn apply_rock_path(&mut self, rocks: &Path) {
        for (start, end) in rocks {
            if self.lowest_point < start.y() {
                self.lowest_point = start.y();
            }
            if self.lowest_point < end.y() {
                self.lowest_point = end.y();
            }

            if start.x() == end.x() {
                let x = start.x();
                let mut y = start.y();

                if y <= end.y() {
                    while y <= end.y() {
                        self.map.insert(Point::new(x, y), Element::Rock);
                        y += 1;
                    }
                } else {
                    while y >= end.y() {
                        self.map.insert(Point::new(x, y), Element::Rock);
                        y -= 1;
                    }
                }
            } else if start.y() == end.y() {
                let mut x = start.x();
                let y = start.y();

                if x <= end.x() {
                    while x <= end.x() {
                        self.map.insert(Point::new(x, y), Element::Rock);
                        x += 1;
                    }
                } else {
                    while x >= end.x() {
                        self.map.insert(Point::new(x, y), Element::Rock);
                        x -= 1;
                    }
                }
            } else {
                panic!("Points are not aligned !!!");
            }
        }
    }

    fn fill(&mut self) -> bool {
        let mut sand = Point::new(500, 0);

        // Sand logic
        while sand.y() < self.lowest_point {
            // 0. Check if we are in the Void

            // 1. Check for tile just below
            let below = Point::new(sand.x(), sand.y() + 1);
            let can_fall = self.map.get(&below);
            if can_fall.is_none() {
                sand = below;
                continue;
            }

            // 2. Check for tile one step down and to the left
            let below_left = Point::new(sand.x() - 1, sand.y() + 1);
            let can_fall = self.map.get(&below_left);
            if can_fall.is_none() {
                sand = below_left;
                continue;
            }

            // 3. Check for tile one step down and to the right
            let below_right = Point::new(sand.x() + 1, sand.y() + 1);
            let can_fall = self.map.get(&below_right);
            if can_fall.is_none() {
                sand = below_right;
                continue;
            }

            // 4. Sand is stuck and comes to rest
            self.map.insert(sand, Element::Sand);
            break;
        }

        return sand.y() >= self.lowest_point;
    }
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
