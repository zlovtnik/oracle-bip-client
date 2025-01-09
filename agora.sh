#!/bin/bash

# A script to add and commit files one by one

# Check for untracked or modified files
files=$(git status --porcelain | awk '{print $2}')

if [ -z "$files" ]; then
    echo "No files to add or commit."
    exit 0
fi

# Loop through each file
for file in $files; do
    echo "Processing file: $file"
    
    # Stage the file
    git add "$file"
    
    # Commit the file with a specific message
    git commit -m "Add/Update file: $file"
done

echo "All files have been committed individually!"