#![recursion_limit = "1024"]

#[macro_use]
extern crate log;

extern crate failure;

#[macro_use]
extern crate lazy_static;

extern crate clap;
extern crate env_logger;
extern crate futures;
extern crate reqwest;
extern crate serde_yaml;
extern crate systemd;
extern crate tokio;
extern crate tokio_ping;
extern crate trust_dns_resolver;

mod args;
mod check;
mod error;
mod logger;
mod lookup;
mod watch;

use args::{get_cli_args, NetmonSubcommand};

use error::Result;

fn main() {
    if let Err(ref e) = run() {
        error!("\x1B[1;31mError: {}\x1B[0m", e);

        //        for inner in e.iter().skip(1) {
        //            error!("  caused by: {}", inner);
        //        }

        ::std::process::exit(1);
    }
}

fn run() -> Result<()> {
    logger::init_logger();

    let args = get_cli_args();

    match args.subcommand {
        NetmonSubcommand::Watch => watch::watch(&args),
        NetmonSubcommand::Check => check::check(&args),
    }
}
