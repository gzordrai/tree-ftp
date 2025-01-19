use serde::{Serialize, Serializer};
use serde::ser::SerializeMap;

use super::node::{Node, NodeEnum};

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

    pub fn to_string(&self, indent: &str) -> String {
        let mut result: String = format!("{}{}\n", indent, self.name);
        let new_indent: String = format!("{}{}", indent, "    ");

        for node in &self.nodes {
            result.push_str(&node.to_string(&new_indent));
        }

        result
    }
}

impl Node for Directory {
    fn name(&self) -> &str {
        &self.name
    }
}

impl Serialize for Directory {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map: <S as Serializer>::SerializeMap = serializer.serialize_map(Some(self.nodes.len()))?;

        for node in &self.nodes {
            match node {
                NodeEnum::Directory(dir) => {
                    map.serialize_entry(&dir.name, dir)?;
                }
                NodeEnum::File(file) => {
                    map.serialize_entry(&file.name, file)?;
                }
            }
        }

        map.end()
    }
}