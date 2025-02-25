mod banner;
mod format;
mod voip;
mod lookup;
mod cli;
mod search;

use colored::*;
use tokio::task;

#[tokio::main]
async fn main() {
    banner::print_banner();

    let cli_args = cli::parse_args();
    let number = cli_args.number;

    // Here, if no specific internet search flag is provided, run all searches:
    if !cli_args.format && !cli_args.lookup && !cli_args.internetsearch {
        println!(
            "{} {}",
            "Running check for".purple(),
            number.yellow().bold()
        );
        // Run non-internet search checks.
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

        // Run all search engines using our unified Bash script.
        banner::print_divider();
        for engine in ["startpage", "duckduckgo", "bing" , "qwant"] {
            if let Err(e) = search::run_search(engine, &number) {
                eprintln!("{}", format!("[!] {} search failed: {}", engine, e).red());
            }
            break;
        }
        banner::print_divider();
    }
}