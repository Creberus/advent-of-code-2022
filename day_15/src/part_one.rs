use std::collections::HashMap;
use std::error::Error;
use std::io;

pub fn main_p1() -> Result<(), Box<dyn Error>> {
    let lines = io::stdin().lines();

    let mut items = HashMap::<Point, Type>::new();

    for line in lines {
        let line = line.unwrap();

        let line: Vec<&str> = line.split(':').collect();

        let sensor = line[0];
        let beacon = line[1];

        let (_, sensor) = sensor.split_at(10);
        let (_, beacon) = beacon.split_at(22);

        let sensor = Point::parse(sensor);
        let beacon = Point::parse(beacon);

        items.insert(sensor, Type::Sensor(sensor.compare(&beacon)));
        items.insert(beacon, Type::Beacon);
    }

    items.iter().for_each(|i| println!("{:?}", i));

    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;

    for (point, t) in &items {
        match t {
            Type::Beacon => {
                min_x = min_x.min(point.x());
                min_y = min_y.min(point.y());
                max_x = max_x.max(point.x());
                max_y = max_y.max(point.y());
            }
            Type::Sensor(value) => {
                min_x = min_x.min(point.x() - *value as i32);
                min_y = min_y.min(point.y() - *value as i32);
                max_x = max_x.max(point.x() + *value as i32);
                max_y = max_y.max(point.y() + *value as i32);
            }
        }
    }

    println!("min_x: {}", min_x);
    println!("max_x: {}", max_x);
    println!("min_y: {}", min_y);
    println!("max_y: {}", max_y);

    let row = 2_000_000;
    let mut taken = 0;

    // Algorithm
    // 1. Take a point in the map
    // 2. Check if it's already a Beacon or a Sensor
    // 3. Compute manathan distance for each sensor
    // 4. Compare with manathan distance of the sensor with it's closest beacon
    // 5. If it's higher than everyone, it's empty ! Else, it's scanned.
    for x in min_x..=max_x {
        // 1.
        let p = Point::new(x, row);
        print!("x: {}\r", x);

        // 2.
        if let Some(item) = items.get(&p) {
            match item {
                Type::Beacon => (),
                Type::Sensor(_) => taken += 1,
            }

            continue;
        }

        // 3+4+5.
        for (point, t) in &items {
            match t {
                Type::Sensor(value) => {
                    if p.compare(point) <= *value {
                        taken += 1;
                        break;
                    }
                }
                _ => (),
            }
        }
    }

    println!(
        "There are {} positions where a beacon cannot be present.",
        taken
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

    fn compare(&self, other: &Self) -> u32 {
        (self.x - other.x).abs() as u32 + (self.y - other.y).abs() as u32
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
    Sensor(u32),
    Beacon,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_pos() {
        let input = "x=2, y=18";
    }

    #[test]
    fn point_neg() {
        let input = "x=-2, y=15";
    }
}
