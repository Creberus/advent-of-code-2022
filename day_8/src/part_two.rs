use std::error::Error;
use std::fmt::Display;
use std::io;

pub fn main_p2() -> Result<(), Box<dyn Error>> {
    let lines = io::stdin().lines();

    let mut index = 0;
    let mut matrix = Vec::<Vec<Tree>>::new();

    // 1. Create a 2D array with all the values
    for line in lines {
        let line = line.unwrap();
        matrix.push(Vec::new());

        for c in line.chars() {
            matrix[index].push(Tree::new(c.to_digit(10).unwrap() as u8));
        }

        index += 1;
    }

    // 2. Set edge trees
    let mut r = 0;
    let matrix_len = matrix.len();
    let row_len = matrix[0].len();
    for row in &mut matrix {
        let mut c = 0;
        for col in row {
            if r == 0 || r == (matrix_len - 1) || c == 0 || c == (row_len - 1) {
                col.set_edge();
            }
            c += 1;
        }
        r += 1;
    }

    for row in 1..(matrix.len() - 1) {
        for col in 1..(matrix[row].len() - 1) {
            let height = matrix[row][col].get_height();

            // Check looking up
            let mut visible_up = 1;
            let mut up = row - 1;
            while !matrix[up][col].is_edge() && matrix[up][col].get_height() < height {
                visible_up += 1;
                up -= 1;
            }

            // Check looking right
            let mut visible_right = 1;
            let mut right = col + 1;
            while !matrix[row][right].is_edge() && matrix[row][right].get_height() < height {
                visible_right += 1;
                right += 1;
            }

            // Check looking down
            let mut visible_down = 1;
            let mut down = row + 1;
            while !matrix[down][col].is_edge() && matrix[down][col].get_height() < height {
                visible_down += 1;
                down += 1;
            }

            // Check looking left
            let mut visible_left = 1;
            let mut left = col - 1;
            while !matrix[row][left].is_edge() && matrix[row][left].get_height() < height {
                visible_left += 1;
                left -= 1;
            }

            matrix[row][col]
                .set_scenic_score(visible_up * visible_right * visible_down * visible_left);
        }
    }

    let mut max_scenic_score = u32::MIN;

    for row in matrix {
        let cur_max_scenic_score = row
            .iter()
            .max_by_key(|tree| tree.get_scenic_score())
            .unwrap()
            .get_scenic_score();

        max_scenic_score = max_scenic_score.max(cur_max_scenic_score);
    }

    println!("Max scenic_score: {}", max_scenic_score);

    Ok(())
}

pub struct Tree {
    height: u8,
    edge: bool,
    scenic_score: u32,
}

impl Tree {
    pub fn new(height: u8) -> Self {
        Tree {
            height,
            edge: false,
            scenic_score: 0,
        }
    }

    pub fn set_edge(&mut self) {
        self.edge = true;
    }

    pub fn is_edge(&self) -> bool {
        self.edge
    }

    pub fn get_height(&self) -> u8 {
        self.height
    }

    pub fn set_scenic_score(&mut self, scenic_score: u32) {
        self.scenic_score = scenic_score;
    }

    pub fn get_scenic_score(&self) -> u32 {
        self.scenic_score
    }
}

impl Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{height:{}}}", self.height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {}
}
