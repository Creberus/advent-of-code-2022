use std::error::Error;
use std::io;

pub fn main_p1() -> Result<(), Box<dyn Error>> {
    let mut lines = io::stdin().lines();

    for line in lines {
        let line = line.unwrap();

        if line.starts_with("$") {
            // Command parsing
            let command: Vec<&str> = line.split(' ').collect();
            match command[0] {
                "cd" => (), //TODO: Change current working directory
                _ => (),
            };
        } else {
            // Directory and File reading from `ls`
            let info: Vec<&str> = line.split(' ').collect();

            match info[0] {
                "dir" => (), //TODO: Create Dir
                _ => (),     //TODO: Create File
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let line = String::from("jlmp");
    }
}
