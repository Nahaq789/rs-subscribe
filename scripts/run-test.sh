#!/bin/bash

function run_test() {
    echo "Running cargo test..."
    cargo test --color=always --lib
}

run_test
