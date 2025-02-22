use colored::*;
use phonenumber::{parse, Mode, PhoneNumber};

/// Validates the phone number and prints formatted output.
/// If valid, it also attempts to retrieve the country information.
pub fn check_number_format(number: &str) -> bool {
    match parse(None, number) {
        Ok(phone) => {
            // Print the phone number in international format
            println!("{} {}", "[+] Valid number:".green(),
                     phone
                         .format()
                         .mode(Mode::International));

            // Try to get the country information using a separate function
            match get_country_info(&phone) {
                Ok(country_str) => println!("{} {}", "[+] Country:".green(), country_str),
                Err(err) => println!("{}", format!("[-] {}", err).red()),
            }
            true
        },
        Err(e) => {
            println!("{} {}", "[-] Invalid number:".red(), e);
            false
        }
    }
}

/// Attempts to extract the country information (e.g., ISO code) from the phone number.
/// Returns Ok(String) with the country code on success,
/// or an Err(String) with an error message if the information is not available.
fn get_country_info(phone: &PhoneNumber) -> Result<String, String> {
    // Retrieve the Country struct from the phone number
    let country = phone.country();

    // Attempt to extract the country ID (ISO code)
    match country.id() {
        Some(id) => Ok(format!("{:?}", id)), // Using debug formatting as a placeholder
        None => Err("Country ID not available".to_string()),
    }
}