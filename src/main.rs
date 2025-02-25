mod banner;
mod format;
mod voip;
mod lookup;
mod googledorking;
mod duckduckgo;

use clap::{Arg, Command};
use colored::*;
use tokio::task;

#[tokio::main]
async fn main() {
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

    let lookup_flag = matches.get_flag("lookup");
    let format_flag = matches.get_flag("format");
    let googledorking_flag = matches.get_flag("googledorking");
    let duckduckgo_flag = matches.get_flag("duckduckgo");

    if !lookup_flag && !format_flag && !googledorking_flag && !duckduckgo_flag {
        println!(
            "{} {}",
            "Running check for".purple(),
            number.yellow().bold()
        );
        task::spawn_blocking({
            let number = number.clone();
            move || format::check_number_format(&number)
        })
            .await
            .unwrap();

        task::spawn_blocking({
            let number = number.clone();
            move || lookup::perform_lookup(&number)
        })
            .await
            .unwrap();

        if let Err(e) = googledorking::google_dork_search(&number).await {
            eprintln!("{}", format!("[!] Google dorking failed: {}", e).red());
        }

        if let Err(e) = duckduckgo::duckduckgo_search(&number).await {
            eprintln!("{}", format!("[!] DuckDuckGo search failed: {}", e).red());
        }
    } else {
        if format_flag {
            task::spawn_blocking({
                let number = number.clone();
                move || format::check_number_format(&number)
            })
                .await
                .unwrap();
        }
        if lookup_flag {
            task::spawn_blocking({
                let number = number.clone();
                move || lookup::perform_lookup(&number)
            })
                .await
                .unwrap();
        }
        if googledorking_flag {
            if let Err(e) = googledorking::google_dork_search(&number).await {
                eprintln!("{}", format!("[!] Google dorking failed: {}", e).red());
            }
        }
        if duckduckgo_flag {
            if let Err(e) = duckduckgo::duckduckgo_search(&number).await {
                eprintln!("{}", format!("[!] DuckDuckGo search failed: {}", e).red());
            }
        }
    }
}
