#!/bin/bash

# Check if both arguments are provided
if [ "$#" -ne 2 ]; then
    echo "Usage: $0 <problem_number> <project_name>"
    exit 1
fi

PROBLEM_NUM=$1
PROJECT_NAME=$2

# 1. Calculate the Folder Range (1-50, 51-100, etc.)
# Logic: ((num-1)/50 * 50) + 1
START_RANGE=$(( ((PROBLEM_NUM - 1) / 50) * 50 + 1 ))
END_RANGE=$(( START_RANGE + 49 ))
RANGE_DIR="solutions_${START_RANGE}-${END_RANGE}"

BASE_DIR=~/repos/project-euler
WORKSPACE_TOML="$BASE_DIR/Cargo.toml"
TARGET_DIR="$RANGE_DIR/problem-$PROBLEM_NUM"
RELATIVE_PATH="$TARGET_DIR/$PROJECT_NAME"

# 2. Create the project structure
mkdir -p "$BASE_DIR/$TARGET_DIR"
cd "$BASE_DIR" || exit

# Create the new cargo project inside the range folder
# We run cargo new from the BASE_DIR to ensure it handles paths correctly
cargo new "$RELATIVE_PATH"

# 3. Add to Workspace Cargo.toml
if [ -f "$WORKSPACE_TOML" ]; then
    # Check if the path is already in the file
    if ! grep -q "$RELATIVE_PATH" "$WORKSPACE_TOML"; then
        # Use sed to insert the new path into the members array
        sed -i "/members = \[/a \    \"$RELATIVE_PATH\"," "$WORKSPACE_TOML"
        echo "Added $RELATIVE_PATH to workspace members."
    fi

    # 4. Link euler_utils
    # PROJECT_TOML="$BASE_DIR/$RELATIVE_PATH/Cargo.toml"
    # if ! grep -q "euler_utils" "$PROJECT_TOML"; then
        # Path is 3 levels deep from the project root to project-euler/
        # echo 'euler_utils = { path = "../../../euler_utils" }' >> "$PROJECT_TOML"
        # echo "Linked euler_utils dependency."
    # fi
else
    echo "Warning: Root Cargo.toml not found. Skipping workspace update."
fi

cd "$BASE_DIR/$RELATIVE_PATH"

# 5. Open in VS Code
code "$BASE_DIR/$RELATIVE_PATH/src/main.rs"