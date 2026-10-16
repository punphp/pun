# pun-tokens

Rust port of PHP-CS-Fixer's token layer (`PhpCsFixer\Tokenizer`, pinned at v3.95.17, the
version bundled in the oracle Pint). Package `punphp-tokens`, imported as `pun_tokens`.

## Public API

Names are upstream's, snake_cased; semantics follow upstream function by function, so a
ported fixer reads like the PHP. Indices are `usize`; upstream's `?int` is `Option<usize>`.

| Rust | Upstream |
|---|---|
| `Kind::T_STRING`, `Kind::CT_ARRAY_BRACKET_OPEN`, `kind.name()` (`"T_STRING"`, `"CT::T_ARRAY_BRACKET_OPEN"`), `Kind::from_name` | `T_*` constants, `CT::T_*`, `Token::getNameForId`. Aliases: `Kind::T_PAAMAYIM_NEKUDOTAYIM`, `Kind::CT_ARRAY_SQUARE_BRACE_OPEN` and the other deprecated `CT::` names |
| `Token { id: Option<Kind>, content: Vec<u8> }`, `Token::new(kind, content)`, `Token::char("(")` | `new Token([T_X, 'x'])`, `new Token('(')` |
| `Proto`: `"(".into()`, `T_STRING.into()`, `(T_STRING, "foo").into()` | token prototypes `'('`, `[T_STRING]`, `[T_STRING, 'foo']` |
| `token.equals(proto, case_sensitive)`, `equals_any(&[..], cs)`, `equals_token(&Token, cs)` | `equals` / `equalsAny` (incl. the `&` special case) |
| `is_kind(k)`, `is_given_kind(&[k..])`, `is_array`, `is_comment`, `is_whitespace(Option<&[u8]>)`, `is_cast`, `is_classy`, `is_object_operator`, `is_keyword`, `is_magic_constant`, `is_native_constant`, `get_id`, `get_content`, `get_name` | same |
| `Tokens::from_code(&[u8])`, `from_array(Vec<Token>)`, `set_code`, `generate_code() -> Vec<u8>`, `generate_partial_code(start, end)` (inclusive) | same |
| `tokens[i]` (read), `tokens.set(i, token)` / `override_at` | `$tokens[$i]` / `$tokens[$i] = ...` (offsetSet: found-kinds + block-cache bookkeeping) |
| `len`/`count`, `iter`, `offset_exists`, `is_empty_at`, `clear_at`, `clear_range` (inclusive), `override_range(start, end, Vec<Token>)`, `insert_at(i, Vec<Token>)`, `insert_slices(BTreeMap<usize, Vec<Token>>)`, `clear_empty_tokens`, `clear_token_and_merge_surrounding_whitespace` | same |
| `ensure_whitespace_at_index(i, offset, ws) -> bool`, `remove_leading_whitespace(i, ws)`, `remove_trailing_whitespace(i, ws)` | same |
| `get_next_meaningful_token`, `get_prev_meaningful_token`, `get_meaningful_token_sibling(i, dir)`, `get_next/prev_non_whitespace(i, ws)`, `get_non_whitespace_sibling`, `get_next/prev_token_of_kind(i, &[Proto], cs)`, `get_token_of_kind_sibling`, `get_token_not_of_kind_sibling(i, dir, &[Proto])`, `get_token_not_of_kinds_sibling(i, dir, &[Kind])`, `get_non_empty_sibling` | same |
| `find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, i)`, `find_block_start`, `Tokens::detect_block_type(&token) -> Option<(BlockType, is_start)>`, `Tokens::block_edges(t)` | `findBlockEnd` etc., all `BLOCK_TYPE_*` constants incl. deprecated aliases. `&self` (cache is interior-mutable) |
| `find_given_kind(k, start, end) -> Vec<usize>`, `find_given_kinds(&[k], ..) -> BTreeMap<Kind, Vec<usize>>`, `find_sequence(&[Proto], start, end, \|key\| cs) -> Option<Vec<usize>>` | `findGivenKind`, `findSequence` (indices instead of `index => Token`) |
| `is_token_kind_found(Kind \| "content")`, `is_any_token_kinds_found`, `is_all_token_kinds_found`, `count_token_kind`, `is_changed`, `clear_changed`, `is_monolithic_php`, `is_partial_code_multiline`, `has_alternative_syntax` | same |
| `TokensAnalyzer::new(&tokens)`: `get_classy_elements() -> BTreeMap<index, ClassyElement>`, `get_classy_modifiers`, `get_import_use_indexes(per_ns)`, `is_array`, `is_array_multi_line`, `is_block_multiline`, `get_method_attributes`, `is_anonymous_class`, `is_lambda`, `get_last_token_index_of_arrow_function`, `is_unary_successor_operator`, `is_unary_predecessor_operator`, `is_binary_operator`, `is_while_part_of_do_while`, `is_enum_case`, `is_super_global` | `TokensAnalyzer` (not yet: `isConstantInvocation`, which needs AttributeAnalyzer/GotoLabelAnalyzer) |
| `transformers::tokenize_name` | `ImportProcessor::tokenizeName` |
| `php_compat::{strlen, strtolower, strtoupper, strcmp, strcasecmp, strnatcmp, strnatcasecmp, trim, rtrim, rtrim_chars, ltrim_chars}` | PHP string functions, byte-oriented, ASCII-only case mapping |
| `tokenize(&[u8]) -> Vec<RawToken { kind, text, line }>`, `apply_parse_mode(&mut [RawToken])` | `token_get_all($code)` / its `TOKEN_PARSE` retagging |

