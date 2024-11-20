#!/bin/bash

# Loop through all files matching the pattern *_????_template.rs
for file in _????_*.rs; do
    echo "$file"
  # Extract the number and template name
  number=$(echo "$file" | sed -E 's/^_([0-9]{4})_.*/\1/')
  name=$(echo "$file" | sed -E 's/^_[0-9]{4}_(.*).rs/\1/')

  # Remove underscores and reformat the filename
  new_name="${name}_${number}.rs"  # Combine as "template_0000.rs"

  # Rename the file
  mv "$file" "$new_name"

  # Print a success message
  echo "Renamed $file -> $new_name"
done