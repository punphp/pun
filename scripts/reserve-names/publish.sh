#!/usr/bin/env bash
# Publishes 0.0.0 placeholders to reserve Pun's crate names on crates.io.
# Needs a crates.io token: run `cargo login` first. Publishing is permanent.
set -uo pipefail
names=(punphp punphp-tokens punphp-fixers punphp-format punphp-config punphp-workspace punphp-harness punphp-php)
work=$(mktemp -d)
for n in "${names[@]}"; do
  if [ "$(curl -s -o /dev/null -w '%{http_code}' -A 'punphp-reserve' "https://crates.io/api/v1/crates/$n")" = 200 ]; then
    echo "$n: already published"; continue
  fi
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
  # crates.io rate-limits new crate names; wait and retry on 429
  until out=$(cd "$work/$n" && cargo publish --allow-dirty 2>&1); do
    echo "$out" | grep -q '429\|Too Many' || { echo "$n: FAILED"; echo "$out" | tail -3; break; }
    echo "$n: rate limited, retrying in 10 min"; sleep 600
  done && echo "$n: published"
done
