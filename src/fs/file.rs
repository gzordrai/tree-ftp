use serde::Serialize;

use super::node::Node;

#[derive(Debug, Serialize)]
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