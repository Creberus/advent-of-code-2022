use std::error::Error;
use std::fmt::Display;
use std::io;

pub fn main_p1() -> Result<(), Box<dyn Error>> {
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
            let mut visible_up = true;
            let mut up = row - 1;
            while visible_up {
                visible_up = matrix[up][col].get_height() < height;
                if up == 0 {
                    break;
                }
                up -= 1;
            }

            // Check looking right
            let mut visible_right = true;
            let mut right = col + 1;
            while visible_right {
                visible_right = matrix[row][right].get_height() < height;
                if right == matrix[row].len() - 1 {
                    break;
                }
                right += 1;
            }

            // Check looking down
            let mut visible_down = true;
            let mut down = row + 1;
            while visible_down {
                visible_down = matrix[down][col].get_height() < height;
                if down == matrix.len() - 1 {
                    break;
                }
                down += 1;
            }

            // Check looking left
            let mut visible_left = true;
            let mut left = col - 1;
            while visible_left {
                visible_left = matrix[row][left].get_height() < height;
                if left == 0 {
                    break;
                }
                left -= 1;
            }

            matrix[row][col]
                .set_visibility(visible_up || visible_right || visible_down || visible_left);
        }
    }

    let number_of_tree_visible = matrix.iter().fold(0, |acc, row| {
        acc + row
            .iter()
            .fold(0, |acc, tree| acc + if tree.is_visible() { 1 } else { 0 })
    });

    println!("Number of tree visible: {}", number_of_tree_visible);

    Ok(())
}

pub struct Tree {
    height: u8,
    visible: bool,
    edge: bool,
}

impl Tree {
    pub fn new(height: u8) -> Self {
        Tree {
            height,
            visible: false,
            edge: false,
        }
    }

    pub fn set_edge(&mut self) {
        self.edge = true;
        self.visible = true;
    }

    pub fn is_edge(&self) -> bool {
        self.edge
    }

    pub fn get_height(&self) -> u8 {
        self.height
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn set_visibility(&mut self, visible: bool) {
        self.visible = visible;
    }
}

impl Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{height:{}, visible{}}}", self.height, self.visible)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {}
}
