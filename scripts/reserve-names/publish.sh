#!/usr/bin/env bash
# Publishes 0.0.0 placeholders to reserve Pun's crate names on crates.io.
# Needs a crates.io token: run `cargo login` first. Publishing is permanent.
set -euo pipefail
names=(punphp pun-tokens pun-fixers pun-format pun-config pun-workspace pun-harness pun-php)
work=$(mktemp -d)
for n in "${names[@]}"; do
  mkdir -p "$work/$n/src"
  cat > "$work/$n/Cargo.toml" <<TOML
[package]
name = "$n"
version = "0.0.0"
edition = "2024"
license = "MIT"
description = "Reserved for Pun, a fast all-in-one PHP toolchain. https://github.com/punphp/pun"
repository = "https://github.com/punphp/pun"
TOML
  echo "//! Reserved for Pun: https://github.com/punphp/pun" > "$work/$n/src/lib.rs"
  (cd "$work/$n" && cargo publish --allow-dirty)
  sleep 15 # crates.io rate-limits new crate names
done
