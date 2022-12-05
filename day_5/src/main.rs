mod part_one;
use part_one::main_p1;

mod part_two;
use part_two::main_p2;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    //main_p1()
    main_p2()
}
