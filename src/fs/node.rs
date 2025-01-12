use super::directory::Directory;
use super::file::File;

pub trait Node {
    fn name(&self) -> &str;
}

#[derive(Debug)]
pub enum NodeEnum {
    Directory(Directory),
    File(File),
}

impl Node for NodeEnum {
    fn name(&self) -> &str {
        match self {
            NodeEnum::Directory(dir) => dir.name(),
            NodeEnum::File(file) => file.name(),
        }
    }
}

// Implementing the From trait for File to allow easy conversion from File to NodeEnum
// Before: directory.add(NodeEnum::File(File::new(String::from("name"))))
// After: directory.add(File::new(String::from("name")))
impl From<File> for NodeEnum {
    fn from(file: File) -> Self {
        NodeEnum::File(file)
    }
}

// Implementing the From trait for Directory to allow easy conversion from Directory to NodeEnum
// Before: directory.add(NodeEnum::Directory(Directory::new(String::from("subdir"))))
// After: directory.add(Directory::new(String::from("subdir")))
impl From<Directory> for NodeEnum {
    fn from(directory: Directory) -> Self {
        NodeEnum::Directory(directory)
    }
}