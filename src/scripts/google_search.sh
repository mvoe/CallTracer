#!/bin/bash
# google_search.sh
# This script performs a Google search for a given query and counts the number of results
# by counting occurrences of "/url?q=" in the HTML response.
# Usage: ./scripts/google_search.sh "<query>"

# Define color variables using ANSI escape codes
PURPLE="\033[0;35m"
CYAN="\033[0;36m"
GREEN="\033[0;32m"
NC="\033[0m"  # No Color (reset)

# Check if exactly one argument (the query) is provided
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <query>"
    exit 1
fi

# URL-encode the query using Python for proper handling of special characters
encoded_query=$(python3 -c "import urllib.parse, sys; print(urllib.parse.quote(sys.argv[1]))" "$1")
url="https://www.google.com/search?q=${encoded_query}"

# Fetch the URL using curl:
# -sL: silent mode and follow redirects
# -A: set a realistic User-Agent to mimic a browser
# -b: set a cookie to bypass Google's consent screen
response=$(curl -sL "$url" -A "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36" -b "CONSENT=YES+")

# Count the number of occurrences of the substring "/url?q=" in the response
count=$(echo "$response" | grep -o "/url?q=" | wc -l)

# Print the results with colored output
echo -e "\n${PURPLE}Google search results:${NC}"
echo -e "${CYAN}[*] Fetching URL:${NC} $url"
echo -e "${GREEN}[+] Number of results:${NC} $count"
