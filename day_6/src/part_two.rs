use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::io;

pub fn main_p2() -> Result<(), Box<dyn Error>> {
    let mut lines = io::stdin().lines();

    let line = lines.next().unwrap().unwrap();

    let marker = marker_index(line);

    println!("Marker start at char number {}", marker);

    Ok(())
}

fn marker_index(packet: String) -> u32 {
    let mut chars = packet.chars();
    let mut buffer = VecDeque::<char>::with_capacity(4);
    let mut index = 14;

    for _ in 0..14 {
        buffer.push_back(chars.next().unwrap());
    }

    for c in chars {
        let mut set = HashSet::<char>::new();

        let res = buffer
            .iter()
            .try_for_each(move |c| if set.insert(*c) { Ok(()) } else { Err(()) });

        if res.is_ok() {
            break;
        }

        buffer.push_back(c);
        buffer.pop_front();

        index += 1;
    }

    index
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn marker_after_nineten() {
        let line = String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb");

        assert_eq!(marker_index(line), 19);
    }

    #[test]
    fn marker_after_twentythree() {
        let line = String::from("bvwbjplbgvbhsrlpgdmjqwftvncz");

        assert_eq!(marker_index(line), 23);
    }

    #[test]
    fn marker_after_twentythree_two() {
        let line = String::from("nppdvjthqldpwncqszvftbrmjlhg");

        assert_eq!(marker_index(line), 23);
    }

    #[test]
    fn marker_after_twentysix() {
        let line = String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");

        assert_eq!(marker_index(line), 26);
    }

    #[test]
    fn marker_after_twentynine() {
        let line = String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");

        assert_eq!(marker_index(line), 29);
    }
}
