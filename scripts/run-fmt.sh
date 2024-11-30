#!/bin/bash

function run_fmt() {
    echo "Running checking cargo fmt..."
    cd ../
    cargo fmt -- --check
}

run_fmt