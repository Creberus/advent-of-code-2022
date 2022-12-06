use std::collections::VecDeque;
use std::error::Error;
use std::io;

pub fn main_p1() -> Result<(), Box<dyn Error>> {
    let mut lines = io::stdin().lines();

    let line = lines.next().unwrap().unwrap();

    let marker = marker_index(line);

    println!("Marker start at char number {}", marker);

    Ok(())
}

fn marker_index(packet: String) -> u32 {
    let mut chars = packet.chars();
    let mut buffer = VecDeque::<char>::with_capacity(4);
    let mut index = 4;

    buffer.push_back(chars.next().unwrap());
    buffer.push_back(chars.next().unwrap());
    buffer.push_back(chars.next().unwrap());
    buffer.push_back(chars.next().unwrap());

    for c in chars {
        // Check if 4 chars aren't the same
        if buffer.get(0) != buffer.get(1)
            && buffer.get(0) != buffer.get(2)
            && buffer.get(0) != buffer.get(3)
            && buffer.get(1) != buffer.get(2)
            && buffer.get(1) != buffer.get(3)
            && buffer.get(2) != buffer.get(3)
        {
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
    fn first_four_is_marker() {
        let line = String::from("jlmp");

        assert_eq!(marker_index(line), 4);
    }

    #[test]
    fn marker_after_five() {
        let line = String::from("bvwbjplbgvbhsrlpgdmjqwftvncz");

        assert_eq!(marker_index(line), 5);
    }

    #[test]
    fn marker_after_six() {
        let line = String::from("nppdvjthqldpwncqszvftbrmjlhg");

        assert_eq!(marker_index(line), 6);
    }

    #[test]
    fn marker_after_ten() {
        let line = String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");

        assert_eq!(marker_index(line), 10);
    }

    #[test]
    fn marker_after_eleven() {
        let line = String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");

        assert_eq!(marker_index(line), 11);
    }
}
