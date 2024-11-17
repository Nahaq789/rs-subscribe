#!/bin/bash

function run_clippy() {
    echo "Running checking cargo clippy..."
    cargo clippy -- -A dead_code -A clippy::upper_case_acronyms -A clippy::enum_variant_names -A clippy::too_many_arguments -A clippy::new_without_default
}

run_clippy