#!/bin/bash

# Check if both arguments are provided
if [ "$#" -ne 2 ]; then
    echo "Usage: $0 <problem_number> <project_name>"
    exit 1
fi

PROBLEM_NUM=$1
PROJECT_NAME=$2

BASE_DIR=~/repos/project-euler
WORKSPACE_TOML="$BASE_DIR/Cargo.toml"
TARGET_DIR="problem-$PROBLEM_NUM"
RELATIVE_PATH="$TARGET_DIR/$PROJECT_NAME"

# 1. Create the project
mkdir -p "$BASE_DIR/$TARGET_DIR" && \
cd "$BASE_DIR/$TARGET_DIR" && \
cargo new "$PROJECT_NAME"

# 2. Add to Workspace Cargo.toml if it's not already there
if [ -f "$WORKSPACE_TOML" ]; then
    # Check if the path is already in the file to avoid duplicates
    if ! grep -q "$RELATIVE_PATH" "$WORKSPACE_TOML"; then
        # Use sed to find the 'members = [' line and insert the new path after it
        # This adds the line and a comma for proper formatting
        sed -i "/members = \[/a \    \"$RELATIVE_PATH\"," "$WORKSPACE_TOML"
        echo "Added $RELATIVE_PATH to workspace members."
    fi

    # 3. Automatically link euler_utils to the new project's Cargo.toml
    PROJECT_TOML="$BASE_DIR/$RELATIVE_PATH/Cargo.toml"
    if ! grep -q "euler_utils" "$PROJECT_TOML"; then
        echo "euler_utils = { path = \"../../euler_utils\" }" >> "$PROJECT_TOML"
        echo "Linked euler_utils dependency."
    fi
else
    echo "Warning: Root Cargo.toml not found. Skipping workspace update."
fi

# 4. Open in VS Code
code "$BASE_DIR/$RELATIVE_PATH/src/main.rs"