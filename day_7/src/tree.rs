pub struct Tree {
    childs: Vec<Box<dyn Node>>,
}

impl Tree {
    pub fn new() -> Self {
        Tree { childs: Vec::new() }
    }

    pub fn add(path: String, node: Box<dyn Node>) -> Result<(), ()> {
        // TODO: Implement add logic
        // node correspond to the full path for a node.
        // Example:
        // For a file: path = "/a/", node = File::new(...)
        // For a dir: path = "/my/sub/", node = Dir::new(...)
        Ok(())
    }
}

pub enum NodeType {
    DIRECTORY,
    FILE,
}

pub trait Node {
    fn name(&self) -> String;
    fn size(&self) -> usize;
    fn get_type(&self) -> NodeType;
    fn add_child(&mut self, child: Box<dyn Node>) -> Result<(), ()>;
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

impl Node for Dir {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn size(&self) -> usize {
        let mut size: usize = 0;

        for node in &self.childs {
            size += node.size()
        }

        size
    }

    fn get_type(&self) -> NodeType {
        NodeType::DIRECTORY
    }

    fn add_child(&mut self, child: Box<dyn Node>) -> Result<(), ()> {
        Ok(())
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

impl Node for File {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn size(&self) -> usize {
        self.size
    }

    fn get_type(&self) -> NodeType {
        NodeType::FILE
    }

    fn add_child(&mut self, child: Box<dyn Node>) -> Result<(), ()> {
        Err(())
    }
}
