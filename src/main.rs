mod banner;
mod config;
mod format;
mod reverse;
mod social;
mod voip;
mod utils;
mod lookup;

use clap::{Arg, Command};
use colored::*;

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
        .arg(
            Arg::new("lookup")
                .short('l')
                .long("lookup")
                .help("Perform lookup using Neutrino API")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    // Retrieve the phone number from the arguments.
    let number = matches.get_one::<String>("number").unwrap();

    let lookup_flag = matches.get_flag("lookup");
    let format_flag = matches.get_flag("format");

    if !lookup_flag && !format_flag {
        println!(
            "{} {}",
            "[*] Running check for".purple(),
            number.yellow().bold()
        );
        format::check_number_format(number);
        lookup::perform_lookup(number);
    } else {
        if format_flag {
            format::check_number_format(number);
        }
        if lookup_flag {
            lookup::perform_lookup(number);
        }
    }
}