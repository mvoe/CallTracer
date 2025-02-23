extern crate neutrino_api_client_reqwest;

use neutrino_api_client_reqwest::NeutrinoAPIClient;
use colored::*; // For colored terminal output
use std::collections::HashMap;

/// Performs a phone lookup using the Neutrino API client.
/// This function constructs the API request parameters, sends the request,
/// and then prints out the returned data or any error messages.
pub fn perform_lookup(number: &str) {
    // Replace these with your actual credentials from Neutrino API.
    let user = "YOUR_USER_ID";
    let api_key = "YOUR_API_KEY";

    // Create a new Neutrino API client using your user ID and API key.
    let client = &mut NeutrinoAPIClient::new(user, api_key);

    let mut params = HashMap::with_capacity(3);

    // Insert the phone number parameter (expected in international format).
    params.insert("number", number);
    // For numbers in international format, the country-code and ip can be left empty.
    params.insert("country-code", "");
    params.insert("ip", "");

    let response = client.phone_validate(params);

    // Check if the API response contains data.
    if let Some(data) = response.data {
        println!("{}", "[+] API Response OK:".green());

        let country = data.get("country").and_then(|v| v.as_str()).unwrap_or("N/A");
        println!("{} {}", "[+] Country:".green(), country);

        let country_code3 = data.get("country-code3").and_then(|v| v.as_str()).unwrap_or("N/A");
        println!("{} {}", "[+] Country Code3:".green(), country_code3);

        let currency_code = data.get("currency-code").and_then(|v| v.as_str()).unwrap_or("N/A");
        println!("{} {}", "[+] Currency Code:".green(), currency_code);

        let international_calling_code = data.get("international-calling-code")
            .and_then(|v| v.as_str())
            .unwrap_or("N/A");
        println!("{} {}", "[+] International Calling Code:".green(), international_calling_code);

        let international_number = data.get("international-number")
            .and_then(|v| v.as_str())
            .unwrap_or("N/A");
        println!("{} {}", "[+] International Number:".green(), international_number);

        let is_mobile = data.get("is-mobile")
            .and_then(|v| v.as_bool())
            .map(|b| b.to_string())
            .unwrap_or("N/A".to_string());
        println!("{} {}", "[+] Is Mobile:".green(), is_mobile);

        let local_number = data.get("local-number")
            .and_then(|v| v.as_str())
            .unwrap_or("N/A");
        println!("{} {}", "[+] Local Number:".green(), local_number);

        let location = data.get("location")
            .and_then(|v| v.as_str())
            .unwrap_or("N/A");
        println!("{} {}", "[+] Location:".green(), location);

        let prefix_network = data.get("prefix-network")
            .and_then(|v| v.as_str())
            .unwrap_or("N/A");
        println!("{} {}", "[+] Prefix Network:".green(), prefix_network);

        let typ = data.get("type")
            .and_then(|v| v.as_str())
            .unwrap_or("N/A");
        println!("{} {}", "[+] Type:".green(), typ);

        let valid = data.get("valid")
            .and_then(|v| v.as_bool())
            .map(|b| b.to_string())
            .unwrap_or("N/A".to_string());
        println!("{} {}", "[+] Valid:".green(), valid);
    } else {
        // If the API did not return data, print an error message with details.
        eprintln!(
            "{} {}, Error Code: {}, HTTP Status Code: {}",
            "[!] API Error:".red(),
            response.error_message,
            response.error_code,
            response.status_code
        );
        if let Some(cause) = response.error_cause {
            eprintln!("Error Caused By: {:?}", cause);
        }
    }
}
