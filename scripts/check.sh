#!/usr/bin/env bash

set -e

# no_std
cargo check --manifest-path num-format/Cargo.toml --no-default-features
cargo check --manifest-path num-format/Cargo.toml --no-default-features --features "with-serde"

# std
cargo check --manifest-path num-format/Cargo.toml
cargo check --manifest-path num-format/Cargo.toml --features "with-num-bigint"
cargo check --manifest-path num-format/Cargo.toml --features "with-system-locale"
cargo check --manifest-path num-format/Cargo.toml --features "with-serde with-system-locale"
cargo check --manifest-path num-format/Cargo.toml --no-default-features --all-features
