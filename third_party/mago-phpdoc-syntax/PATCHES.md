# Mago PHPDoc syntax 1.57.0: maintained patch

## Source

The library source and normalized manifest come from the published `mago-phpdoc-syntax` 1.57.0
package (checksum `0eeb52927d77253f3467d696bddf9dce88349b3ed4917baf7408d46ba6733437`). Its
`.cargo_vcs_info.json` identifies upstream revision
[`9656cd55`](https://github.com/carthage-software/mago/tree/9656cd55d51d96bd6dc57f1f1e73c15f22cb7cc3/crates/phpdoc-syntax).
Both license files come from that revision. Registry bookkeeping, the package lockfile, and
external tests are omitted, along with their manifest targets. The `indoc` development dependency
remains because the upstream lexer unit tests use it. Sibling Mago dependencies stay pinned to
`=1.57.0`. The root manifest patches this crate and excludes it from Pun's workspace members.

## Quoted shape-key source tokens

`ShapeKey::String::raw` retains the complete quoted token, including its delimiters and escapes.
The quoted constructor stores `Some(token.value)`. The three bare constructors (float, signed
float, and identifier) store `None`. The existing `value` slice, spans, display, and parsing rules
are unchanged. This follows the existing `LiteralStringType` raw-token representation.

Codex's array-key builder uses this metadata to decode quoted keys with PHPStan's PHPDoc escape
grammar before interning their semantic bytes and applying its existing numeric-key conversion.
Object-shape properties still use the existing `value` field. The parser does not decode strings
or depend on Pun's semantic decoder.

Public scanner, inline-annotation, and duplicate-key controls live in
`crates/pun-analyse/tests/duplicate_array_keys.rs`. The fixtures and reproducible PHPStan 2.3.1
capture under `tests/fixtures/duplicate-array-keys/` retain hexadecimal key and value bytes separately
from UTF-8 diagnostic presentation. Bare float spellings are parser controls, not a claim that
PHPStan accepts float keys in PHPDoc shapes.
