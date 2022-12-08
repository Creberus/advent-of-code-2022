use std::error::Error;
use std::io;
use std::path::PathBuf;

use crate::cwd::CurrentWorkingDirectory;
use crate::tree::{Dir, File, Node, Tree};

pub fn main_p1() -> Result<(), Box<dyn Error>> {
    let lines = io::stdin().lines();
    let mut cwd = CurrentWorkingDirectory::new();
    let mut fs = Tree::new();

    for line in lines {
        let line = line.unwrap();

        if line.starts_with("$") {
            // Command parsing
            let command: Vec<&str> = line.split(' ').collect();
            match command[1] {
                "cd" => {
                    cwd.mv(command[2])?;
                    println!("Updated CurrentWorkingDirectory: {}", cwd);
                }
                _ => (),
            };
        } else {
            // Directory and File reading from `ls`
            let info: Vec<&str> = line.split(' ').collect();

            match info[0] {
                "dir" => {
                    let dir = Dir::new(String::from(info[1]));
                    println!("Creating {}", dir);
                    let path = PathBuf::from(cwd.get());
                    fs.add(&mut path.components(), Box::new(dir))?;
                }
                _ => {
                    let file = File::new(String::from(info[1]), info[0].parse().unwrap());
                    println!("Creating {}", file);
                    let path = PathBuf::from(cwd.get());
                    fs.add(&mut path.components(), Box::new(file))?;
                }
            }
        }
    }

    println!("Printing File System:");
    println!("{}", fs);

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
