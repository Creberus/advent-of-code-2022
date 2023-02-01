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

    // 2. Compute the new position for the number
    let mut new_index = index as i32 + instruction;

    // 3. While the new index is negative, take the number of elements and remove the offset
    while new_index < 0 {
        new_index = size - new_index;
    }

    // 4. Set the index in range of the array
    new_index %= size;

    // 5 Now we have 3 possibilities
    // 5.a The new_index is after index
    if (new_index as usize) > index {
        let mut current_index = index;

        while current_index < index {
            *numbers.get_mut(current_index).unwrap() = *numbers.get(current_index + 1).unwrap();
            current_index += 1;
        }

        *numbers.get_mut(current_index).unwrap() = instruction;
    }
    // 5.b The new_index is before index
    else if (new_index as usize) < index {
        let mut current_index = index;

        while current_index > index {
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

        //assert_eq!(numbers, vec![1, 2, -3, 4, 0, 3, -2]);
    }
}
