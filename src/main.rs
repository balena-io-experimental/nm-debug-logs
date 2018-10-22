#![recursion_limit = "1024"]

#[macro_use]
extern crate log;

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate lazy_static;

extern crate clap;
extern crate env_logger;
extern crate serde_yaml;
extern crate systemd;

extern crate futures;
extern crate reqwest;
extern crate tokio;
extern crate tokio_ping;
extern crate trust_dns_resolver;

mod args;
mod check;
mod errors;
mod logger;
mod watch;

use args::{get_cli_args, NetmonSubcommand};
use errors::*;

fn main() {
    if let Err(ref e) = run() {
        error!("\x1B[1;31mError: {}\x1B[0m", e);

        for inner in e.iter().skip(1) {
            error!("  caused by: {}", inner);
        }

        ::std::process::exit(exit_code(e));
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
