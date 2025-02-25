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
- **🔍 Web Search:** Perform a search across multiple search engines for the phone number.

More features will follow as development continues.

## Installation 🛠️

1. **Clone the repository:**
   ```bash
   git clone https://github.com/your-username/CallTracer.git
   ```
2. **Navigate to the project directory:**
   ```bash
   cd CallTracer
   ```
3. **Navigate to the Bash script directory:**
   ```bash
   cd src/bahs_scripts
   ```
4. **Make the Bash script executable:**
   ```bash
   chmod +x search.sh
   ```
5. **Return to the main directory and run the project:**
   ```bash
   cd ..
   cargo run -- --number "+4915123456789"
   ```

## API Key Setup 🔑

To use the country and provider lookup feature, you need a Neutrino API key. Update the `perform_lookup` function in the code:

```rust
let user = "YOUR_USER_ID";
let api_key = "YOUR_API_KEY";
```

Replace `YOUR_USER_ID` and `YOUR_API_KEY` with valid credentials.

## Usage ⚙️

### Format Check

Validates the structure of a phone number.

```bash
cargo run -- --number "+4915123456789" --format
```

### Country & Provider Info (API-Based)

Retrieves country and provider details using an API.

```bash
cargo run -- --number "+4915123456789" --lookup
```

### Web Search

Performs a search for the phone number across multiple search engines.

```bash
./search.sh "+4915123456789"
```

Supported search engines:

- ✅ **Bing**
- ✅ **DuckDuckGo**


## Disclaimer ⚠️

**Note:** This tool is still under development. Additional features will be implemented over time. Contributions, feedback, and suggestions are welcome!

