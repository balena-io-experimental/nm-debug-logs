use futures::{Future, Stream};
use reqwest;
use tokio;
use tokio_ping;

use lookup::lookup_ip;
use args::Args;

use error::Result;

pub fn check(_args: &Args) -> Result<()> {
    info!("Checking...");

    let mut ips = lookup_ip("www.google.com")?;
    let ip = ips.pop().unwrap();


    let pinger = tokio_ping::Pinger::new();
    let stream = pinger.and_then(move |pinger| Ok(pinger.chain(ip).stream()));
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
