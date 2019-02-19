#!/usr/bin/env bash

set -e

./scripts/check.sh

# no_std
cargo test --manifest-path num-format/Cargo.toml --no-default-features
cargo test --manifest-path num-format/Cargo.toml --no-default-features --features "with-serde"

# std
cargo test --manifest-path num-format/Cargo.toml
cargo test --manifest-path num-format/Cargo.toml --features "with-num-bigint"
cargo test --manifest-path num-format/Cargo.toml --features "with-system-locale"
cargo test --manifest-path num-format/Cargo.toml --features "with-serde with-system-locale"
cargo test --manifest-path num-format/Cargo.toml --no-default-features --all-features
