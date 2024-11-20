#!/bin/bash

# Count the number of *.ts files under current directory recursively with prefix "leetcode_"
count=$(find . -type f -name "leetcode_*.ts" ! -name "*.spec.ts" | wc -l)

# Output the result
echo "Solved $count leetcode problems."