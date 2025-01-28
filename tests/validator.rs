use tree_ftp::utils::validator::DomainAllowPort;
use validators::traits::ValidateString;

#[test]
fn test_validate_address() {
    let address: &str = "127.0.0.1:21";

    assert!(DomainAllowPort::parse_str(address).is_ok());
}