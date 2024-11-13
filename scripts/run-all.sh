#!/bin/bash

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

echo "=== Running Cargo Format ==="
"$SCRIPT_DIR/run-fmt.sh"

if [ $? -eq 0 ]; then
    echo -e "\n=== Running Cargo Clippy ==="
    "$SCRIPT_DIR/run-clippy.sh"

    if [ $? -eq 0 ]; then
        echo -e "\n=== Running Cargo Tests ==="
        "$SCRIPT_DIR/run-test.sh"
    else
        echo -e "\n❌ Clippy failed. Skipping tests."
        exit 1
    fi
else 
    echo -e "\n❌ Format check failed. Skipping clippy and tests."
    exit 1
fi