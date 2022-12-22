use std::collections::{HashMap, VecDeque};
use std::error::Error;
use std::io;

pub fn main_p1() -> Result<(), Box<dyn Error>> {
    let lines = io::stdin().lines();

    let mut items = Vec::<(Item, Item)>::new();
    let mut map = Map::new();

    for line in lines {
        let line = line.unwrap();

        let line: Vec<&str> = line.split(':').collect();

        let sensor = line[0];
        let beacon = line[1];

        let sensor = Item::parse(sensor);
        let beacon = Item::parse(beacon);

        items.push((sensor, beacon));
    }

    items.iter().for_each(|i| println!("{:?}", i));

    // For each couple of Sensor/Beacon, fill the map with waves from the sensor to the beacon in
    // circular shape.
    for (sensor, beacon) in items {
        let mut q = VecDeque::<Point>::new();
        let p = sensor.point();
        let mut beacon_found = false;
        let mut p_left = 1;

        map.insert(sensor.point(), sensor.t());
        map.insert(beacon.point(), beacon.t());

        q.push_back(p);

        while !q.is_empty() {
            let p = q.pop_front().unwrap();
            map.insert(p, Type::Scanned);

            if let Some(beacon) = map.has(&p) {
                if *beacon == Type::Beacon {
                    beacon_found = true;
                }
            }

            let up = Point::new(p.x(), p.y() - 1);
            let down = Point::new(p.x(), p.y() + 1);
            let left = Point::new(p.x() - 1, p.y());
            let right = Point::new(p.x() + 1, p.y());

            q.push_back(up);
            q.push_back(down);
            q.push_back(left);
            q.push_back(right);

            p_left -= 1;
            if p_left == 0 {
                p_left = q.len();
                if beacon_found {
                    break;
                }
            }
        }
    }

    // Look for row y=2_000_000
    let beacant = map.check(10);

    println!(
        "There are {} positions where a beacon cannot be present",
        beacant
    );

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

    fn x(&self) -> i32 {
        self.x
    }

    fn y(&self) -> i32 {
        self.y
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Type {
    Sensor,
    Beacon,
    Scanned,
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

    fn point(&self) -> Point {
        self.point
    }

    fn t(&self) -> Type {
        self.t
    }
}

#[derive(Debug)]
struct Map {
    map: HashMap<Point, Type>,
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

impl Map {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            min_x: i32::MAX,
            max_x: i32::MIN,
            min_y: i32::MAX,
            max_y: i32::MIN,
        }
    }

    fn insert(&mut self, p: Point, t: Type) -> bool {
        if self.map.contains_key(&p) {
            return false;
        } else {
            self.map.insert(p, t);
            self.min_y = self.min_y.min(p.y());
            self.min_x = self.min_x.min(p.x());
            self.max_y = self.max_y.max(p.y());
            self.max_x = self.max_x.max(p.x());
            return true;
        }
    }

    fn has(&self, p: &Point) -> Option<&Type> {
        self.map.get(&p)
    }

    fn check(&self, row: i32) -> u32 {
        let mut beacant = 0;

        for x in self.min_x..=self.max_x {
            if let Some(item) = self.map.get(&Point::new(x, row)) {
                match item {
                    Type::Beacon => continue,
                    _ => beacant += 1,
                }
            }
        }

        beacant
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
