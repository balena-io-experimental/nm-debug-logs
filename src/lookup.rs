use std::net::IpAddr;

use trust_dns_resolver::system_conf::read_system_conf;
use trust_dns_resolver::Resolver;

use error::Result;

pub fn lookup_ip(host: &str) -> Result<Vec<IpAddr>> {
    let (config, opts) = read_system_conf()?;

    let resolver = Resolver::new(config, opts)?;

    let response = resolver.lookup_ip(host)?;

    Ok(response.iter().collect())
}
