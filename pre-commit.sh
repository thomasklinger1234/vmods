#!/usr/bin/env bash

cargo update
cargo fmt --all

./scripts/lint.sh
./scripts/test.sh
