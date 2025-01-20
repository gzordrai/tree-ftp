// // use tree_ftp::ftp::client::FtpClient;
// // use tree_ftp::ftp::command_stream::MockCommandStream;
// // use tree_ftp::ftp::data_stream::MockFtpDataStream;
// // use tree_ftp::ftp::error::Result;

// #[test]
// fn test_new() {
//     let addr = "127.0.0.1:21";
//     let mut mock_stream = MockCommandStream::new();
//     mock_stream.expect_read_response().returning(|| Ok("220 Welcome".to_string()));

//     let client = FtpClient::new(addr);
//     assert!(client.is_ok());
// }

// #[test]
// fn test_authenticate() {
//     let addr = "127.0.0.1:21";
//     let mut mock_stream = MockCommandStream::new();
//     mock_stream.expect_send_command().returning(|_| Ok(()));
//     mock_stream.expect_read_response().returning(|| Ok("230 User logged in".to_string()));

//     let mut client = FtpClient::new(addr).unwrap();
//     let result = client.authenticate("user", "pass");
//     assert!(result.is_ok());
// }

// #[test]
// fn test_retrieve_server_info() {
//     let addr = "127.0.0.1:21";
//     let mut mock_stream = MockCommandStream::new();
//     mock_stream.expect_send_command().returning(|_| Ok(()));
//     mock_stream.expect_read_response().returning(|| Ok("215 UNIX Type: L8".to_string()));

//     let mut client = FtpClient::new(addr).unwrap();
//     let result = client.retrieve_server_info();
//     assert!(result.is_ok());
// }