use tree_ftp::fs::node::NodeEnum;
use tree_ftp::fs::directory::Directory;
use tree_ftp::fs::file::File;

#[test]
fn test_node_enum_directory() {
    let dir: Directory = Directory::new("test_dir".to_string());
    let node: NodeEnum = NodeEnum::Directory(dir);

    assert!(matches!(node, NodeEnum::Directory(_)));
}

#[test]
fn test_node_enum_file() {
    let file: File = File::new("test_file".to_string());
    let node: NodeEnum = NodeEnum::File(file);

    assert!(matches!(node, NodeEnum::File(_)));
}