use serde::Serialize;

use super::directory::Directory;
use super::file::File;

/// A trait representing a node in the filesystem.
///
/// A node can be either a file or a directory.
pub trait Node {
    /// Returns the name of the node.
    ///
    /// # Returns
    ///
    /// A string slice that holds the name of the node.
    fn name(&self) -> &str;
}

/// An enum representing either a file or a directory in the filesystem.
#[derive(Debug, Serialize)]
pub enum NodeEnum {
    /// A directory node.
    Directory(Directory),

    /// A file node.
    File(File),
}

impl NodeEnum {
    /// Converts the node to a string with the given indentation.
    ///
    /// # Arguments
    ///
    /// * `indent` - A string slice that holds the current level of indentation.
    ///
    /// # Returns
    ///
    /// A `String` representation of the node.
    pub fn to_string(&self, indent: &str) -> String {
        match self {
            NodeEnum::Directory(dir) => format!(".\n{}", dir.to_string(indent)),
            NodeEnum::File(file) => format!(".\n└── {}", file.name),
        }
    }
}

impl Node for NodeEnum {
    /// Returns the name of the node.
    ///
    /// # Returns
    ///
    /// A string slice that holds the name of the node.
    fn name(&self) -> &str {
        match self {
            NodeEnum::Directory(dir) => dir.name(),
            NodeEnum::File(file) => file.name(),
        }
    }
}

/// Implementing the `From` trait for `File` to allow easy conversion from `File` to `NodeEnum`.
///
/// This allows adding a `File` to a directory without explicitly converting it to `NodeEnum`.
impl From<File> for NodeEnum {
    fn from(file: File) -> Self {
        NodeEnum::File(file)
    }
}

/// Implementing the `From` trait for `Directory` to allow easy conversion from `Directory` to `NodeEnum`.
///
/// This allows adding a `Directory` to another directory without explicitly converting it to `NodeEnum`.
impl From<Directory> for NodeEnum {
    fn from(directory: Directory) -> Self {
        NodeEnum::Directory(directory)
    }
}