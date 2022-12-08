use std::error::Error;
use std::io;

pub fn main_p1() -> Result<(), Box<dyn Error>> {
    let lines = io::stdin().lines();

    let mut index = 0;
    let mut matrix = Vec::<Vec<u8>>::new();

    // 1. Create a 2D array with all the values
    for line in lines {
        let line = line.unwrap();
        matrix.push(Vec::new());

        for c in line.chars() {
            matrix[index].push(c.to_digit(10).unwrap() as u8);
        }

        index += 1;
    }

    for row in matrix {
        for col in row {
            print!("{}", char::from_digit(col as u32, 10).unwrap());
        }
        println!("");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {}
}
