use validators::prelude::*;

#[derive(Validator)]
#[validator(domain(ipv4(Allow), local(Allow), port(Allow), at_least_two_labels(Allow)))]
pub struct DomainAllowPort {
    pub domain: String,
    pub port: Option<u16>,
}