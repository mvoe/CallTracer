use reqwest;
use std::error::Error;
use colored::Colorize;
use urlencoding::encode;

pub async fn google_dork_search(query: &str) -> Result<(), Box<dyn Error>> {
    let encoded_query = encode(query);
    let url = format!("https://www.google.com/search?q={}", encoded_query);

    println!("{} {}", "Google search results:".purple(), query);

    println!("{} {}", "[*] Fetching URL:".cyan(), url);
    let response = reqwest::get(&url).await?.text().await?;

    let count = response.matches("/url?q=").count();
    println!("{} {}", "[*] Number of results found:".green(), count);

    Ok(())
}
