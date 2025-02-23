# CallTracer
# CallTracer

```text
  ______        __      ___      ___  ___________  _______        __       ______    _______   _______   
 /" _  "\      /""\    |"  |    |"  |("     _   ")/"      \      /""\     /" _  "\  /"     "| /"      \  
(: ( \___)    /    \   ||  |    ||  | )__/  \\__/|:        |    /    \   (: ( \___)(: ______)|:        | 
 \/ \        /' /\  \  |:  |    |:  |    \\_ /   |_____/   )   /' /\  \   \/ \      \/    |  |_____/   ) 
 //  \ _    //  __'  \  \  |___  \  |___ |.  |    //      /   //  __'  \  //  \ _   // ___)_  //      /  
(:   _) \  /   /  \\  \( \_|:  \( \_|:  \\:  |   |:  __   \  /   /  \\  \(:   _) \ (:      "||:  __   \  
 \_______)(___/    \___)\_______)\_______)\__|   |__|  \___)(___/    \___)\_______) \_______)|__|  \___) 
```

## Overview 🚀

CallTracer is a CLI OSINT tool for phone number investigation, written in Rust. It supports the following features:

- **📏 Format Check:** Validate the structure of phone numbers.
- **🌍 Country & Provider Info:** Retrieve details about the phone number’s origin and carrier using an external API.

More features will follow as development continues.

## Installation 🛠️

1. **Clone the repository:**
   ```bash
   git clone https://github.com/your-username/CallTrace.git
   ```
2. Navigate to the project directory:
    ```bash 
    cd CallTracer
    ``` 
3. Initialize the Cargo project (if not already initialized):
    ```bash
   cargo init
    ```
4. Build and run the project:
    ```bash
   cargo run -- --number "+1234567890" --format
    ```

## API Key Setup 🔑
To use the country and provider lookup feature, you need an neutrino API key. Update the `perform_lookup` function in the code:

```rust
let user = "YOUR_USER_ID";
let api_key = "YOUR_API_KEY";
```
Replace `YOUR_USER_ID` and `YOUR_API_KEY` with valid credentials.

## Usage ⚙️

### Format Check
Validates the structure of a phone number.

```bash
cargo run -- --number "+1234567890" --format
```

### Country & Provider Info (API-Based)
Retrieves country and provider details using an API.

```bash
cargo run -- --number "+1234567890" --lookup
```

## Disclaimer ⚠️

**Note:** This tool is still under development. Additional features will be implemented over time. Contributions, feedback, and suggestions are welcome!
