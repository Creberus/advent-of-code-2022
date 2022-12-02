use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let lines = io::stdin().lines();

    let mut elves = Vec::<u32>::new();
    elves.push(0);

    for line in lines {
        let l = line.unwrap();

        if l.is_empty() {
            elves.push(0);
        } else {
            if let Some(last) = elves.last_mut() {
                *last += l.parse::<u32>().unwrap();
            }
        }
    }

    println!("Max: {:?}", elves.iter().max());

    elves.sort();
    let (_, right) = elves.split_at(elves.len() - 3);

    let maxs: u32 = right.iter().sum();

    println!("Max 3: {:?}", maxs);

    Ok(())
}
