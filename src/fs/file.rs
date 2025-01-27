use serde::Serialize;

use super::node::Node;

/// Represents a file in the filesystem.
///
/// A `File` contains a name.
#[derive(Clone, Debug, Serialize)]
pub struct File {
    /// The name of the file.
    pub name: String,
}

impl File {
    /// Creates a new file with the given name.
    ///
    /// # Arguments
    ///
    /// * `name` - A `String` that holds the name of the file.
    ///
    /// # Returns
    ///
    /// A new `File` instance.
    pub fn new(name: String) -> Self {
        File { name }
    }
}

impl Node for File {
    /// Returns the name of the file.
    ///
    /// # Returns
    ///
    /// A string slice that holds the name of the file.
    fn name(&self) -> &str {
        &self.name
    }
}