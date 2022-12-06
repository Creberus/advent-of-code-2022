use std::error::Error;

mod part_one;
use part_one::main_p1;

fn main() -> Result<(), Box<dyn Error>> {
    main_p1()
}
