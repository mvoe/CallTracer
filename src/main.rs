mod banner;
mod config;
mod format;
mod leak;
mod reverse;
mod social;
mod voip;
mod utils;

use clap::{Arg, Command};

fn main() {
    banner::print_banner();

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
        .get_matches();

    // Retrieve the phone number from the arguments
    let number = matches.get_one::<String>("number").unwrap();

    // Check if the format flag is set
    if matches.get_flag("format") {
        format::check_number_format(number);
    }
}
