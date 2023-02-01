use std::error::Error;
use std::io;

pub fn main_p2() -> Result<(), Box<dyn Error>> {
    let lines = io::stdin().lines();

    let decryption_key = 811_589_153;
    let mut numbers = Vec::<i64>::new();

    for line in lines {
        let line = line.unwrap();

        numbers.push(line.parse().unwrap());
    }

    let mut numbers = apply_encryption_key(numbers, decryption_key);

    println!("Initial arrangement:\n{:?}", numbers);

    for round in 1..11 {
        for instruction in 0..numbers.len() {
            apply_instruction(instruction, &mut numbers);
        }
        println!("After {} rounds of mixing:\n{:?}", round, numbers);
    }

    let (index, _) = numbers.iter().enumerate().find(|&n| (*n.1).1 == 0).unwrap();

    let index_1000 = (index + 1000) % numbers.len();
    let index_2000 = (index + 2000) % numbers.len();
    let index_3000 = (index + 3000) % numbers.len();

    let number_1000 = numbers.get(index_1000).unwrap();
    let number_2000 = numbers.get(index_2000).unwrap();
    let number_3000 = numbers.get(index_3000).unwrap();

    println!(
        "1000 {} 2000 {} 3000 {}",
        number_1000.1, number_2000.1, number_3000.1
    );
    println!("Sum: {}", number_1000.1 + number_2000.1 + number_3000.1);

    Ok(())
}

fn apply_instruction(instruction: usize, numbers: &mut Vec<(usize, i64)>) {
    // 1. Find the number in the array as well as the index
    let (index, number) = numbers
        .iter()
        .enumerate()
        .find(|&n| (n.1).0 == instruction)
        .unwrap();

    let number = number.1;

    numbers.remove(index);

    let mut new_index = index as i64 + number;

    new_index %= numbers.len() as i64;

    if new_index <= 0 {
        new_index += numbers.len() as i64;
    }

    assert!(new_index > 0);
    numbers.insert(new_index as usize, (instruction, number));
}

fn apply_encryption_key(arr: Vec<i64>, decryption_key: i64) -> Vec<(usize, i64)> {
    arr.iter()
        .enumerate()
        .map(|n| (n.0, n.1 * decryption_key))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    static DECRYPTION_KEY: i64 = 811_589_153;

    #[test]
    fn simple() {
        let mut numbers = vec![1, 2, -3, 3, -2, 0, 4];
        let mut numbers = apply_encryption_key(numbers, DECRYPTION_KEY);

        for instruction in 1..numbers.len() {}
    }

    /*
    #[test]
    fn zero() {
        let mut numbers = vec![1, 2, -3, 3, -2, 0, 4];
        let mut numbers = apply_encryption_key(numbers, decryption_key)

        apply_instruction(0, &mut numbers);

        assert_eq!(numbers, vec![1, 2, -3, 3, -2, 0, 4]);
    }

    #[test]
    fn move_forward_to_end() {
        let mut numbers = vec![1, -2, -3, 3, 2, 0, 4];
        let size = numbers.len() as i32;

        apply_instruction(2, &mut numbers);

        assert_eq!(numbers, vec![1, -2, -3, 3, 0, 4, 2]);
    }*/
}
