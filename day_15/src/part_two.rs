use std::collections::HashMap;
use std::error::Error;
use std::io;

pub fn main_p2() -> Result<(), Box<dyn Error>> {
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

    min_x = min_x.max(0);
    min_y = min_y.max(0);
    max_x = max_x.min(4_000_000);
    max_y = max_y.min(4_000_000);

    println!("min_x: {}", min_x);
    println!("max_x: {}", max_x);
    println!("min_y: {}", min_y);
    println!("max_y: {}", max_y);

    let mut free_spot = Point::new(-1, -1);
    let mut empty = true;

    // Algorithm
    // TODO
    unimplemented!();

    println!("Found free spot at {:?}", free_spot);
    println!(
        "Tuning frequency: {}",
        free_spot.x() * 4_000_000 + free_spot.y()
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

    fn x_mut(&mut self) -> &mut i32 {
        &mut self.x
    }

    fn y(&self) -> i32 {
        self.y
    }

    fn y_mut(&mut self) -> &mut i32 {
        &mut self.y
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
