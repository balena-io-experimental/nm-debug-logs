use futures::{Future, Stream};
use reqwest;
use tokio;
use tokio_ping;
use trust_dns_resolver::system_conf::read_system_conf;
use trust_dns_resolver::Resolver;

use errors::*;

use args::Args;

pub fn check(_args: &Args) -> Result<()> {
    info!("Checking...");

    let (config, opts) = read_system_conf()?;

    let resolver = Resolver::new(config, opts).unwrap();

    let response = resolver.lookup_ip("www.google.com").unwrap();

    for addr in response.iter() {
        info!("Resolved: {:?}", addr);
    }

    let address = response.iter().next().expect("no addresses returned!");

    let pinger = tokio_ping::Pinger::new();
    let stream = pinger.and_then(move |pinger| Ok(pinger.chain(address).stream()));
    let future = stream.and_then(|stream| {
        stream.take(3).for_each(|mb_time| {
            match mb_time {
                Some(time) => info!("time={}", time),
                None => info!("timeout"),
            }
            Ok(())
        })
    });

    tokio::run(future.map_err(|err| error!("Error: {}", err)));

    let text = reqwest::get("https://api.resin.io/ping")?.text()?;

    info!("{}", text);

    Ok(())
}
