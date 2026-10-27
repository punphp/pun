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

## Opt-in legacy-octal token retention

`LexerSettings::retain_invalid_legacy_octal` defaults to `false`. Pun's central analyser parser enables
it to retain complete malformed legacy-octal tokens for PHPParser's nonfatal `parseLNumber` validation.
Default lexing, lint and format still reject these literals. Other lexical failures and grammar recovery
are unchanged. Tokens and CSTs retain the original source bytes and full spans.

PHP's tokenizer selects `T_LNUMBER` or `T_DNUMBER` from the octal prefix before the first `8` or `9`,
after ignoring separators. An overflowing prefix produces a float even though native compilation
rejects the source. PHPParser accepts that float and converts only the prefix. `octal_prefix` follows
PHP's base-8 integer accumulation and floating-point widening; literal construction preserves that value
instead of treating the malformed spelling as decimal. `OffsetNumber`, explicit-octal boundaries,
ordinary floating-point literals and short-tag settings keep their existing behavior.

PHPStan 2.3.1 and PHPParser captures are in `pun-analyse/tests/fixtures/parser/numeric-admission-expected.json`.
The public parser tests cover reduction order, target versions, fatal escapes, delayed array checks,
name-resolution suppression and whole-file scan rejection. `pun-lint/tests/numeric_admission.rs` checks
native token classes, prefix values, source fidelity and strict consumers. General grammar recovery
remains outside this patch.

## Overflowing integer literal values

`parser::internal::literal` lowers valid integer tokens above signed `INT_MAX` to `Literal::Float`.
It reads the original magnitude instead of converting the clamped `parse_literal_integer()` result.
Tokens, raw bytes, and spans remain unchanged. The positive operand of `-9223372036854775808` remains
a float, as in PHPParser. The earlier malformed-legacy-octal path keeps its separate prefix conversion.

The reference is PHPParser 5.9.0's
[`Float_::parse()`](https://github.com/nikic/PHP-Parser/blob/9e33da9553fe7786f0962b35f4e4ecf01be89def/lib/PhpParser/Node/Scalar/Float_.php#L45-L72),
used by PHPStan 2.3.1 at `cb3b7e99489b2e3789ee2eec10501cb7a72580ea`. Decimal literals use
`parse_literal_float()`. Binary, octal, and hexadecimal literals follow PHP 8.5.12's
[`_php_math_basetozval()`](https://github.com/php/php-src/blob/136e049ac10b032c83018e65b63b6df858943750/ext/standard/math.c#L797-L878):
accumulate integers through `INT_MAX`, then continue in floating point. The pinned syntax-core
`parse_literal_integer_as_float()` is not equivalent: it rounds radix literals after `u128`
accumulation and returns infinity on `u128` overflow. PHP can retain finite values beyond that limit.
For example, decimal `18446744073709553665` and hexadecimal `0x10000000000000801` produce different
floats in PHP, so a single integer-to-float conversion cannot replace radix accumulation.

`pun-analyse/tests/fixtures/duplicate-array-keys/capture-wide-integers.php` captures the complete public
level-0 analyser output and PHPParser literal kinds, float bits, raw bytes, and spans. The public
`duplicate_array_keys` tests check direct and copied keys, global and class initializers, rounding,
negative values, separators, finite values above `u128`, and infinity. Run them with
`cargo test -p punphp-analyse --test duplicate_array_keys`. The existing analyser parser tests and
lint numeric-admission tests retain their original expectations.

Mago's `sorted-integer-keys` rule still orders lexical integer tokens using its pinned `u64` values.
Its Pun port recovers that value from the preserved raw bytes, excluding decimal-point and exponent
spellings. Fresh Mago 1.57.0 captures in `pun-lint/tests/mago/style-require-sorted-integer-keys/Own/wide.*`
cover all radices, separators, clamping and ordinary-float negative controls. Existing captures are unchanged.
