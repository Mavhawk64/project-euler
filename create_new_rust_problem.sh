#!/bin/bash

# Check if both arguments are provided
if [ "$#" -ne 2 ]; then
    echo "Usage: $0 <problem_number> <project_name>"
    exit 1
fi

# Assign arguments to variables
PROBLEM_NUM=$1
PROJECT_NAME=$2

# Define the base path
BASE_DIR=~/repos/project-euler
TARGET_DIR="problem-$PROBLEM_NUM"

# Execute the workflow
mkdir -p "$BASE_DIR/$TARGET_DIR" && \
cd "$BASE_DIR/$TARGET_DIR" && \
cargo new "$PROJECT_NAME" && \
cd "$PROJECT_NAME" && \
code ./src/main.rs