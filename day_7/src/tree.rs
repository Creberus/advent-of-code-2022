pub enum NodeType {
    DIRECTORY,
    FILE,
}

pub trait Node {
    fn name(&self) -> String;
    fn get_type(&self) -> NodeType;
    fn size(&self) -> usize;
}

pub struct Dir {
    name: String,
    childs: Vec<Box<dyn Node>>,
}

impl Dir {
    pub fn new(name: String) -> Self {
        Dir {
            name,
            childs: Vec::new(),
        }
    }
}

pub struct File {
    name: String,
    size: usize,
}

impl File {
    pub fn new(name: String, size: usize) -> Self {
        File { name, size }
    }
}
