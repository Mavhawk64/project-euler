#!/bin/bash
set -e

# Extract problem number from current directory
dir=$(basename "$PWD")
problem_num=${dir#problem-}

# Validate we got a number
if [[ ! "$problem_num" =~ ^[0-9]+$ ]]; then
    echo "Error: Not in a problem-* directory"
    exit 1
fi

echo "Committing Problem $problem_num..."
git add .
git commit -m "Problem $problem_num"
git push

echo "✓ Successfully pushed Problem $problem_num"