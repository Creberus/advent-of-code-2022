use std::{
    error::Error,
    fmt::{Debug, Display},
    path::Components,
};

#[derive(Debug)]
pub struct Tree {
    root: Box<dyn Node>,
}

impl Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.root)
    }
}

impl Node for Tree {
    fn name(&self) -> String {
        String::from("/")
    }

    fn size(&self) -> usize {
        self.root.size()
    }

    fn get_type(&self) -> NodeType {
        NodeType::DIRECTORY
    }

    fn add_child(&mut self, _: Box<dyn Node>) -> Result<(), TreeError> {
        Err(TreeError::new())
    }

    fn add(&mut self, components: &mut Components, node: Box<dyn Node>) -> Result<(), TreeError> {
        components.next(); // Skip "/"
        self.root.add(components, node)
    }

    fn visit(&self, visitor: &mut dyn TreeVisitor) {
        visitor.visit(&self.root)
    }

    fn visit_size(&self, visitor: &mut dyn TreeSizeVisitor) -> usize {
        visitor.visit(&self.root)
    }
}

impl Tree {
    pub fn new() -> Self {
        Tree {
            root: Box::new(Dir::new(String::from("/"))),
        }
    }
}

#[derive(Debug)]
pub struct TreeError {}

impl TreeError {
    pub fn new() -> Self {
        TreeError {}
    }
}

impl Display for TreeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", 0)
    }
}

impl Error for TreeError {}

#[derive(Debug)]
pub enum NodeType {
    DIRECTORY,
    FILE,
}

pub trait Node: Display {
    fn name(&self) -> String;
    fn size(&self) -> usize;
    fn get_type(&self) -> NodeType;
    fn add(&mut self, components: &mut Components, node: Box<dyn Node>) -> Result<(), TreeError>;
    fn add_child(&mut self, child: Box<dyn Node>) -> Result<(), TreeError>;
    fn visit(&self, visitor: &mut dyn TreeVisitor);
    fn visit_size(&self, visitor: &mut dyn TreeSizeVisitor) -> usize;
}

impl Debug for dyn Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node{{{}}}", self.name())
    }
}

#[derive(Debug)]
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

impl Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Dir {}", self.name)?;

        /*
        for child in &self.childs {
            write!(f, "{}", child)?;
        }*/

        Ok(())
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

    fn add(&mut self, components: &mut Components, node: Box<dyn Node>) -> Result<(), TreeError> {
        let component = components.next();

        match component {
            Some(path) => {
                for child in &mut self.childs {
                    if child.name() == path.as_os_str().to_str().unwrap() {
                        return child.add(components, node);
                    }
                }

                Err(TreeError::new())
            }
            None => self.add_child(node),
        }
    }

    fn add_child(&mut self, child: Box<dyn Node>) -> Result<(), TreeError> {
        Ok(self.childs.push(child))
    }

    fn visit(&self, visitor: &mut dyn TreeVisitor) {
        visitor.visit_dir(self)
    }

    fn visit_size(&self, visitor: &mut dyn TreeSizeVisitor) -> usize {
        visitor.visit_dir(self)
    }
}

#[derive(Debug)]
pub struct File {
    name: String,
    size: usize,
}

impl File {
    pub fn new(name: String, size: usize) -> Self {
        File { name, size }
    }
}

impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "File {}", self.name)
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

    fn add(&mut self, _: &mut Components, _: Box<dyn Node>) -> Result<(), TreeError> {
        Err(TreeError::new())
    }

    fn add_child(&mut self, _: Box<dyn Node>) -> Result<(), TreeError> {
        Err(TreeError::new())
    }

    fn visit(&self, visitor: &mut dyn TreeVisitor) {
        visitor.visit_file(self)
    }

    fn visit_size(&self, visitor: &mut dyn TreeSizeVisitor) -> usize {
        visitor.visit_file(self)
    }
}

pub struct TreeDisplay {
    indent: u32,
}

impl TreeDisplay {
    pub fn new() -> Self {
        TreeDisplay { indent: 0 }
    }

    pub fn print_ident(&self) {
        for _ in 0..self.indent {
            print!(" ");
        }
    }
}

pub trait TreeVisitor {
    fn visit_tree(&mut self, t: &Tree);
    fn visit(&mut self, t: &Box<dyn Node>);
    fn visit_file(&mut self, f: &File);
    fn visit_dir(&mut self, d: &Dir);
}

impl TreeVisitor for TreeDisplay {
    fn visit_tree(&mut self, t: &Tree) {
        self.visit(&t.root)
    }

    fn visit(&mut self, t: &Box<dyn Node>) {
        t.visit(self)
    }

    fn visit_file(&mut self, f: &File) {
        self.print_ident();
        println!("{}", f);
    }

    fn visit_dir(&mut self, d: &Dir) {
        self.print_ident();
        println!("{}", d);

        self.indent += 1;

        for child in &d.childs {
            self.visit(child);
        }

        self.indent -= 1;
    }
}

const MAX_DIR_SIZE: usize = 100_000;

pub struct TreeMaxDirSize {
    pub size: usize,
}

impl TreeMaxDirSize {
    pub fn new() -> Self {
        TreeMaxDirSize { size: 0 }
    }
}

pub trait TreeSizeVisitor {
    fn visit_tree(&mut self, t: &Tree) -> usize;
    fn visit(&mut self, t: &Box<dyn Node>) -> usize;
    fn visit_file(&mut self, f: &File) -> usize;
    fn visit_dir(&mut self, d: &Dir) -> usize;
}

impl TreeSizeVisitor for TreeMaxDirSize {
    fn visit_tree(&mut self, t: &Tree) -> usize {
        self.visit(&t.root)
    }

    fn visit(&mut self, t: &Box<dyn Node>) -> usize {
        t.visit_size(self)
    }

    fn visit_file(&mut self, f: &File) -> usize {
        f.size()
    }

    fn visit_dir(&mut self, d: &Dir) -> usize {
        let mut size: usize = 0;

        for child in &d.childs {
            size += self.visit(child);
        }

        if size < MAX_DIR_SIZE {
            self.size += size;
        }

        size
    }
}
