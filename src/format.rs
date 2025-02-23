use colored::*;
use phonenumber::{parse, Mode};

/// Validates the phone number and prints formatted output.
/// If valid, it also attempts to retrieve the country information.
pub fn check_number_format(number: &str){
    match parse(None, number) {
        Ok(phone) => {
            // Print the phone number in international format
            println!("{} {}", "[+] Valid number:".green(),
                     phone
                         .format()
                         .mode(Mode::International));
        },
        Err(e) => {
            println!("{} {}", "[-] Invalid number:".red(), e);
        }
    }
}