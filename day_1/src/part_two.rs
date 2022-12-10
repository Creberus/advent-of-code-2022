use std::collections::BinaryHeap;
use std::error::Error;
use std::io::{self, BufRead, Lines};

pub fn main_p2() -> Result<(), Box<dyn Error>> {
    let mut lines = io::stdin().lines();

    let calories = parse_max_calories(&mut lines);

    assert_eq!(calories.len(), 3);
    println!("Max calories: {}", calories.iter().sum::<u32>());

    Ok(())
}

fn parse_max_calories<B: BufRead>(lines: &mut Lines<B>) -> Vec<u32> {
    let mut heap = BinaryHeap::<u32>::new();

    loop {
        let calories = parse_calories(lines);
        heap.push(calories);

        if calories == 0 {
            break; // EOF
        }
    }

    let mut max_calories = Vec::new();

    for _ in 0..3 {
        max_calories.push(heap.pop().unwrap());
    }

    max_calories
}

fn parse_calories<B: BufRead>(lines: &mut Lines<B>) -> u32 {
    let mut calories: u32 = 0;

    for line in lines {
        let line = line.unwrap();

        if line.is_empty() {
            break; // Empty line between two elves
        }

        calories += line.parse::<u32>().unwrap();
    }

    calories
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_calory() {
        let zero = String::from("0");
        let line = io::IoSlice::new(zero.as_bytes());

        assert_eq!(parse_calories(&mut line.lines()), 0);
    }

    #[test]
    fn one_thousand_calory() {
        let calory = String::from("1000");
        let line = io::IoSlice::new(calory.as_bytes());

        assert_eq!(parse_calories(&mut line.lines()), 1000);
    }

    #[test]
    fn one_hundred_calories() {
        let calories = String::from("20\n50\n30");
        let line = io::IoSlice::new(calories.as_bytes());

        assert_eq!(parse_calories(&mut line.lines()), 100);
    }

    #[test]
    fn two_elves_carrying_same_calories() {
        let calories = String::from("20\n50\n30\n\n40\n60");
        let line = io::IoSlice::new(calories.as_bytes());

        assert_eq!(parse_max_calories(&mut line.lines()), 100);
    }

    #[test]
    fn two_elves_second_carrying_more_calories() {
        let calories = String::from("20\n50\n30\n\n40\n60\n10");
        let line = io::IoSlice::new(calories.as_bytes());

        assert_eq!(parse_max_calories(&mut line.lines()), 110);
    }

    #[test]
    fn three_elves_first_carrying_more_calories() {
        let calories = String::from("20\n50\n30\n\n40\n\n10");
        let line = io::IoSlice::new(calories.as_bytes());

        assert_eq!(parse_max_calories(&mut line.lines()), 100);
    }
}
