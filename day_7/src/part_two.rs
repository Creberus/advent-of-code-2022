use std::error::Error;
use std::io;
use std::path::PathBuf;

use crate::cwd::CurrentWorkingDirectory;
use crate::tree::{
    Dir, File, Node, Tree, TreeDisplay, TreeMaxDirSize, TreeSizeVisitor, TreeVisitor,
};

pub fn main_p2() -> Result<(), Box<dyn Error>> {
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

    let mut visitor = TreeDisplay::new();
    visitor.visit_tree(&fs);

    let mut size_visitor = TreeMaxDirSize::new(0);
    let total_size = size_visitor.visit_tree(&fs);

    println!("Total size: {}", total_size);
    let free_size = 70_000_000 - total_size;
    println!("Free size: {}", free_size);
    let size_to_free = 30_000_000 - free_size;
    println!("Size to free: {}", size_to_free);

    size_visitor = TreeMaxDirSize::new(size_to_free);
    size_visitor.visit_tree(&fs);
    println!("Size freed: {}", size_visitor.dir_size);

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
