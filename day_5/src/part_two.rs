use std::error::Error;
use std::io;

pub fn main_p2() -> Result<(), Box<dyn Error>> {
    let lines = io::stdin().lines();
    let mut containers = Vec::<Vec<char>>::new();
    let mut lines_it = lines.into_iter();

    // 1. Parse cargo containers
    loop {
        let line = lines_it.next().unwrap().unwrap();
        if line.is_empty() {
            break; // We have finished parsing the cargo
        }

        let mut it = line.chars();

        let mut row = Vec::<char>::new();
        loop {
            if it.next().is_none() {
                // '\n'
                break;
            }
            // '['
            let value = it.next();
            let value = value.unwrap();
            if value.is_ascii_uppercase() {
                row.push(value);
            } else if value.is_ascii_digit() {
                break;
            } else {
                row.push('\0');
            }
            it.next(); // ']'
            it.next(); // ' '
        }

        if containers.is_empty() {
            for _ in 0..row.len() {
                containers.push(Vec::new());
            }
        }

        for i in 0..row.len() {
            if row[i] != '\0' {
                containers[i].push(row[i]);
            }
        }
    }

    // 1.5 Reverse the containers
    for i in 0..containers.len() {
        containers[i].reverse();
    }

    // 2. Parse the move
    loop {
        let line = lines_it.next();
        if line.is_none() {
            break; // EOF
        }

        let line = line.unwrap().unwrap();
        let data: Vec<&str> = line.split(' ').collect();

        let moves: u32 = data[1].parse().unwrap();
        let source: usize = data[3].parse().unwrap();
        let dest: usize = data[5].parse().unwrap();

        println!("Perform {moves} moves from {source} to {dest}");

        // 3. Do the move
        let mut crane = Vec::<char>::new();
        for _ in 0..moves {
            let value = containers[source - 1].pop().unwrap();
            crane.push(value);
        }

        for _ in 0..moves {
            let value = crane.pop().unwrap();
            containers[dest - 1].push(value);
        }
    }

    for i in 0..containers.len() {
        print!("{}", containers[i].pop().unwrap());
    }
    println!("");

    Ok(())
}
