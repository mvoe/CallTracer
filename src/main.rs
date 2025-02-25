mod banner;
mod format;
mod voip;
mod lookup;
mod googledorking;
mod duckduckgo;
mod cli;  // new module

use colored::*;
use tokio::task;

#[tokio::main]
async fn main() {
    banner::print_banner();

    // Parse CLI arguments using our dedicated module
    let cli_args = cli::parse_args();
    let number = cli_args.number;

    // Depending on the flags, run the corresponding checks
    if !cli_args.format && !cli_args.lookup && !cli_args.googledorking && !cli_args.duckduckgo {
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

        if let Err(e) = googledorking::google_dork_search(&number) {
            eprintln!("{}", format!("[!] Google dorking failed: {}", e).red());
        }

        if let Err(e) = duckduckgo::duckduckgo_search(&number).await {
            eprintln!("{}", format!("[!] DuckDuckGo search failed: {}", e).red());
        }
    } else {
        if cli_args.format {
            task::spawn_blocking({
                let number = number.clone();
                move || format::check_number_format(&number)
            })
                .await
                .unwrap();
        }
        if cli_args.lookup {
            task::spawn_blocking({
                let number = number.clone();
                move || lookup::perform_lookup(&number)
            })
                .await
                .unwrap();
        }
        if cli_args.googledorking {
            if let Err(e) = googledorking::google_dork_search(&number) {
                eprintln!("{}", format!("[!] Google dorking failed: {}", e).red());
            }
        }
        if cli_args.duckduckgo {
            if let Err(e) = duckduckgo::duckduckgo_search(&number).await {
                eprintln!("{}", format!("[!] DuckDuckGo search failed: {}", e).red());
            }
        }
    }
}
