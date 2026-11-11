#!/usr/bin/env bash
# Regenerate THIRD_PARTY_LICENSES.md, the license texts of every crate in the `pun` binary, with cargo-about
# (https://github.com/EmbarkStudios/cargo-about, 0.9.3: `cargo install cargo-about --locked`). Run it after a
# dependency changes; `--check` fails when the committed file is out of date.
set -euo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
out="$root/THIRD_PARTY_LICENSES.md"
tmp="$(mktemp)"
trap 'rm -f "$tmp"' EXIT
cargo about generate --locked -c "$root/scripts/about.toml" -m "$root/crates/pun-cli/Cargo.toml" "$root/scripts/about.hbs" -o "$tmp"
if [ "${1:-}" = "--check" ]; then
  diff -q "$tmp" "$out" > /dev/null || { echo "THIRD_PARTY_LICENSES.md is out of date: run scripts/gen-third-party-licenses.sh" >&2; exit 1; }
else
  cp "$tmp" "$out"
fi
