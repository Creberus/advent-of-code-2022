use std::error::Error;
use std::io;

pub fn main_p1() -> Result<(), Box<dyn Error>> {
    let mut lines = io::stdin().lines();

    let line = lines.next().unwrap().unwrap();

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
