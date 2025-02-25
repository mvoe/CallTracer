use std::process::Command;
use std::error::Error;

pub fn google_dork_search(query: &str) -> Result<(), Box<dyn Error>> {
    let output = Command::new("src/scripts/google_search.sh")
        .arg(query)
        .output()?;

    println!("{}", String::from_utf8_lossy(&output.stdout));
    Ok(())
}