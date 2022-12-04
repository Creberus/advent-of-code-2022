use std::error::Error;
use std::io;

struct Sections {
    start: u32,
    end: u32,
}

impl Sections {
    fn new(start: u32, end: u32) -> Self {
        Sections { start, end }
    }
}

pub fn main_p2() -> Result<(), Box<dyn Error>> {
    let lines = io::stdin().lines();
    let mut overlap: u32 = 0;

    for line in lines {
        let l = line.unwrap();

        let elements: Vec<&str> = l.split_terminator(['-', ',']).collect();
        let first = Sections::new(
            elements[0].to_string().parse().unwrap(),
            elements[1].to_string().parse().unwrap(),
        );
        let second = Sections::new(
            elements[2].to_string().parse().unwrap(),
            elements[3].to_string().parse().unwrap(),
        );

        overlap += if first.end < second.start || second.end < first.start {
            0
        } else {
            1
        }
    }

    println!("Part1: Total overlaps: {}", overlap);

    Ok(())
}
