use std::error::Error;
use std::io;

pub fn main_p1() -> Result<(), Box<dyn Error>> {
    let lines = io::stdin().lines();

    let mut numbers = Vec::<i32>::new();

    for line in lines {
        let line = line.unwrap();

        numbers.push(line.parse().unwrap());
    }

    let instructions = numbers.clone();

    for instruction in &instructions {
        apply_instruction(*instruction, instructions.len() as i32, &mut numbers);
    }

    println!("{:?}", numbers);

    Ok(())
}

fn apply_instruction(instruction: i32, size: i32, numbers: &mut Vec<i32>) {
    // 1. Find the number in the array as well as the index
    let (index, _) = numbers
        .iter()
        .enumerate()
        .find(|&n| *n.1 == instruction)
        .unwrap();

    let index = index as i32;

    // 2. Compute the new position for the number
    let mut new_index = index + instruction;

    // 3. If the new_index is negatif, compute the same but in positive
    if new_index < 0 {
        new_index += size - 1;
    } else if new_index == 0 {
        new_index = size - 1;
    }

    // 4. Set the index in range of the array
    if new_index >= size {
        new_index %= size;
        new_index += 1;
    }

    // 5 Now we have 3 possibilities
    // 5.a The new_index is after index
    if new_index > index {
        let mut current_index = index as usize;

        while current_index < new_index as usize {
            *numbers.get_mut(current_index).unwrap() = *numbers.get(current_index + 1).unwrap();
            current_index += 1;
        }

        *numbers.get_mut(current_index).unwrap() = instruction;
    }
    // 5.b The new_index is before index
    else if new_index < index {
        let mut current_index = index as usize;

        while current_index > new_index as usize {
            *numbers.get_mut(current_index).unwrap() = *numbers.get(current_index - 1).unwrap();
            current_index -= 1;
        }

        *numbers.get_mut(current_index).unwrap() = instruction;
    }
    // 5.c The new_index is equal to index
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let mut numbers = vec![1, 2, -3, 3, -2, 0, 4];
        let size = numbers.len() as i32;

        apply_instruction(1, size, &mut numbers);
        assert_eq!(numbers, vec![2, 1, -3, 3, -2, 0, 4]);

        apply_instruction(2, size, &mut numbers);
        assert_eq!(numbers, vec![1, -3, 2, 3, -2, 0, 4]);

        apply_instruction(-3, size, &mut numbers);
        assert_eq!(numbers, vec![1, 2, 3, -2, -3, 0, 4]);

        apply_instruction(3, size, &mut numbers);
        assert_eq!(numbers, vec![1, 2, -2, -3, 0, 3, 4]);

        apply_instruction(-2, size, &mut numbers);
        assert_eq!(numbers, vec![1, 2, -3, 0, 3, 4, -2]);

        apply_instruction(0, size, &mut numbers);
        assert_eq!(numbers, vec![1, 2, -3, 0, 3, 4, -2]);

        apply_instruction(4, size, &mut numbers);
        assert_eq!(numbers, vec![1, 2, -3, 4, 0, 3, -2]);
    }
}
