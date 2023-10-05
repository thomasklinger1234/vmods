#!/bin/bash

set -eu

cargo_feature() {
    echo "checking $1 with features $2"
    cargo clippy --manifest-path=$1/Cargo.toml --all-targets --features "$2" --no-default-features -- \
    `# Exit with a nonzero code if there are clippy warnings` \
    -Dwarnings
}


if rustup component add clippy; then
  cargo clippy --all-targets --all-features -- \
    `# Exit with a nonzero code if there are clippy warnings` \
    -Dwarnings

  cargo_feature "." "default"
fi
