#!/bin/bash

# Loop through all files matching the pattern *_????_template.rs
for file in _????_*.rs; do
  echo "$file"
  # Extract the number part (e.g., 0003)
  number=$(echo "$file" | sed -E 's/^_([0-9]{4})_.*/\1/')
   # Extract the name part (e.g., longest_substring_without_repeating_characters)
  name=$(echo "$file" | sed -E 's/^_[0-9]{4}_(.*).rs/\1/')

  # Remove underscores and reformat the filename
  new_name="${name}_${number}.rs"  # Combine as "template_0000.rs"

  # Rename the file
  mv "$file" "$new_name"

  # Print a success message
  echo "Renamed $file -> $new_name"
done