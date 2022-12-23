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

    // Algorithm
    // 1. For each sensor
    // 2. Take all the positions at sensor(value) + 1
    // 3. For each positions, check if another sensor is nearby

    for (point, t) in &items {
        match *t {
            Type::Beacon => continue,
            Type::Sensor(radius) => {
                println!("Looking at Sensor at position {:?}", point);
                let points = compute_point_outside_radius(point, radius);

                for p in points {
                    // 1. Check if in range (0,4_000_000)
                    if p.x() < 0 || p.x() > 4_000_000 || p.y() < 0 || p.y() > 4_000_000 {
                        continue;
                    }

                    // 2. Check if is already a Sensor or Beacon
                    if items.contains_key(&p) {
                        continue;
                    }

                    // 3. Check if in range of all Sensors
                    let mut is_in_range = false;

                    for (point, t) in &items {
                        match *t {
                            Type::Beacon => continue,
                            Type::Sensor(value) => {
                                if p.compare(point) <= value {
                                    is_in_range = true;
                                    break;
                                }
                            }
                        }
                    }
                    if is_in_range {
                        continue;
                    }

                    // 4. If all previous checks failed, success, we found the distress beacon
                    free_spot = p;
                    break;
                }
            }
        }

        if free_spot != Point::new(-1, -1) {
            break;
        }
    }

    println!("Found free spot at {:?}", free_spot);
    println!(
        "Tuning frequency: {}",
        free_spot.x() as u64 * 4_000_000 + free_spot.y() as u64
    );

    Ok(())
}

fn compute_point_outside_radius(p: &Point, radius: u32) -> Vec<Point> {
    let mut points = Vec::new();

    let mut left = Point::new(p.x() - radius as i32 - 1, p.y());
    let mut right = Point::new(p.x() + radius as i32 + 1, p.y());
    let mut up = Point::new(p.x(), p.y() - radius as i32 - 1);
    let mut down = Point::new(p.x(), p.y() + radius as i32 + 1);

    while left.x() != p.x() {
        points.push(left);

        *left.x_mut() += 1;
        *left.y_mut() -= 1;
    }

    while up.y() != p.y() {
        points.push(up);

        *up.x_mut() += 1;
        *up.y_mut() += 1;
    }

    while right.x() != p.x() {
        points.push(right);

        *right.x_mut() -= 1;
        *right.y_mut() += 1;
    }

    while down.x() != p.x() {
        points.push(down);

        *down.x_mut() -= 1;
        *down.y_mut() -= 1;
    }

    points
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
