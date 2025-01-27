use serde::ser::SerializeMap;
use serde::{Serialize, Serializer};

use super::node::{Node, NodeEnum};

/// Represents a directory in the filesystem.
///
/// A `Directory` contains a name and a list of nodes, which can be either
/// subdirectories or files.
#[derive(Clone, Debug, Default)]
pub struct Directory {
    /// The name of the directory.
    pub name: String,

    /// The nodes contained within the directory.
    ///
    /// This can include both subdirectories and files, represented by the `NodeEnum` enum.
    pub nodes: Vec<NodeEnum>,
}

impl Directory {
    /// Creates a new directory with the given name.
    ///
    /// # Arguments
    ///
    /// * `name` - A `String` that holds the name of the directory.
    ///
    /// # Returns
    ///
    /// A new `Directory` instance.
    pub fn new(name: String) -> Self {
        Directory {
            name,
            nodes: Vec::new(),
        }
    }

    /// Adds a node to the directory.
    ///
    /// # Arguments
    ///
    /// * `node` - An item that can be converted into a `NodeEnum`.
    pub fn add(&mut self, node: impl Into<NodeEnum>) {
        self.nodes.push(node.into());
    }

    /// Converts the directory and its contents to a string with the given indentation.
    ///
    /// # Arguments
    ///
    /// * `indent` - A string slice that holds the current level of indentation.
    /// * `prefix` - A string slice that holds the prefix to be used for each line.
    ///
    /// # Returns
    ///
    /// A `String` representation of the directory and its contents.
    pub fn to_string(&self, indent: &str) -> String {
        let mut ret: String = String::new();

        for (i, node) in self.nodes.iter().enumerate() {
            let is_last: bool = i == self.nodes.len() - 1;
            let prefix: &str = if is_last { "└── " } else { "├── " };

            match node {
                NodeEnum::Directory(directory) => {
                    ret.push_str(&format!("{}{}{}\n", indent, prefix, directory.name()));

                    if is_last {
                        ret.push_str(&directory.to_string(&format!("{}    ", indent)));
                    } else {
                        ret.push_str(&directory.to_string(&format!("{}│    ", indent)));
                    }
                }
                NodeEnum::File(file) => {
                    ret.push_str(&format!("{}{}{}\n", indent, prefix, file.name()))
                }
            }
        }

        ret
    }
}

impl Node for Directory {
    /// Returns the name of the directory.
    ///
    /// # Returns
    ///
    /// A string slice that holds the name of the directory.
    fn name(&self) -> &str {
        &self.name
    }
}

impl Serialize for Directory {
    /// Serializes the directory and its contents.
    ///
    /// # Arguments
    ///
    /// * `serializer` - The serializer to use for serialization.
    ///
    /// # Returns
    ///
    /// A `Result` containing the serialized output or an error.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map: <S as Serializer>::SerializeMap =
            serializer.serialize_map(Some(self.nodes.len()))?;

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
