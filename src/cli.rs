use clap::{Arg, Command};

/// Structure holding the parsed CLI arguments.
pub struct CliArgs {
    pub number: String,
    pub format: bool,
    pub lookup: bool,
    pub internetsearch: bool,
}

/// Parses the command line arguments and returns a `CliArgs` instance.
pub fn parse_args() -> CliArgs {
    let matches = Command::new("CallTracer")
        .version("0.1.0")
        .author("mël")
        .about("CLI OSINT tool for phone number investigation")
        .arg(
            Arg::new("number")
                .short('n')
                .long("number")
                .help("Phone number to analyze")
                .required(true)
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
            Arg::new("format")
                .short('f')
                .long("format")
                .help("Perform only format check")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("lookup")
                .short('l')
                .long("lookup")
                .help("Perform lookup using Neutrino API")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("internetsearch")
                .short('i')
                .long("internetsearch")
                .help("Search the internet")
                .action(clap::ArgAction::SetTrue),
        )

        .get_matches();

    let number = matches.get_one::<String>("number").unwrap().to_string();
    let format = matches.get_flag("format");
    let lookup = matches.get_flag("lookup");
    let internetsearch = matches.get_flag("internetsearch");

    CliArgs {
        number,
        format,
        lookup,
        internetsearch
    }
}
