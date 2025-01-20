use tree_ftp::fs::{file::File, node::Node};

#[test]
fn test_file_new() {
    let file: File = File::new("test_file".to_string());

    assert_eq!(file.name(), "test_file");
}