Where upstream throws on code it cannot handle (a null index after a failed lookup, an
unbalanced block in a transformer), the port panics at the same point; callers that want
upstream's "skip this file" behaviour wrap `from_code` in `catch_unwind`.

## Representation choices

- **Bytes, not `String`.** Content is `Vec<u8>`: PHP source may be invalid UTF-8 or contain
  binary after `__halt_compiler`, and PHP's string functions are byte-oriented anyway.
- **Kinds by name, not number.** `Kind` is a `u16` enum; PHP's token numbers change between
  versions, names don't. `id: None` models upstream's non-array tokens, whose identity is
  their content (`'('`, `'b"'`, the empty cleared token).
- **Bookkeeping kept.** The found-kinds registry (`isTokenKindFound`, used to short-circuit
  searches) and the block-edge cache are reproduced with upstream's invalidation rules,
  because a stale cache is observable. The static code-hash cache is not (pure optimization).
- **TOKEN_PARSE.** PHP-CS-Fixer tokenizes with `TOKEN_PARSE`, where the parser retags
  semi-reserved keywords used as identifiers (`Foo::class`, `function list()`,
  `case Default`, named args `default:`) to `T_STRING`. `parse_mode.rs` reproduces this with
  a context heuristic over neighbouring tokens covering every `zend_lex_tstring` grammar
  rule; it does not detect ParseErrors (upstream refuses such files).
- **Transformer order.** Priority descending, ties in the order `Transformers::createSingleton()`
  reports on the oracle machine (Finder order). No current transformer pair with equal
  priority interacts on the corpus.

## Lexer: own port, not mago-syntax

Phase 0 asked to map mago-syntax's lexer onto `token_get_all`. We wrote our own instead
(`lexer.rs`, a port of `Zend/zend_language_scanner.l` as driven by ext/tokenizer), because
Mago's lexer is parser-oriented and differs structurally, not just in naming:

- `T_OPEN_TAG` / `T_CLOSE_TAG` absorb one trailing newline in PHP; Mago splits it off.
- Mago ends `//` comments before the whitespace preceding `?>`; PHP keeps it in the comment.
- Mago has no `ST_LOOKING_FOR_PROPERTY` (`$a->class` must be `T_STRING`), no
  `T_AMPERSAND_(NOT_)FOLLOWED_BY_VAR_OR_VARARG`, no `T_BAD_CHARACTER`, no
  `__halt_compiler` tail as `T_INLINE_HTML`, no int-overflow → `T_DNUMBER`, and lexes
  removed casts (`(real)`) and cast whitespace differently.
