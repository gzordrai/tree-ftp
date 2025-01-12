use super::{node::{Node, NodeEnum}};

#[derive(Debug)]
pub struct Directory {
    pub name: String,
    pub nodes: Vec<NodeEnum>, // Use NodeEnum to store both Directory and File
}

impl Directory {
    pub fn new(name: String) -> Self {
        Directory {
            name,
            nodes: Vec::new(),
        }
    }

    pub fn add(&mut self, node: impl Into<NodeEnum>) {
        self.nodes.push(node.into());
    }
}

impl Node for Directory {
    fn name(&self) -> &str {
        &self.name
    }
}