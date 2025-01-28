use validators::prelude::*;

/// Represents a domain with an optional port.
///
/// This struct is validated to ensure the domain is valid, can be an IPv4 address,
/// can be a local address, and has at least two labels. The port is optional.
#[derive(Validator)]
#[validator(domain(ipv4(Allow), local(Allow), port(Allow), at_least_two_labels(Allow)))]
pub struct DomainAllowPort {
    pub domain: String,
    pub port: Option<u16>,
}