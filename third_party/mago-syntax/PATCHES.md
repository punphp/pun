# Mago syntax 1.57.0: maintained patch

## Source

The library source and normalized manifest come from the published `mago-syntax` 1.57.0 package
(checksum `c9f8e390954afaad9f6d5138b9f67449de8c99e125bf046fafcdda4a1abd8d0a`). Its
`.cargo_vcs_info.json` identifies upstream revision
[`9656cd55`](https://github.com/carthage-software/mago/tree/9656cd55d51d96bd6dc57f1f1e73c15f22cb7cc3/crates/syntax).
Both license files come from that revision. Registry bookkeeping, the package lockfile, external
tests, and benchmarks are omitted, along with their manifest targets and development dependencies.
Sibling Mago dependencies remain pinned to `=1.57.0`. The root manifest patches this crate and
excludes its directory from Pun's workspace members.

## Opening tags within leading shebang text

`lexer::Lexer::advance()` searches the first shebang line for enabled opening tags before consuming
that line as inline text. The existing inline scanner owns tag recognition, including
`LexerSettings::enable_short_tags`. A same-line opening tag or an opening tag after a bare CR
therefore starts PHP code instead of disappearing into an `InlineShebang` token.
Ordinary LF and CRLF shebangs retain their token and CST classification. Source bytes and spans
are unchanged. This follows PHP's `token_get_all` INITIAL mode, which PHPParser uses, rather than
the separate executable-file SHEBANG mode.

Public PHPStan 2.3.1 captures are in `pun-analyse/tests/fixtures/parser/shebang-expected.json`.
They cover namespace placement, parameter validation, executable diagnostics, and valid and invalid
scan-only declarations. `pun-lint/tests/shebang.rs` checks lexer settings, token bytes and spans,
owned and borrowed CSTs, and source-preserving lint edits. General opening-tag grammar differences
and recovered-grammar translation remain outside this patch.
