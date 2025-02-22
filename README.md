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

CallTrace is a CLI OSINT tool for phone number investigation. It supports the following features:

- **🌍 Format Check & Origin Analysis:** Validate phone numbers and determine their country and carrier.
- **🔓 Leak Database Check:** Verify if a phone number has appeared in any data breaches.
- **🔎 Reverse Lookup:** Attempt to retrieve associated information using reverse lookup.
- **📱 Social Media Check:** Determine if the phone number is linked to social media profiles.
- **☎️ VOIP Check:** Identify if the number is a VOIP number.


## Installation 🛠️

1. **Clone the repository:**
   ```bash
   git clone https://github.com/your-username/CallTrace.git
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
   cargo run -- --number "+1234567890" --all
    ```

## Usage ⚙️

Run the tool via Cargo with the following commands. Replace `"+1234567890"` with the phone number you want to investigate.

### Running All Checks
To perform all available checks (format, leak, reverse lookup, social media, and VOIP), run:

```bash
cargo run -- --number "+1234567890" --all
```
## Running Specific Checks

### Format Check & Origin Analysis
Validates the phone number and shows country/carrier details.

```bash
cargo run -- --number "+1234567890" --format
```

### Leak Database Check
Checks if the number has appeared in any known data breaches.

```bash
cargo run -- --number "+1234567890" --leak
```

### Reverse Lookup
Retrieves associated information using reverse lookup services.

```bash
cargo run -- --number "+1234567890" --reverse
```

### Social Media Check
Checks for social media profiles linked to the phone number.

```bash
cargo run -- --number "+1234567890" --social
```

### VOIP Check
Determines whether the number is a VOIP number.

```bash
cargo run -- --number "+1234567890" --voip
```

## Disclaimer ⚠️

**Note:** This project is still under active development. Some features may be incomplete or subject to change. Contributions, feedback, and suggestions are very welcome!



