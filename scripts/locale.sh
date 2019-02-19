#!/usr/bin/env bash

cargo +nightly run --manifest-path num-format-dev/Cargo.toml --features "nightly" \
|| cargo run --manifest-path num-format-dev/Cargo.toml

cargo fmt
