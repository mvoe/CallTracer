use clap::{Arg, Command};

/// Structure holding the parsed CLI arguments.
pub struct CliArgs {
    pub number: String,
    pub format: bool,
    pub lookup: bool,
    pub googledorking: bool,
    pub duckduckgo: bool,
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
            Arg::new("googledorking")
                .short('g')
                .long("googledorking")
                .help("Perform lookup with Google dorking")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("duckduckgo")
                .short('k')
                .long("duckduckgo")
                .help("Perform lookup with DuckDuckGo")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let number = matches.get_one::<String>("number").unwrap().to_string();
    let format = matches.get_flag("format");
    let lookup = matches.get_flag("lookup");
    let googledorking = matches.get_flag("googledorking");
    let duckduckgo = matches.get_flag("duckduckgo");

    CliArgs {
        number,
        format,
        lookup,
        googledorking,
        duckduckgo,
    }
}
