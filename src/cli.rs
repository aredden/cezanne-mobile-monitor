extern crate clap;
use clap::ArgMatches;
use clap::{App, Arg};

#[derive(PartialEq, Eq)]
pub enum CliOptions {
    Table,
    Run,
}

pub fn cli() -> CliOptions {
    let matches: ArgMatches = App::new("AMD Ryzen Mobile Monitor")
        .version("1.0")
        .about(
            "Call with -t or --table to see only table version, or none for constant monitoring.",
        )
        .arg(
            Arg::with_name("table_version")
                .short("t")
                .long("table")
                .value_name("boolean")
                .takes_value(false)
                .help("Print only the table version and exit."),
        )
        .get_matches();
    if matches.is_present("table_version") {
        CliOptions::Table
    } else {
        CliOptions::Run
    }
}
