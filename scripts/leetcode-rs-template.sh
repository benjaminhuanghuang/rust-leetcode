# Create a file named leetcode
#!/bin/bash

# Prompt the user for the file name
read -p "Enter the problem title: " problem_title

# Prompt the user for the file name
read -p "Enter the category: " problem_category

# Extract the problem number
number=$(echo "$problem_title" | cut -d '.' -f 1)
padded_number=$(printf "%04d" $number)

# Remove the problem number from the input string
title=$(echo "$problem_title" | cut -d '.' -f 2-)

# Convert the title to lowercase and replace spaces and special characters with hyphens
converted_title=$(echo "$title" | tr '[:upper:]' '[:lower:]' | sed 's/[^a-z0-9]/-/g' | sed 's/-\+/-/g' | sed 's/^-//;s/-$//')

folder_name="src/${problem_category}"
file_path="src/${problem_category}/${converted_title}_${padded_number}.rs"

# Check if the file already exists
if [ -e "$file_path" ]; then
  echo "File $file_path already exists."
  exit 1
fi

if [ ! -d "$folder_name" ]; then
  mkdir -p "$folder_name"
fi

# Create the file
touch "$file_path"

echo "/*" >> "$file_path"
echo "$problem_title" >> "$file_path"
echo "\n" >> "$file_path"
echo "*/" >> "$file_path"
# Confirm that the file was created
# if [ -e "$filename" ]; then
#   echo "File $filename created successfully."
# else
#   echo "Failed to create file $filename."
#   exit 1
# fi
