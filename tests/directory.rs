use tree_ftp::fs::directory::Directory;
use tree_ftp::fs::file::File;
use tree_ftp::fs::node::Node;

#[test]
fn test_directory_new() {
    let dir: Directory = Directory::new("test_dir".to_string());

    assert_eq!(dir.name(), "test_dir");
}

#[test]
fn test_directory_add() {
    let mut dir: Directory = Directory::new("test_dir".to_string());
    let file: File = File::new("test_file".to_string());

    dir.add(file);

    assert_eq!(dir.nodes.len(), 1);
}

#[test]
fn test_directory_to_string() {
    let mut dir: Directory = Directory::new("test_dir".to_string());
    let file: File = File::new("test_file".to_string());

    dir.add(file);

    let dir_str: String = dir.to_string("");

    assert!(dir_str.contains("test_file"));
}
