use std::collections::VecDeque;
use std::error::Error;
use std::io;

pub fn main_p1() -> Result<(), Box<dyn Error>> {
    let lines = io::stdin().lines();

    let mut start: (usize, usize) = (0, 0);
    let mut mat = Vec::<Vec<HeightNode>>::new();

    let mut row_idx = 0;
    for line in lines {
        let mut row = Vec::new();
        let mut col_idx = 0;

        let line = line.unwrap();

        for c in line.chars() {
            if c == 'S' {
                start = (col_idx, row_idx);
            }
            row.push(HeightNode::new(col_idx, row_idx, c));
            col_idx += 1;
        }

        mat.push(row);
        row_idx += 1;
    }

    let map = HeightMap::new(mat);
    let mut path = FindShortestPath::new(&map);

    let min_path = path.visit(start);

    println!("{:?}", min_path);

    Ok(())
}

#[derive(Debug)]
struct HeightNode {
    x: usize,
    y: usize,
    value: char,
}

impl HeightNode {
    fn new(x: usize, y: usize, value: char) -> Self {
        HeightNode { x, y, value }
    }

    fn value(&self) -> char {
        self.value
    }

    fn x(&self) -> usize {
        self.x
    }

    fn y(&self) -> usize {
        self.y
    }
}

#[derive(Debug)]
struct HeightMap {
    map: Vec<Vec<HeightNode>>,
    height: usize,
    width: usize,
}

impl HeightMap {
    fn new(map: Vec<Vec<HeightNode>>) -> Self {
        let height = map.len();
        assert!(height > 0);
        let width = map[0].len();

        HeightMap { map, height, width }
    }

    fn height(&self) -> usize {
        self.height
    }

    fn width(&self) -> usize {
        self.width
    }

    fn get(&self, x: usize, y: usize) -> Option<&HeightNode> {
        if x >= self.width || y >= self.height {
            None
        } else {
            Some(&self.map[y][x])
        }
    }
}

trait Visitor {
    fn visit(&mut self, start: (usize, usize)) -> Result<u32, ()>;
}

#[derive(Debug)]
struct FindShortestPath<'a> {
    map: &'a HeightMap,
    visited: Vec<Vec<bool>>,
}

impl<'a> FindShortestPath<'a> {
    fn new(map: &'a HeightMap) -> Self {
        let mut visited = Vec::new();

        for row in 0..map.height() {
            visited.push(Vec::new());
            for _ in 0..map.width() {
                visited[row].push(false);
            }
        }

        FindShortestPath { map, visited }
    }
}

impl<'a> Visitor for FindShortestPath<'a> {
    fn visit(&mut self, start: (usize, usize)) -> Result<u32, ()> {
        let mut q = VecDeque::<&HeightNode>::new();
        let node = self.map.get(start.0, start.1).unwrap();
        let mut depth = 1;
        let mut node_left = 1;

        self.visited[node.y()][node.x()] = true;
        q.push_back(node);

        while !q.is_empty() {
            let node = q.pop_front().unwrap();

            if node.value() == 'E' {
                break;
            }

            let value = if node.value() == 'S' {
                'a'
            } else {
                node.value()
            };

            // Check Up node
            if node.y() != 0 {
                let up = (node.x(), node.y() - 1);
                if let Some(n) = self.map.get(up.0, up.1) {
                    if !self.visited[up.1][up.0]
                        && ((value as u32) + 1 >= (n.value() as u32) || n.value() == 'E')
                    {
                        self.visited[up.1][up.0] = true;
                        q.push_back(&n);
                    }
                }
            }
            // Check Right node
            let right = (node.x() + 1, node.y());
            if let Some(n) = self.map.get(right.0, right.1) {
                if !self.visited[right.1][right.0]
                    && ((value as u32) + 1 >= (n.value() as u32) || n.value() == 'E')
                {
                    self.visited[right.1][right.0] = true;
                    q.push_back(&n);
                }
            }

            // Check Down node
            let down = (node.x(), node.y() + 1);
            if let Some(n) = self.map.get(down.0, down.1) {
                if !self.visited[down.1][down.0]
                    && ((value as u32) + 1 >= (n.value() as u32) || n.value() == 'E')
                {
                    self.visited[down.1][down.0] = true;
                    q.push_back(&n);
                }
            }
            // Check Left node
            if node.x() != 0 {
                let left = (node.x() - 1, node.y());
                if let Some(n) = self.map.get(left.0, left.1) {
                    if !self.visited[left.1][left.0]
                        && ((value as u32) + 1 >= (n.value() as u32) || n.value() == 'E')
                    {
                        self.visited[left.1][left.0] = true;
                        q.push_back(&n);
                    }
                }
            }

            node_left -= 1;
            if node_left == 0 {
                depth += 1;
                node_left = q.len();
            }
        }

        return Ok(depth);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line() {
        let mut mat = Vec::<Vec<HeightNode>>::new();

        mat.push(Vec::new());

        mat[0].push(HeightNode::new(0, 0, 'S'));
        for c in 1..26 {
            mat[0].push(HeightNode::new(
                c,
                0,
                char::from_u32((97 + c).try_into().unwrap()).unwrap(),
            ));
        }
        mat[0].push(HeightNode::new(26, 0, 'E'));

        let map = HeightMap::new(mat);

        let mut visitor = FindShortestPath::new(&map);

        let path = visitor.visit((0, 0));

        println!("{:?}", visitor);
        assert_eq!(path, Ok(27));
    }
}
