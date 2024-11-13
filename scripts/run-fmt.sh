#!/bin/bash

function run_fmt() {
    echo "Running checking cargo fmt..."
    cargo fmt -- --check
}

run_fmt