mod part_one;
mod part_two;
use std::error::Error;

use part_one::main_p1;
use part_two::main_p2;

fn main() -> Result<(), Box<dyn Error>> {
    // main_p1()
    main_p2()
}
