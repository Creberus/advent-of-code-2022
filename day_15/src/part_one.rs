use std::error::Error;
use std::io;

pub fn main_p1() -> Result<(), Box<dyn Error>> {
    let lines = io::stdin().lines();

    let mut items = Vec::<Item>::new();

    for line in lines {
        let line = line.unwrap();

        let line: Vec<&str> = line.split(':').collect();

        let sensor = line[0];
        let beacon = line[1];

        let sensor = Item::parse(sensor);
        let beacon = Item::parse(beacon);

        items.push(sensor);
        items.push(beacon);
    }

    items.iter().for_each(|i| println!("{:?}", i));

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    fn parse(s: &str) -> Self {
        let input: Vec<&str> = s.split(", ").collect();

        let x_value: Vec<&str> = input[0].split('=').collect();
        let y_value: Vec<&str> = input[1].split('=').collect();

        let x = x_value[1].parse().unwrap();
        let y = y_value[1].parse().unwrap();

        Point::new(x, y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Type {
    Sensor,
    Beacon,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Item {
    t: Type,
    point: Point,
}

impl Item {
    fn new(t: Type, p: Point) -> Self {
        Item { t, point: p }
    }

    fn parse(s: &str) -> Self {
        let input: Vec<&str> = s.split(' ').collect();

        if input[0] == "Sensor" {
            let (_, coordinates) = s.split_at(10);

            let p = Point::parse(coordinates);

            Item::new(Type::Sensor, p)
        } else if input[2] == "beacon" {
            let (_, coordinates) = s.split_at(22);

            let p = Point::parse(coordinates);

            Item::new(Type::Beacon, p)
        } else {
            panic!("Can't parse item.");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_sensor() {
        let input = "Sensor at x=2, y=18";

        let item = Item::parse(input);

        assert_eq!(item, Item::new(Type::Sensor, Point::new(2, 18)));
    }

    #[test]
    fn simple_beacon() {
        let input = " closest beacon is at x=-2, y=15";

        let item = Item::parse(input);

        assert_eq!(item, Item::new(Type::Beacon, Point::new(-2, 15)));
    }
}
