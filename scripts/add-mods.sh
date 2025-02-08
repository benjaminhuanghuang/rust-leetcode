#!/bin/bash
# 
# Append module declarations to mod.rs in the specified directory
#

# Directory containing the Rust files (e.g., src/utils)
TARGET_DIR="$1"

# Path to the mod.rs file (will create this file if it doesn't exist)
MOD_RS="$TARGET_DIR/mod.rs"

# Create or overwrite the mod.rs file
echo "// Automatically generated mod.rs" > "$MOD_RS"

# Loop through all .rs files in the target directory (excluding mod.rs)
for file in "$TARGET_DIR"/*.rs; do
  # Get the filename without path and extension
  filename=$(basename -- "$file")
  modulename="${filename%.*}"
  
  # Skip mod.rs itself
  if [[ "$modulename" != "mod" ]]; then
    echo "pub mod $modulename;" >> "$MOD_RS"
  fi
done

echo "mod.rs updated with the module declarations."