- Mago returns errors and stops for unterminated comments/strings where `token_get_all`
  keeps going (`T_COMMENT` / `T_ENCAPSED_AND_WHITESPACE` to EOF).
- No line numbers; keyword context (`enum`, `yield from` with comments, `readonly`) differs.

A mapping layer would have re-implemented most of the scanner's state machine on top of
Mago's output. The port is ~900 lines and matches PHP everywhere we measured. Mago stays
the choice for parsing/analysis elsewhere in Pun.

Assumptions: `short_open_tag=Off` (php.ini default); PHP 8.5 token set.

## Parity

`cargo run --release -p punphp-tokens --example parity -- --mode raw|parse|fixer|classy <path>...`
runs `tools/dump.php` under system PHP 8.5.11 and diffs every `.php` file (`vendor/`,
`node_modules/` skipped). `raw` compares (kind, text, line) to `token_get_all`; `parse` to
`token_get_all(.., TOKEN_PARSE)`; `fixer` compares (kind incl. `CT::`, content) to
`PhpCsFixer\Tokenizer\Tokens::fromCode` from `.oracle/php-cs-fixer`; `classy` compares
`getClassyElements()`. `cargo test -p punphp-tokens` runs all modes on `tests/fixtures/`
(heredoc/nowdoc incl. flexible and nested, inline HTML, `?>` at EOF, `<?=`, `__halt_compiler`
with binary tail, CRLF, BOM, invalid UTF-8, attributes, property hooks, asymmetric
visibility, casts, number overflow, unterminated constructs, TOKEN_PARSE demotions, every CT
kind).

Results (files identical / total; "skipped" = PHP itself rejects the file under TOKEN_PARSE):

| corpus | files | tokens | raw | parse | fixer (post-transform) |
|---|---|---|---|---|---|
| pinkary (app, config, database, routes, tests, bootstrap, public, resources/views incl. Blade) | 483 | 163,731 | 483/483 | 484/484¹ | 484/484¹ |
| laravel/framework @86864f3 | 3,158 | 3,382,340 | 3,158/3,158 | 3,157/3,157 (1 skipped) | 3,157/3,157 (1 skipped) |
| pestphp/pest | 633 | 294,631 | 633/633 | 633/633 | 633/633 |
| larastan | 579 | 232,837 | 579/579 | 579/579 | 579/579 |
| laravel/pint | 336 | 70,850 | 336/336 | 335/335 (1 skipped) | 335/335 (1 skipped) |
| laravel/laravel | 27 | 6,176 | 27/27 | 27/27 | 27/27 |
| tests/fixtures | 23 | | 23/23 | all parseable identical | all parseable identical |

¹ `corpus/pinkary` (whole checkout) rather than the dir list; same result.

Mismatched tokens: 0 in every mode. `get_classy_elements` matches upstream on every file checked.

Timings (release, one thread, laravel/framework `src/`, 1.12M tokens): lexing 65 ms;
`Tokens::from_code` (lex + TOKEN_PARSE + 20 transformers) ~340 ms. PHP's `token_get_all`
loop takes ~1.8 s and PHP-CS-Fixer's `Tokens::fromCode` ~10 s on the same files.

## Known gaps

- `TokensAnalyzer::isConstantInvocation` and the `Analyzer\*` classes (Namespaces,
  NamespaceUses, Functions, Arguments, Attribute, ...) are not ported yet.
- `short_open_tag=On` is not supported.
- `strcmp` returns only the sign (PHP returns libc `memcmp`'s raw value); `strcasecmp`
  returns PHP's exact byte difference.
- Transformer tie order inside Pint's PHAR is assumed equal to the source checkout's.
