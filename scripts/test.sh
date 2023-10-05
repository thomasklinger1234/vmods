#!/bin/bash

set -eu

cargo build --all # required for vtc tests
cargo test --all --all-features "$@" -- --test-threads=1
