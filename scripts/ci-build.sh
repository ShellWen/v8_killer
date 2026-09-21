#!/usr/bin/env bash
set -euo pipefail

target=$1
profiles=$2
for profile in $profiles; do
  options=(--locked --target "$target")
  if [[ $profile == release ]]; then options+=(--release); fi
  cargo build "${options[@]}"
  cargo test "${options[@]}"
  platform=linux
  if [[ $target == x86_64-pc-windows-gnu ]]; then platform=win; fi
  bash scripts/ci-node.sh "$platform" "$profile" "$target"
done
cargo clippy --locked --target "$target" --all-targets --all-features -- -D warnings
