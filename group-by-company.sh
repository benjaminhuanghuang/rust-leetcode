#!/bin/bash

# Prompt the user for the company name
read -p "Enter the company name: " company_name

# Directory to start the search from (current directory)
start_dir="."

# File to store results
output_file="$company_name.txt"

# Clear the output file if it exists
> "$output_file"

# Search recursively for files containing the string, excluding a specific folder
find "$start_dir" -type d -name 'node_modules' -prune -o -type f -exec grep -l "$company_name" {} + | while IFS= read -r file; do
    # Print file name and its folder name into the output file
    echo "$(dirname "$file")/$(basename "$file")" >> "$output_file"
done

echo "Search results written to $output_file"