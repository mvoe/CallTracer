use std::process::Command;
use std::error::Error;

/// Runs the external search script for the specified query.
/// The script itself handles all search engines.
/// Runs the external search script for the specified query.
pub fn run_search(_engine: &str, query: &str) -> Result<(), Box<dyn Error>> {
    let output = Command::new("bash")
        .arg("./search.sh")
        .arg(format!("{}", query)) // Query übergeben
        .output()?;

    println!("{}", String::from_utf8_lossy(&output.stdout));
    Ok(())
}
