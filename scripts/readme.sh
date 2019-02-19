#!/usr/bin/env bash

set -e

(
    cd num-format
    cargo readme > README.md
)
cp num-format/README.md ./README.md

(
    cd num-format-windows
    cargo readme > README.md
)
