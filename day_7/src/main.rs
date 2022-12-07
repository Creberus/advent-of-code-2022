mod cwd;
mod part_one;
mod tree;

use part_one::main_p1;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    main_p1()
}
