#!/usr/bin/env bash

set -e

(
    cd num-format
    cargo readme > README.md
)
cp num-format/README.md ./README.md
