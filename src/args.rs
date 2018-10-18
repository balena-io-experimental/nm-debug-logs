use clap::{App, AppSettings, SubCommand};

pub enum NetmonSubcommand {
    Watch,
    Check,
}

pub struct Args {
    pub subcommand: NetmonSubcommand,
}

pub fn get_cli_args() -> Args {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(SubCommand::with_name("watch").about("Watch about help string"))
        .subcommand(SubCommand::with_name("check").about("Check about help string"))
        .get_matches();

    let subcommand = match matches.subcommand() {
        ("watch", _) => NetmonSubcommand::Watch,
        ("check", _) => NetmonSubcommand::Check,
        _ => unreachable!(),
    };

    Args { subcommand }
}
