use super::node::Node;

#[derive(Debug)]
pub struct File {
    pub name: String,
}

impl File {
    pub fn new(name: String) -> Self {
        File { name }
    }
}

impl Node for File {
    fn name(&self) -> &str {
        &self.name
    }
}