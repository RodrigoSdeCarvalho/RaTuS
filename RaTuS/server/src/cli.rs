use clap::{App, Arg, ArgMatches};

pub const CONFIG_FILE: &str = "config-file";

pub(crate) fn parse_args<'a>() -> ArgMatches<'a> {
    App::new("tuple-space-server")
        .arg(
            Arg::with_name(CONFIG_FILE)
                .long("config-file")
                .short("c")
                .takes_value(true)
                .required(true)
                .help("Config file to use"),
        )
        .get_matches()
}
