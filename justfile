default: check

bench:
    cargo bench --manifest-path num-format-benches/Cargo.toml

check:
    cargo clippy --all-features --all-targets --tests --workspace

clippy: check

fmt:
    cargo fmt --all

locale:
    cargo run --manifest-path num-format-dev/Cargo.toml
    cargo fmt --all

readme:
    #!/usr/bin/env bash
    set -euo pipefail
    (
        cd num-format
        cargo readme > README.md
    )
    cp num-format/README.md ./README.md
    (
        cd num-format-windows
        cargo readme > README.md
    )

test:
    cargo fmt --all --check
    cargo clippy --all-features --all-targets --tests --workspace -- -D warnings
    cargo hack --package num-format --feature-powerset test
    cargo hack --package num-format-benches --feature-powerset test
    cargo hack --package num-format-dev --feature-powerset test
    cargo hack --package num-format-windows --feature-powerset test
