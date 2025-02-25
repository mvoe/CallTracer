use std::error::Error;
use colored::Colorize;
use scraper::{Html, Selector};
use urlencoding::encode;

/// Performs a DuckDuckGo search using the static HTML interface and counts the results.
pub async fn duckduckgo_search(query: &str) -> Result<(), Box<dyn Error>> {
    let encoded_query = encode(query);
    let url = format!("https://html.duckduckgo.com/html?q={}", encoded_query);

    let response = reqwest::get(&url).await?.text().await?;
    let document = Html::parse_document(&response);
    let selector = Selector::parse("a.result__a").unwrap();

    println!("{} {}", "DuckDuckGo search results for query:".purple(), query);
    println!("{} {}", "[*] Fetching URL:".cyan(), url);

    let count = document.select(&selector).count();
    println!("{} {}", "[*] Number of results found:".green(), count);

    Ok(())
}