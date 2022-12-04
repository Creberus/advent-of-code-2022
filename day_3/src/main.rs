use std::error::Error;

mod part1;
use part1::main_p1;

mod part2;
use part2::main_p2;

fn main() -> Result<(), Box<dyn Error>> {
    //main_p1()
    main_p2()
}
