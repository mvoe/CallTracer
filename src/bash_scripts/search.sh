#!/bin/bash
# search.sh
# This script performs a search on multiple search engines and counts the number of results.

if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <query>"
    exit 1
fi

query="$1"

# Define color variables
PURPLE="\033[0;35m"
CYAN="\033[0;36m"
GREEN="\033[0;32m"
NC="\033[0m"  # No Color (reset)

# URL-encode the query using Python
encoded_query=$(python3 -c "import urllib.parse, sys; print(urllib.parse.quote(sys.argv[1]))" "$query")

# Function that performs the search for a given engine
perform_search() {
    local engine="$1"
    local url="$2"
    local marker="$3"

    response=$(curl -sL "$url" -A "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36" -b "CONSENT=YES+")
    count=$(echo "$response" | grep -F -o "$marker" | wc -l)

    echo -e "\n${PURPLE}Search results for ${engine}:${NC}"
    echo -e "${CYAN}[*] Fetching URL:${NC} $url"
    echo -e "${GREEN}[+] Number of results:${NC} $count"
}

# Define URLs and markers for each engine
duckduckgo_url="https://html.duckduckgo.com/html?q=${encoded_query}"
duckduckgo_marker="class=\"result__a\""

bing_url="https://www.bing.com/search?q=${encoded_query}"
bing_marker="<li class=\"b_algo\""

# Run searches for all engines
perform_search "DuckDuckGo" "$duckduckgo_url" "$duckduckgo_marker"
perform_search "Bing" "$bing_url" "$bing_marker"