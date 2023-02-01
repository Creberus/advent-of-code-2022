use std::error::Error;
use std::io;

pub fn main_p1() -> Result<(), Box<dyn Error>> {
    let lines = io::stdin().lines();

    let mut numbers = Vec::<(i32, bool)>::new();

    for line in lines {
        let line = line.unwrap();

        numbers.push((line.parse().unwrap(), false));
    }

    let instructions = numbers.clone();

    for instruction in &instructions {
        apply_instruction(instruction.0, &mut numbers);
    }

    let (index, _) = numbers.iter().enumerate().find(|&n| (*n.1).0 == 0).unwrap();

    let index_1000 = (index + 1000) % numbers.len();
    let index_2000 = (index + 2000) % numbers.len();
    let index_3000 = (index + 3000) % numbers.len();

    let number_1000 = numbers.get(index_1000).unwrap();
    let number_2000 = numbers.get(index_2000).unwrap();
    let number_3000 = numbers.get(index_3000).unwrap();

    println!(
        "1000 {} 2000 {} 3000 {}",
        number_1000.0, number_2000.0, number_3000.0
    );
    println!("Sum: {}", number_1000.0 + number_2000.0 + number_3000.0);

    Ok(())
}

fn apply_instruction(instruction: i32, numbers: &mut Vec<(i32, bool)>) {
    // 1. Find the number in the array as well as the index
    let (index, _) = numbers
        .iter()
        .enumerate()
        .find(|&n| !(*n.1).1 && (*n.1).0 == instruction)
        .unwrap();

    numbers.remove(index);

    let mut new_index = index as i32 + instruction;

    new_index %= numbers.len() as i32;

    if new_index <= 0 {
        new_index += numbers.len() as i32;
    }

    assert!(new_index > 0);
    numbers.insert(new_index as usize, (instruction, true));
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
    #[test]
    fn simple() {
        let mut numbers = vec![1, 2, -3, 3, -2, 0, 4];

        apply_instruction(1, &mut numbers);
        assert_eq!(numbers, vec![2, 1, -3, 3, -2, 0, 4]);

        apply_instruction(2, &mut numbers);
        assert_eq!(numbers, vec![1, -3, 2, 3, -2, 0, 4]);

        apply_instruction(-3, &mut numbers);
        assert_eq!(numbers, vec![1, 2, 3, -2, -3, 0, 4]);

        apply_instruction(3, &mut numbers);
        assert_eq!(numbers, vec![1, 2, -2, -3, 0, 3, 4]);

        apply_instruction(-2, &mut numbers);
        assert_eq!(numbers, vec![1, 2, -3, 0, 3, 4, -2]);

        apply_instruction(0, &mut numbers);
        assert_eq!(numbers, vec![1, 2, -3, 0, 3, 4, -2]);

        apply_instruction(4, &mut numbers);
        assert_eq!(numbers, vec![1, 2, -3, 4, 0, 3, -2]);
    }

    #[test]
    fn zero() {
        let mut numbers = vec![1, 2, -3, 3, -2, 0, 4];
        let size = numbers.len() as i32;

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
