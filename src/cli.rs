extern crate clap;
use clap::{App, Arg};

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum CliOptions {
    Table,
    Run,
    Query,
    Exit
}


pub fn cli() -> CliOptions {

    
    let mut app: App = App::new("AMD Ryzen Mobile Monitor")
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
                .help("Print only the table version and exit.")
                .conflicts_with_all(&[
                    "run",
                    "query"
                ])
        ).arg(
            Arg::with_name("query")
                .short("q")
                .long("query")
                .value_name("boolen")
                .takes_value(false)
                .help("Print current processor info and exit.")
                .conflicts_with_all(&[
                    "table_version",
                    "run"
                ])
        ).arg(
            Arg::with_name("run")
                .short("r")
                .long("run")
                .value_name("boolean")
                .takes_value(false)
                .help("Run and print processor info continuously.")
                .conflicts_with_all(&[
                    "table_version",
                    "query"
                ])
        );
    
    let matches = app.clone().get_matches();
    
    if matches.is_present("table_version") {
        CliOptions::Table
    } else if matches.is_present("query"){
        CliOptions::Query
    } else if matches.is_present("run"){
        CliOptions::Run
    } else {
        match app.print_help() {
            Result::Ok (_ok) => {},
            Result::Err (err) => {
                panic!("{:?}",err);
            }
        };
        CliOptions::Exit
    }
}
