use phonenumber::{parse, Mode};
use colored::*;

pub fn check_number_format(number: &str) {
    match phonenumber::parse(None, number) {
        Ok(phone) => {
            println!("{} {}", "[+] Valid number:".green(), phone.format().mode(phonenumber::Mode::International));
        },
        Err(e) => {
            println!("{} {}", "[-] Invalid number:".red(), e);
        }
    }
}