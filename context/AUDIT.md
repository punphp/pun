# Code Audit

Tick an item when it lands, and note the commit next to it.

- P0: broken today: wrong output, a panic on user input, unsoundness, or a parity break.
- P1: misleading today: an API, name or error that callers or contributors will get wrong.
- P2: inconsistent with siblings or other crates, or costly to change.
- P3: polish, duplication, dead code.

Audited 2026-10-17 at `90ee331` with `context/prompts/rust-audit.md`: every crate read in full in 8 slices, plus
`cargo clippy` with pedantic, nursery and 12 restriction lints (5,186 hits) and `cargo doc` (6 warnings). "Reproduced"
means the input was run against the code (a scratch crate calling the public API, or `cargo fmt`/`cargo clippy`);
"PHP" means checked with `php` 8.3 or the pinned PHP-CS-Fixer 3.95.17 source. `.oracle/pint` was not installed, so
items marked "not checked against Pint" still need one oracle run before the fix.

## P0: Broken

- [x] 1. `crates/pun-workspace/tests/finder.rs:107`: `pun_config::migrate::from_pint(&pint, None)` no longer compiles
  since fb790c3 added the `runs` parameter (`crates/pun-config/src/migrate.rs:25`), so `cargo test --workspace` and
  the CI test and coverage jobs fail before running anything. Pass `&|_| true` (or the `pun_fixers::all` check `pun
  migrate` uses). Reproduced (clippy compile error E0061). Spans crates. Fixed in 0c5bd30.
- [x] 2. `crates/pun-cli/tests/output.rs:591` and `:605` are not rustfmt-clean (last touched in 4e5a182), so CI's
  `cargo fmt --all --check` (`.github/workflows/ci.yml:29`) is red. Run `cargo fmt -p punphp`. Reproduced. Fixed in 0c5bd30.
- [x] 3. `crates/pun-config/src/options.rs:358-367` (`rejected`'s `entry` closure) and `:411-416` unwrap a bad entry
  that isn't there when a map or list option holds `null`, so `pun migrate` panics on
  `{"rules":{"class_attributes_separation":{"elements":{"method":null}}}}`,
  `{"phpdoc_tag_type":{"tags":{"param":null}}}` and `{"group_import":{"group_types":[null]}}` (also through
  `.php-cs-fixer.php`). Handle a `null` entry first with upstream's message and return `Option` from `entry`; add the
  three inputs as oracle cases. Reproduced. Pint stops these at Symfony's `verifyTypes` ("one of the elements is of
  type ..."), so `to_pun` now runs that check first. Fixed in 239e5f5, covered by 46af956.
- [x] 4. `crates/pun-phpstan/src/php.rs:134`: `normalize_path` slices `&original[1..3]` on a `&str`, so an
  `excludePaths` or `ignoreErrors.path` entry like `*/€uro/*` or `*€/a.php` panics ("byte index 3 is not a char
  boundary") in `FileExcluder::new` and `IgnoredErrors`. Compare bytes: `original.as_bytes().get(1..3) ==
  Some(b":\\")`. Reproduced. Fixed in 9c85148.
- [x] 5. `crates/pun-phpstan/src/phpfile.rs:33-36` (`eat`): `rest[..t.len()]` cuts a `&str` mid-character, so any
  included `.php` config holding non-ASCII text (`<?php return ["a€"];`) panics instead of returning "Unsupported
  PHP config". Use `rest.as_bytes().get(..t.len()).is_some_and(|p| p.eq_ignore_ascii_case(t.as_bytes()))`.
  Reproduced. Fixed in fdebef0.
- [x] 6. `crates/pun-phpstan/src/php.rs:94-95`: `preg` wraps an `A`-modifier pattern as `\G(?:body)` and
  `expect("compiles unanchored")`, but `#\Qfoo#A` (an unclosed `\Q` swallows the `)`) and `#(*ANYCRLF)a#A` (a verb
  must stay at offset 0) fail to compile, so an `ignoreErrors` message regex panics `config::resolve`. PHP compiles
  both. Return the error with `?`, close a trailing `\Q` with `\E`, and keep leading `(*VERB)`s before `\G`.
  Reproduced (PHP and Pun). Fixed in 9c85148 and c28d355.
- [x] 7. `crates/pun-fixers/src/fixers/pun_prefer_first_class_callable.rs:106-111`, `:155-163`: the arrow-function
  branch never checks the receiver against the parameters, so `fn ($x) => $x->equals($x)` becomes
  `$x->equals(...)`, which reads an undefined outer `$x`. Return `None` when `callee` is one of `params`, and add the
  case to the tests. Reproduced. Fixed in d226ab4.
- [x] 8. `crates/pun-fixers/src/fixers/pun_prefer_dedicated_status_assertion.rs:58-67` parses `0404` as decimal 404,
  so `assertStatus(0404)` (octal 260 in PHP) becomes `assertNotFound()` and changes what the test asserts. Skip
  literals with a leading `0`, or decode them like `switch_continue_to_break::base_dec`. Reproduced (PHP gives 260). Fixed in 250c5f1.
- [x] 9. `crates/pun-fixers/src/fixers/pint_laravel_blade/embedded_blade_masker.rs:56-70` returns
  `result.unwrap_or_default()` when the `<style|script>` regex fails, and `pint_laravel_blade.rs:65-71` then clears
  every token, so `pun format` empties the file. The pattern's `(?:"[^"]*"|'[^']*'|[^>])*` is ambiguous: a
  `<script>` tag with 8 quoted attributes and no closing tag in a 34 KB template exhausts the backtrack limit (PHP
  returns `null` with "Backtrack limit exhausted"). Return the input unchanged, or bail so the PrettierException
  path reports the file; do the same for `php_block_formatting.rs:255`, `:478`. Reproduced the regex failure in PHP;
  the empty write is read from the code. Checked at 0c5bd30: Pint (`.oracle/pint`) empties the same 34 KB file too
  (`(string) preg_replace_callback(..)` is `""`), and Pun's `RawPreg` fails on the same input, so Pun matches Pint
  today; keeping the input would break parity on purpose. `:255`/`:478` (`/;$/`, `/^<\?php\s*\n/`) cannot exhaust
  the limit. Needs a decision. Decided (2026-10-18): report the file as errored, like a PrettierException. The masker
  now fails the file with `preg_replace_callback(): Backtrack limit exhausted` and leaves it untouched (Pint empties
  it), a PrettierException fails the file too (it used to print to stderr and pass), and the two `php_block_formatting`
  patterns panic, failing only the file, if they ever fail. Fixed in f4e297f.
- [ ] 10. `crates/pun-cli/src/report.rs:208-219` (`caret_satisfies`): with base `0.8.0` (`first == 1`) an installed
  `prettier-plugin-tailwindcss` version string `"1"` gives `v = [1]`, `v >= b` holds, and `v[..=1]` panics on the
  main thread outside `isolated`. Pad both to three parts before comparing. Traced by hand; the trigger is a
  malformed `package.json`.

## P1: Misleading

- [ ] 11. `crates/pun-cli/src/run.rs:224-238`, `report.rs:769-780` and `report.rs:32` (`FileResult.diff: String`)
  round-trip the byte diff from `php_diff::unified` through `String::from_utf8_lossy`, so `-v` and the
  txt/junit/xml diffs print U+FFFD for every non-UTF-8 byte of a Latin-1 file, and paths go through
  `to_string_lossy` (`run.rs:184-188`). Keep diffs and paths as bytes until the console boundary. Not checked against
  Pint.
- [ ] 12. `crates/pun-format/src/php_parser.rs:333-338`, `lib.rs:389-393` (`LintError.message: String`) and
  `fixer_annotation.rs:99-106` build parse-error and annotation messages with `from_utf8_lossy`, so a token cut at
  byte 30 inside `é`, or any Latin-1 byte, prints `�` where PHP prints the raw bytes. Keep messages as `Vec<u8>`
  until pun-cli writes them. Not checked against Pint. Spans crates. Reproduced at 88d4cfe against Pint: its `--format=*`
  ERROR lines and `-q` lines print the raw bytes (`\xc3...`, `caf\xe9`) where Pun prints `�` (Pint's own human
  output and `--format=agent` do not keep them). The annotation messages are never lossy: the pattern's values are
  ASCII, or whole UTF-8 characters under `/u`. Deferred: the fix (`LintError.message: Vec<u8>`, and pun-cli's
  `report.rs`/`run.rs`/`console.rs` writing those two lines through a byte `console::format_bytes`) touches pun-cli
  files another track is reworking; the validated patch is `pun-wf/scratch/audit-format-item12.patch`.
- [x] 13. `crates/pun-fixers/src/analyzers/utils.rs:8-15` (`camel_case_to_underscore(&str)`), called with
  `from_utf8_lossy` from `php_unit_method_casing.rs:108`, writes U+FFFD into a renamed test method where upstream
  keeps PHP's byte semantics (`mb_strtolower` gives `?`). Take `&[u8]` and port the substitution into `php_compat`.
  Not checked against PHP. Spans crates. Reproduced with PHP-CS-Fixer: `testÉcole` in Latin-1 (`test\xC9cole`) gives
  `test_?cole` (Preg drops `u`, so 0xC9 is an uppercase letter); Pun wrote U+FFFD. Added
  `php_compat::mb_strtolower`. Fixed in 5a24bbc.
- [x] 14. `crates/pun-fixers/src/analyzers/preg.rs:487-520` panics with "PregException: error occurred when calling
  {method} with pattern ...", and drops `exec`'s error code (`:282-284`, `:512`). Upstream `Preg::newPregException`
  (`.oracle/php-cs-fixer/src/Preg.php:191-216`) prints "Unknown error occurred when calling {method}: {msg}." or the
  "Invalid PCRE pattern" form, so a file that hits the match or JIT-stack limit reports a different message. Carry
  the code out and format it like upstream. Checked against the PHP source. The messages now come from
  `preg_last_error_msg()` (backtrack, JIT stack, internal) or PHP's compile warning with `PCRE_VERSION`; the UTF-8
  and recursion codes cannot occur in Pun (UTF-8 is never matched unchecked, every pattern is JIT-compiled). Fixed
  in 5a24bbc.
- [x] 15. `crates/pun-fixers/src/analyzers/docblock/type_expression.rs:15-26` formats "Unable to parse phpdoc type '{}'"
  with `from_utf8_lossy`; upstream (`DocBlock/TypeExpression.php:486`) uses `var_export`, which escapes `\` and `'`,
  so every failing namespaced type (`\Foo\Bar<`) prints differently. Add a byte-exact `var_export` to `php_compat`
  and assert the full message. Checked against the PHP source. Spans crates. Added `php_compat::var_export_string`.
  Fixed in 5a24bbc.
- [ ] 16. Built-in symbols ignore the project's PHP version and disagree with each other:
  `crates/pun-phpstan/src/symbols.rs:236-248` (`class`, `function`) always read PHP 8.4, while
  `crates/pun-analyse/src/rules/mod.rs:1003-1005` (`builtins`) and `codebase.rs:64` use 8.5 and `members.rs:123` uses
  `symbols::class`. So 8.5-only methods (`Closure::getCurrent`) are missing, `phpVersion: 80400` still knows
  `array_first()`, and symbols.json (5.8 MB) is parsed twice per run. Take the version as a parameter and map
  `Project.php_version` to it once. Spans crates.
- [ ] 17. `crates/pun-analyse/src/rules/names.rs:553` reports `property.trait`, which `IDENTIFIERS`
  (`rules/mod.rs:43-98`) lacks, so `tests/rules.rs:57-61` counts Pun's correct errors as extra, the
  `examples/analyse.rs` rule list is short, and `testPromotedProperties` sits in `gaps.txt`. Identifiers are also built
  with `format!` (`names.rs:514`), which grep can't check. Replace Type Code with Enum, or at least test that every
  reported identifier is listed.
- [ ] 18. `crates/pun-cli/src/watch.rs:92-96` prints `{e}`, dropping anyhow's cause chain, so a broken pun.toml during
  `--watch` shows only "invalid /p/pun.toml" with no line or column, while the first run shows the chain. Errors are
  printed four ways (`main.rs:236-247`, `run.rs:53-60`, `pint_error.rs:12-14`, `watch.rs`). Extract one
  `report_error(&anyhow::Error)`.
- [ ] 19. I/O errors without the file: `crates/pun-cli/src/migrate.rs:352` (`fs::write(f, new)?`), `lint.rs:50`,
  `crates/pun-config/src/migrate.rs:120` print "Permission denied (os error 13)" alone; `pun-cli/src/migrate.rs:443-458`
  discards PHP's stderr, so a fatal error in `.php-cs-fixer.php` prints `pun: ` with nothing after it. Add
  `.with_context(|| path)` and fall back to stderr.
- [x] 20. `crates/pun-fixers/src/fixers/pint_laravel_blade.rs:140-150` (`resources_in`) writes the Prettier worker
  through a temp name that is the same for every rayon thread (`worker.<pid>`), so on a cold temp dir two threads can
  truncate each other's file and one `rename` fails, reporting a Blade file as errored. The guard also checks only
  `worker.cjs`. Build the files once per process in a `OnceLock`. Likely; not reproduced. Reproduced at 0c5bd30: 16
  threads on a fresh dir fail the rename with ENOENT. Fixed in e48bd81.
- [x] 21. `crates/pun-tokens/src/tokens.rs:23-27`: the derived public `Default` leaves `found_kinds` empty, so
  `Tokens::default().is_token_kind_found(..)` and `insert_at` index out of bounds (`:681`, `:860`). No caller today.
  Implement `Default` as `from_array(Vec::new())`, or make `found_kinds` a fixed-size array. Reproduced (index 134 out of bounds); `Default` now sizes `found_kinds`. Fixed in 84dc274.

## P2: Inconsistent or costly to change

- [x] 22. Fixer `configure` messages written by hand drift from PHP-CS-Fixer's (checked with the pinned fixer):
  `blank_lines_before_namespace.rs:133-141` drops ", but is of type \"string\"";
  `general_phpdoc_tag_rename.rs:69-78` says `"array<string, string>"` where upstream says `"string[]"` and prints
  JSON where upstream prints `array`; `binary_operator_spaces.rs:664-703` drops "Accepted values are: ...", the
  "expected any of" list and `string#`/`integer#` keys; the same `{v}`-as-JSON pattern is in
  `types_spaces.rs:39-41`, `ordered_class_elements.rs:182-184` and `final_internal_class.rs:88-90`. pun-config
  validates pun.toml first, so these surface only where its checks are looser, and the fixers' tests assert only
  `is_err()` (`method_argument_space.rs:439-447`). Make `Resolved::invalid`, `type_error` and `format_value`
  (`analyzers/fixer_configuration_resolver.rs`) `pub(crate)`, use them, and record the upstream messages as oracle
  cases. Recorded 40 oracle cases (configure errors, including the int-key ones of item 63); binary_operator_spaces
  also printed `string#7` for an int key where upstream prints `integer#7`. Fixed in c9adf6f.
- [ ] 23. `crates/pun-fixers/src/fixers/ordered_class_elements.rs:473-476` panics ("no position for
  property_public_static_readonly") on modifier mixes the grammar accepts and PHP's compiler rejects (`public static
  readonly int $x;`, `readonly public function f()`). Reproduced. Decide whether to mirror upstream's undefined-key
  behaviour or skip the element. Needs a decision.
- [ ] 24. `crates/pun-fixers/src/fixers/general_phpdoc_tag_rename.rs:147-173` puts replacement keys into the pattern
  unquoted and unwraps the key lookup, so `{"replacements": {"a|b": "c"}}` with `/** @a */` panics inside `fix()`.
  Fall back to the matched text, or mirror upstream's error. Needs a decision.
- [x] 25. `crates/pun-fixers/src/fixers/pint_laravel_blade.rs:235-238` unwraps both worker markers in Node's output; a
  template containing `[PINT_PRETTIER_WORKER_END]` desyncs the reply stream and the next file panics. Bail with
  Pint's "worker terminated unexpectedly" message and search END after START. Reproduced at 0c5bd30 with a stand-in
  worker; Pint's own message for a missing marker is "did not return a valid response" (`Prettier::format`), and it
  keeps the worker, so Pun does the same. Fixed in e48bd81.
- [ ] 26. Run configuration lives in process globals: `crates/pun-fixers/src/analyzers/active_preset.rs:6-18`
  (`static PRESET: RwLock`, set twice from `pun-cli/src/run.rs:83`, `:367`, read by the Blade fixer) and
  `pun-cli/src/agent.rs:16` (`MODE: OnceLock`, read at 11 sites, `expect`s it was initialised). A fixer's output
  depends on whichever `set` ran last. Pass the preset into the pipeline and an `OutputMode` down from `main`. Spans
  crates. Needs a decision.
- [ ] 27. Libraries read harness environment variables and their tests set them with `unsafe { set_var }` while other
  test threads call `temp_dir()` (a `getenv`): `crates/pun-format/src/lib.rs:333-337` with tests at `:755-764`, and
  `crates/pun-config/src/lib.rs:495` with tests at `tests.rs:581-583`. Read each variable once in pun-cli and pass a
  field (`Pipeline::ignore_mismatched_rules`, `Options::ignore_php_constraint`); the `unsafe` blocks go. Spans crates.
- [ ] 28. No shared lint policy: the root `Cargo.toml` has no `[workspace.lints]`, clippy doesn't gate in CI
  (`ci.yml:30-32`), and nothing compiles the `mago-lint` feature that release builds ship
  (`crates/pun-cli/Cargo.toml:15-16`; no test passes `--mago`). Add the block below, `[lints] workspace = true` in each
  crate, `--all-features` to the clippy job and one `pun lint --mago` smoke test. Spans crates. Needs a decision.
- [ ] 29. `crates/pun-cli/src/main.rs:210` and `lint.rs:24`: `pun lint --mago` parses every `FormatArgs` flag and then
  drops them, so `--dirty`, `--fix` and `--format=json` silently do nothing. Give it its own args or
  `conflicts_with_all`.
- [ ] 30. Report formats are strings: `main.rs:133-141` (`Option<String>`, help lists 3 of 7), `report.rs:662-745`
  (`match format`), `run.rs:317` and `report.rs:260,297` (`format.is_none()` as a mode), `list_files.rs:23,37-41`, and
  `report.rs:371-379` compares `agent::cmd()` with `"format"`. Replace Type Code with Enum `ReportFormat` (`FromStr`),
  parsed where `emit` runs today so Pint's "Format [x] is not supported." keeps its place.
- [ ] 31. `crates/pun-cli/src/pint_error.rs:59-120` recovers error kinds by matching message text produced in
  `run.rs:203,259-262`, `pun-workspace/src/lib.rs:87,215,252,263` and pun-format, so rewording any of them silently
  changes the Pint box's class, file and line. A caller branches on the kind, so a typed `PintError` downcast with
  `downcast_ref` earns its keep. Spans crates.
- [ ] 32. `crates/pun-cli/src/run.rs:50-364`: `format` is 315 lines with a 100-line `process` closure, returns
  `type Processed = (FileResult, Option<String>, bool, Option<FileResult>)` read as `.0`/`.2`, and
  `pint_error.rs:123-124` is generic only to accept that tuple; `lint_fixers: Vec<(Rule, bool, bool)>` and
  `Stage::Lint(bool)` are bools as modes. Introduce Parameter Object `Processed`, Extract Function `resolve_rules`,
  `select_files`, `process_file`, and name the bools.
- [ ] 33. `crates/pun-cli/src/migrate.rs:148-393`: `run` is 245 lines covering detection, prompts, planning, the
  summary table, conversion, clash checks, ignore-comment rewrites, writes and two output modes, and rebinds `from`
  and `toml` with new meanings (`:264`, `:280`, `:355`). Extract Function per phase and return a `Report` the two
  printers render.
- [ ] 34. `crates/pun-cli/tests/agent.rs:366` and `watch.rs:194` fake a read error with mode `000`, which root can
  read, so on this machine (uid 0) `errors` fails and `watch_reformats_like_pint` waits 60 s and fails. Use a
  directory or a dangling path instead. Likely; not run.
- [x] 35. `crates/pun-tokens/src/tokens.rs:14-15`: `BlockType(pub u8)` accepts 0 and 16, which panic in
  `block_edges` (`:73-126`); the `1..=15` range is copied in `tokens.rs:131`,
  `pun-fixers/src/analyzers/blocks_analyzer.rs:24` and `fixers/class_reference_name_casing.rs:78`; and
  `detect_block_type` rebuilds up to 30 prototypes per call on per-token paths (`tokens_analyzer.rs:164`, `:245`).
  Replace Type Code with Enum (`#[repr(u8)]`, `BlockType::ALL`, keeping the `BLOCK_TYPE_*` consts) and match
  directly. Spans crates. Reproduced (`BlockType(0)` compiles and panics; the api cases' `blockEnd 0`/`16` throw). `BlockType` is now a `#[repr(u8)]` enum, `detect_block_type` matches the token directly, `blockEnd 0`/`16` in the api test index `BlockType::ALL` and still panic like upstream. Fixed in 416ac92, 9b368e5.
- [x] 36. `crates/pun-tokens/src/tokens.rs:30-31`, `:244-261` keep a `dyn Any` cache that downcasts a pun-fixers
  type (`NamespaceAnalysis`), and a second user's failed downcast would be swallowed by `.ok()`. Move
  `NamespaceAnalysis` and `NamespacesAnalyzer::compute` (pun-tokens-only dependencies) into pun-tokens, where upstream
  keeps them, and type the cache. Spans crates. Holds: the cache took any `T`, and a call with another type would recompute and overwrite it on every call. `NamespaceAnalysis` and `NamespacesAnalyzer` now live in `pun-tokens/src/namespaces_analyzer.rs` (tested against PHP-CS-Fixer's NamespacesAnalyzer), `Tokens::namespace_declarations()` returns a typed `Arc<Vec<NamespaceAnalysis>>`, and `get_namespace_at` clones only the match (the `namespaces_analyzer.rs` part of item 88). Fixed in 475eb65.
- [x] 37. `crates/pun-format/src/php_parser.rs:440-442` copies the reduction's right-hand side with `.to_vec()` on
  every grammar reduction of every linted file; `values` is a local, so `&values[values.len() - len..]` works. Hot
  path; removes an allocation. Fixed in 2cb0bbf.
- [x] 38. `crates/pun-fixers/src/analyzers/preg.rs`: seven `unsafe` blocks (`:43`, `:54`, `:93`, `:182-239`, `:244`,
  `:268`, `:285`) with no `// SAFETY:` (clippy `undocumented_unsafe_blocks`), the 58-line block at `:182` mixes safe
  code with FFI, `Matcher::new` never checks the two `*_create` calls for NULL, and the `!Send` invariant of `Regex`
  is implicit. Add SAFETY comments, shrink the blocks to the FFI calls, return `Option` on NULL, and make `!Send`
  explicit. Confirmed at 74e2937 by reading: no SAFETY comments, unchecked `*_create` results. Each `unsafe` now holds one FFI call with a `// SAFETY:` comment, `Code` owns the compiled pattern, `Matcher::new` returns `None` when a `*_create` call returns NULL (freeing the other), and `tests::regex_is_not_send` fails to compile if `Regex` becomes `Send`. Fixed in wf/audit-fixers-perf.
- [x] 39. `crates/pun-fixers/src/analyzers/docblock/tag.rs:10,21-34` and `annotation.rs:36-37,91-99,172-192` cache
  in `RefCell<Option<Vec<u8>>>` and clone on every read; `support_types` calls `get_name` 18 times per annotation,
  so phpdoc_types and phpdoc_scalar clone up to ~37 vectors per annotation. Use `OnceCell` and return slices.
  Hot path. Confirmed at 74e2937. `Tag::get_name` is a `OnceCell` returning `&[u8]`, so `support_types` reads the name once instead of cloning it per tag. The `Annotation` type caches stay `RefCell`s: `set_types`/`remove` clear them through `&self` (annotations are borrowed from `DocBlock`'s `OnceCell`), so they still clone once per read. Fixed in wf/audit-fixers-perf.
- [x] 40. `crates/pun-fixers/src/analyzers/switch_analyzer.rs:6-40` byte-compares the whole file on every
  `belongs_to_switch` call from `ternary_operator_spaces.rs:34`'s per-`?`/`:` loop (O(tokens × colons)) and keeps a
  copy of the last file per rayon thread. Cache the colon indices on `Tokens`, invalidated on change. Spans crates. Confirmed at 74e2937 (`same_code` walks every token per `?`/`:`). `Tokens::switch_colon_indices` caches the sorted indices until the next edit, like `namespace_declarations` (dropped together in `drop_derived`); `belongs_to_switch` binary-searches them. Test: `tokens::tests::derived_values_cached_until_changed`. Fixed in wf/audit-fixers-perf.
- [x] 41. `crates/pun-fixers/src/analyzers/preg.rs:89-105`, `:494-503`: the per-thread pattern caches never evict,
  and `docblock/annotation.rs:77-88`, `:126-131` build patterns from docblock content, so memory grows with the
  number of distinct `@param` types in a project. Add an uncached entry point for content-built patterns. Confirmed at 74e2937. `Preg::match_groups_uncached` / `replace_limit_uncached` compile for the call only; `Annotation::get_variable_name` and `set_types` use them. Test: `preg::tests::uncached_patterns_are_not_kept` (values from PHP). Fixed in wf/audit-fixers-perf.
- [x] 42. `crates/pun-fixers/src/analyzers/preg.rs:534-573`: the `flags` argument of `match_groups`, `match_at` and
  `match_all` is dropped as `_flags` at all 48 call sites, including the one `PREG_OFFSET_CAPTURE`
  (`global_namespace_import.rs:489`). Remove Parameter. Confirmed at 74e2937 (48 call sites, all dropped). Removed the parameter and `PREG_OFFSET_CAPTURE`. Fixed in wf/audit-fixers-perf.
- [x] 43. Enum-valued fixer options are kept as `String` and compared in `fix()`, some per token:
  `braces_position.rs:19-28,77-85` (five fields looked up by name through a catch-all `_` arm),
  `align_multiline_comment.rs:13`, `cast_spaces.rs:10,56`, `concat_space.rs:10,80`,
  `class_attributes_separation.rs:22,64,167-177,398-426`, `method_argument_space.rs:14,16,252`,
  `multiline_whitespace_before_semicolons.rs:14,138`, `no_spaces_around_offset.rs:10`,
  `no_trailing_comma_in_singleline.rs:13,79-81`, `modifier_keywords.rs:40,59-70` (despite `ClassyElementType`),
  `no_unneeded_control_parentheses.rs:10,72-81`, `ordered_imports.rs:43` and `analyzers/import_processor.rs:10`
  (despite `NamespaceUseType`). A typo in a literal compiles and disables a branch. Parse once in `configure` into
  enums; siblings like `declare_strict_types::Strategy` already do. Confirmed at 74e2937. `Resolved::choice`/`choices` map an allowed value to its variant, keeping upstream's order for the error message; every listed option is an enum now (`class_attributes_separation` and `modifier_keywords` use `ClassyElementType`, `ordered_imports` and `ImportProcessor` use `NamespaceUseType`, `ordered_imports` `sort_algorithm` drops its `_` arm). New oracle error cases in `tests/cases/{cast_spaces,concat_space,modifier_keywords,no_spaces_around_offset,ordered_imports}.json`. Fixed in wf/audit-fixers-perf.
- [x] 44. Per-token lowercasing before checking: `native_type_declaration_casing.rs:51` (every token),
  `lowercase_keywords.rs:27`, `lowercase_cast.rs:29`, `no_alias_functions.rs:99`, `mb_str_functions.rs:65`. All but
  the last are in the Laravel preset. Test with `eq_ignore_ascii_case` / "has uppercase" first and allocate only on
  rewrite. Hot path. Confirmed at 74e2937. Each checks for an ASCII uppercase byte (or compares with `eq_ignore_ascii_case`) first and lowercases only for the rewrite. Fixed in wf/audit-fixers-perf.
- [x] 45. Pipelines are built far more often than needed: `crates/pun-format/src/lib.rs:214` calls `factory()` (all
  ~175 fixers) once per enabled rule, and `pint_laravel_blade/php_block_formatting.rs:57-79` builds one or two
  full-preset pipelines per Blade file, re-parsing the preset and swallowing errors (`unwrap_or_default()`, `.ok()`).
  That is ~22k boxed fixers per Blade file on Pinkary. Call the factory once and `take()` by name, and cache the Blade
  pipelines per preset in a `LazyLock`. Spans crates. The preset's parse errors are still swallowed, as before.
  Fixed in 9ce385c.
- [x] 46. Pest and Laravel `Pun/*` rules copy helpers that have already drifted: the global-call check in
  `pun_no_only.rs:99-105`, `pun_use_dedicated_expectation.rs:183-194`, `pun_use_simpler_expectation.rs:104-115` and
  `pun_use_specific_expectations.rs:103-105` (this copy accepts `Foo\expect(..)`); the matcher in
  `pun_use_dedicated_expectation.rs:154-201` and `pun_use_simpler_expectation.rs:84-117`; compound-assignment lists
  in three files. Extract one Pest helper module and drop the dead `T_NAME_FULLY_QUALIFIED` branch and the
  always-`true` `allow_not`. Fixed in 273f905: `analyzers/pest.rs` holds the global-call check, the
  chain-root walk and the `expect()` matcher; `analyzers/helpers.rs` holds `COMPOUND_ASSIGNMENTS`. `Foo\expect(..)` is
  no longer rewritten (Mago 1.57.0 leaves it too).
- [ ] 47. `ignoreErrors` entries are read by string key in `crates/pun-phpstan/src/config.rs:676-810`,
  `ignore.rs:83-492` and `baseline.rs:390-433` (58 literals), and `crates/pun-analyse/src/project.rs:47-100` re-digs
  `Resolved.parameters` by hand; `should_ignore` (`ignore.rs:276-283`) takes four `Option<String>`s rebuilt, with a
  fresh `FileExcluder` (two stats), for every (error, entry) pair. Parse each entry once into `IgnoreEntry` and add
  typed accessors on `Resolved`. Spans crates. Needs a decision.
- [ ] 48. Two ports of `ReflectionProvider::getClass()` answer differently: `crates/pun-analyse/src/rules/mod.rs:995-1000`
  (`class_like`, Mago's prelude) and `names.rs:51-65` (`known`, PHPStan's export, trims `\`). Keep `known` and move
  it with `function`/`builtins` into `reflection.rs`.
- [ ] 49. `crates/pun-analyse/src/rules/mod.rs` (1,005 lines) holds the identifier registry, 20 fields of checker
  state, the per-file driver, Mago type inference, guard bookkeeping (split with `scope.rs`) and reflection; the trait
  context is three fields that must change together (`:138-145`). Move Function into `driver.rs`, `infer.rs`,
  `reflection.rs` and `scope.rs`, and replace the three fields with `Option<TraitContext>`.
- [x] 50. `rules/mapping.toml` is parsed by slicing its text in eight places (`crates/pun-config/src/lib.rs:146-151,
  339-341, 371-376`, `schema.rs:272-273`, `options.rs:61-63`, `explain.rs:53-55, 117-121, 126-127`), each depending on
  section order and on no value line starting with `[`. Extract Function `section(name)` with one lazily parsed value
  per section. `crates/pun-config/src/mapping.rs` splits the file at the lines TOML reads as headers. Fixed in
  f498858.
- [ ] 51. Presets, ecosystems and fix safety are strings matched in several places: `pun-config/src/lib.rs:28-51,
  236-243`, `migrate.rs:194-198`, `schema.rs:25-33, 511, 581, 597, 226-233, 256` (`Gates::locked("doctrine")` falls
  through to PHPUnit), and `RuleMeta.fix` compared as `== "safe"` in `pun-cli/src/report.rs:398` although
  `pun_problem::Safety` exists. Replace Type Code with Enum (`FormatPreset`, `AnalysePreset`, `Ecosystem`,
  `FixSafety`) and build error lists from them. Spans crates. Partly done in 0be9ca3: `AnalysePreset` and
  `Ecosystem` (`Gates::locked` now matches all four, `doctrine` has no package). `FormatPreset` (`Config.preset`, read
  as a string in `pun-cli/src/run.rs` and `main.rs`) and `FixSafety` (`pun-cli/src/report.rs:398`) wait for the
  platform/format tracks that hold those files.
- [ ] 52. Unreadable files are treated as missing: `crates/pun-config/src/composer.rs:124-126` skips the PHP 8.4 gate
  on any read error, and `crates/pun-analyse/src/codebase.rs:36` with `rules/mod.rs:204-206` reports nothing for an
  unreadable file. Treat only `ErrorKind::NotFound` as absent and propagate the rest with the path. The
  `composer.rs` part (reproduced with a directory named composer.json) is fixed in afe5778.
- [ ] 53. The PHP target version has six representations: `Config.php: String`, `(u64, u64)` in
  `pun-config/src/schema.rs:220,264`, `composer::Version`, `constraint::V` (bridged by `From`), and `u32` ids in
  `pun-lint/src/lib.rs:28,136` and `pun-analyse/src/project.rs:21`. One `PhpVersion` in pun-config with `FromStr`,
  `Display` and `to_id()`. Spans crates. Needs a decision.
- [ ] 54. `crates/pun-lint/src/lib.rs:25,164,217` and `pun-problem`'s `apply_fixes` take source as `&str`, so a
  Latin-1 file that `pun format` handles can't be linted (the examples `continue` past it). Take `&[u8]`, as Mago's
  parser and `Pipeline::run` do. Spans crates.
- [x] 55. `crates/pun-config/src/constraint.rs:13,32,78,160` and `options.rs:134,278` return `Result<_, String>` that
  every caller converts (`composer.rs:118`, `lib.rs:270`, `migrate.rs:255`) and no caller branches on. Return
  `anyhow::Result` and use `.with_context` for the `{at}: ` prefix. They return `anyhow::Result` now, but the
  prefixes stay in one message: `pun format --watch` prints `{e}` (`crates/pun-cli/src/watch.rs:94`), which would
  drop a context's cause; switch to `.with_context` once it prints `{e:#}`. Fixed in 0b5ca8b.

## P3: Polish

### Duplicates to remove

- [ ] 56. Newline counting (`substr_count($s, "\n")`) is written out 14 times (clippy `naive_bytecount`):
  `blank_line_before_statement.rs:49-51`, `class_attributes_separation.rs:76-78`, `blank_lines_before_namespace.rs:52`,
  `single_line_after_imports.rs:13-17`, `no_whitespace_in_blank_line.rs:49`, `no_blank_lines_after_class_opening.rs:18`,
  `no_blank_lines_after_phpdoc.rs:14`, `fully_qualified_strict_types.rs:428,432,501`, `nullable_type_declaration.rs:120`,
  `nullable_type_declaration_for_default_null_value.rs:160`, `pint_phpdoc_type_annotations_only.rs:90-97`,
  `embedded_blade_masker.rs:41`, plus `pun-tokens/src/parse_lexer.rs:44` and `pun-phpstan/src/comments.rs:178`. Add
  `php_compat::substr_count`. Spans crates.
- [ ] 57. Other PHP functions have several ports, some already disagreeing: `strpos` (`binary_operator_spaces.rs:83`,
  `pint_laravel_blade/php.rs:6`, `pun-phpstan/src/comments.rs:182`), `explode` (`full_opening_tag.rs:14`,
  `php_unit_method_casing.rs:164`), `substr`, `preg_quote` (`pun-workspace/src/lib.rs:552-561`,
  `pun-phpstan/src/php.rs:104-119`, `pun-fixers/src/analyzers/preg.rs:672-689`), `fnmatch`
  (`pun-workspace/src/lib.rs:448-483` builds a regex and rejects `a[`, `pun-phpstan/src/php.rs:122-130` uses libc and
  matches it literally), PHP-delimited regex compile (`php.rs:8-96` vs `pun-workspace/src/lib.rs:515`), and
  `float_to_string` (`pun-phpstan/src/value.rs:280` vs `pun-tokens/src/php_compat.rs:208`, which panics on INF). Move
  each into `pun_tokens::php_compat` once; `pint_laravel_blade/php.rs` becomes re-exports. Spans crates.
- [ ] 58. `Utils::naturalLanguageJoin` is hand-written six times and the copies differ on one item:
  `binary_operator_spaces.rs:679-686`, `final_internal_class.rs:131-142`, `phpdoc_return_self_reference.rs:36-41`
  (gives ` and "a"`), `ordered_imports.rs:47-53`, `phpdoc_order.rs:44-48`, `pun-config/src/options.rs:313-320`; PHP's
  int-key test is in `general_phpdoc_tag_rename.rs:32-34` and `class_attributes_separation.rs:105` but missing from
  `binary_operator_spaces.rs:685`. Extract into `analyzers/utils.rs`. Spans crates.
- [ ] 59. `crates/pun-fixers/src/analyzers/docblock/mod.rs:128-142` (`replace_all`) is `php_compat::str_replace`
  (`pun-tokens/src/php_compat.rs:69-83`) with arguments reordered; five callers. Both loop forever on an empty
  needle, where PHP returns the subject. Delete the copy and add the empty-needle guard. Spans crates.
- [ ] 60. Insertion-ordered PHP arrays are emulated by hand six times: `fully_qualified_strict_types.rs:54-71`
  (`OMap`), `global_namespace_import.rs:43-63`, `lambda_not_used_import.rs:10-21`, `ordered_class_elements.rs:189-200`,
  `no_extra_blank_lines.rs:299-305`, `ordered_imports.rs:268-273`. `indexmap` is already a workspace dependency; keep
  order-sensitive equality with `iter().eq`.
- [x] 61. `Preg` and `RawPreg` repeat the match, match-all and replace loops (`preg.rs:550-555` vs `829-834`,
  `566-572` vs `845-851`, `632-646` vs `862-874`). Extract Method on `Regex`. Confirmed at 74e2937. `Regex::captures_at`, `captures_all` and `replace_with` hold the loops; both `Preg` and `RawPreg` call them. Fixed in wf/audit-fixers-perf.
- [x] 62. Option defaults are written in both `fixer()` and `configure()` in about 24 fixers (for example
  `increment_style.rs:14/35`, `ordered_imports.rs:31-33/72-110`, `yoda_style.rs:27/48-50`,
  `trailing_comma_in_multiline.rs:20-23/43`), while `phpdoc_align`, `method_argument_space` and others call
  `configure(&Value::Null)` in `fixer()`. `Pipeline::new` always configures, so use that one pattern everywhere.
  Every configurable fixer's `fixer()` now builds neutral fields and calls `configured()` (`configure(Null)`), and
  phpdoc_return_self_reference's `configure` sets its default when the option is absent (it relied on `fixer()`).
  Pinkary and pinkary-dirty: 0 differing files. Fixed in c9adf6f.
- [x] 63. Four configures walk "a PHP array as list or map" by hand with unwraps after `check_types`:
  `phpdoc_align.rs:96-111`, `phpdoc_return_self_reference.rs:53-66`, `phpdoc_separation.rs:33-62`,
  `phpdoc_tag_type.rs:84-93`. Add `Resolved::php_array(key)`. `php_array` also reads int-like object keys as PHP int
  keys, which fixed phpdoc_return_self_reference (`string#7` for `integer#7`) and phpdoc_tag_type (no TypeError for
  `{"0": ...}`); class_attributes_separation uses it too. Fixed in c9adf6f.
- [ ] 64. Small helpers copied inside crates: `c`/`k`/`idx` in `pun-tokens/src/transformers.rs:51-69` and
  `tokens_analyzer.rs:9-14,66-81`; `CLASSY_KINDS` in `fully_qualified_strict_types.rs:26` and
  `pun-tokens/src/token.rs:82`; the PCRE `\s` byte set in `docblock/line.rs:52`, `tag.rs:50`, `annotation.rs:209`
  and private `php_compat::isspace`; `is_numeric` (`pun-phpstan/src/types.rs:982`, `value.rs:306`); `truncate`
  (`types.rs:1050`, `schema.rs:380`); the scheme test in `php.rs:136-142` vs `has_scheme` (`:184`); the PHPUnit
  method list twice in `ordered_class_elements.rs:121-132,360-374` (the second rebuilt per method). Keep one of each.
- [ ] 65. Analyse rules repeat message blocks: the private/protected wording (`calls.rs:174-185,344-355`,
  `classes.rs:220-231`, `names.rs:212-223`), the `outOfClass.*` identifier (four copies), the parent lookup with
  `class.noParent` (three copies), and `String::from_utf8_lossy(x.original_name.as_bytes())` (15 copies); Mago's own
  `Argument::value()` and `ArrayElement::get_value()` are re-implemented in `scope.rs:131-155`,
  `laravel/schema.rs:70-75` and `pun-lint/src/rules/no_unused_catch_variable.rs:128-174`; the parse triple is copied
  six times in pun-analyse. Extract Function and call Mago's methods.
- [ ] 66. Test helpers are copied across files and have drifted (some clear the environment, some don't; some
  `snapshot` copies skip different entries): `oracle()`, temp-dir builders, `snapshot()`, `reset()`, `migrate()` and
  `pun` spawners across `crates/pun-cli/tests/{cli,flags,output,watch,agent,perf,migrate,rules_e2e}.rs` and
  `pun-workspace/tests/{finder,files}.rs`; doTest in `pun-fixers/tests/cases.rs:21-77` and `fixtures.rs:230-275`
  (cases.rs drops the "marked changed" check); `tests/blade.rs:217-229` re-implements `Pipeline::run` without
  `supports`; one 9-line `fix(code)` helper in all 17 `pun_*.rs`; four PHP file walkers in `pun-lint/examples/`. One
  `tests/common/mod.rs` per crate.
- [ ] 67. Small duplication in pun-workspace and list-files: `PINT_NOT_NAME` built twice (`lib.rs:69-72`, `:419-422`),
  the canonicalize-or-"not readable" block twice (`:212-216`, `:261-264`), and `CommandArg` mirroring
  `pun_workspace::Command` (`pun-cli/src/list_files.rs:30-54`).
- [ ] 68. Crate manifests repeat `homepage`, `keywords` and `categories` in every crate instead of
  `[workspace.package]`; `pprof` (`pun-cli/Cargo.toml:24`) and `pcre2-sys` bypass `[workspace.dependencies]`;
  `pun-fixers/Cargo.toml:15-17` repeats the `pcre2` and `libc` versions, uses `pcre2` only in a test, and uses `libc`
  only for `c_int`/`c_void` (in `std::ffi`); the root has no `rust-version` while CI uses unpinned `@stable` and the
  Mago crates need rustc 1.99 (this audit had to update from 1.97).

### Dead code

- [ ] 69. `crates/pun-phpstan/src/types.rs` (3,377 lines), `resolve.rs` (665), `phpdoc.rs` (707), `baseline.rs` (131)
  and the NEON encoder (`neon.rs:588-805`) have no production caller (`rg 'pun_phpstan::(types|resolve|phpdoc|baseline)'`
  finds only tests and one helper in `pun-analyse/tests/laravel.rs`), with ~208k fixture lines behind them, and
  `context/plan/analyse.md` §2 says the type model is no longer needed. Delete them, or split `types.rs` along its
  concerns (Tri, model, queries, describe, ConstArray, super-type, combinator) if they stay. Needs a decision.
- [ ] 70. `crates/pun-cli/src/render/` (~1,500 lines) is compiled under `#![allow(dead_code, unused_imports)]`
  (`mod.rs:7-8`) and is the only user of `pun-problem` in pun-cli. It waits for output.md Build 3; switch to
  `#![expect(..., reason = "output.md Build 3")]` so it fails once wired, or gate it behind `cfg(test)`.
- [ ] 71. `crates/pun-format/src/lib.rs:366-376` (`pun_ignored`, for the removed delegate mode) has no caller, and its
  first doc line belongs to `has_annotation` (`:378`), which has none. Also unused outside tests: `Kind::is_custom`,
  `php_compat::{strtoupper, strcmp, strnatcasecmp}`, `token::is_native_constant`, the names in `TRANSFORMERS`
  (`transformers.rs:15-39`). Needs a decision on test-only ports.
- [ ] 72. Unused fields and parameters: `no_extra_blank_lines.rs:40,48,307` (`tokens_config`, written never read);
  `no_unneeded_control_parentheses.rs:593-600` (always `Some`) and its dead `Option` parameter; `method_argument_space.rs:211`
  (`_override`, passed `true`/`false`); `embedded_blade_masker.rs:350-351` (`length`); `pun-lint/src/lib.rs:24-29`
  (`Context.path`, `Context.php_version`); `pun-lint/Cargo.toml:25` (`serde_json` dev-dependency); the orphan comment
  at `no_unneeded_control_parentheses.rs:83`.
- [x] 73. Forwarding wrappers: `ordered_imports.rs:153-155` (`spaceship`), `no_alias_functions.rs:56-58` (`merge`),
  `import_processor.rs:59-61` (`tokenize_name`), `type_expression.rs:358-364`, `helpers.rs:28-30` (`has_newline`,
  used twice while 28 sites call `Preg` directly), `pun-phpstan/src/types.rs:288-290`, and all of
  `docblock/preg.rs` (49 lines renaming `Preg` methods). Inline Function. Fixed in fe1af67 (`has_newline` now has 7 call sites, all
  inlined; in `types.rs` the forwarded `is_implicit_mixed` went and `is_plain_implicit_mixed` keeps its body).
- [ ] 74. `crates/pun-cli/src/run.rs:480-487`: `assert!(thread::spawn(|| panic!()).join().is_err())` passes whatever
  the panic hook does. Delete it or assert the hook's effect.

### Types

- [ ] 75. Tuples and sentinels that hide meaning: `pun-analyse/src/rules/params.rs:79` (`(usize, bool, bool)`),
  `params.rs:119-170` (`Call.kind: &'static str`), `params.rs:99` (`Option<Result<usize, ()>>`), `rules/mod.rs:111-115`
  (`method_static: Option<bool>`), `pun-config/src/schema.rs:443` (`(Level, bool, Value, bool)`),
  `preg.rs:396-440` (`Captures::offset` returns `-1` that 11 callers cast to `usize`), `pun-tokens/src/lexer.rs:68-88`
  (`need_tokens: i32 = -1`), and signed-index casts in `tokens.rs:209-211,404-407,806-833` and
  `abstract_php_unit_fixer.rs:21-24` (use `checked_add_signed`/`checked_sub`). Name them with structs, enums and
  `Option`.
- [ ] 76. `crates/pun-tokens/src/token.rs:8-11,202-208`: `Token` has public fields and getters for them (pun-fixers
  reads `.content` 213 times and `get_content()` 246 times), and struct literals skip `Token::new`'s check. Pick one.
  Needs a decision.
- [x] 77. Three `Error` types in pun-phpstan (`neon::Error(pub String)` with no `Display`, `ignore::Error`,
  `baseline::Error<'a>` for the same PHPStan concept), and `trait Files` (`di.rs:257-262`) with one impl whose
  default `is_file` reads the whole file, so a large baseline is read about four times per resolve. The read-based
  `is_file` also skipped an unreadable `phpstan.neon` for `phpstan.neon.dist`, where PHPStan fails to read it. Fixed in
  fa5a93c.
- [x] 78. `crates/pun-fixers/src/analyzers/classy_analyzer.rs:14`, `constant_invocation.rs:10`, `utils.rs:21` format
  `Option` with `{:?}`, printing `got "Some("T_OPEN_TAG")"` where upstream prints `got "T_OPEN_TAG"`, and the test
  at `classy_analyzer.rs:122` locks it in. A token without an id now prints `""`, as upstream's null name does. Fixed
  in 5a24bbc.

### Errors and tests

- [ ] 79. `crates/pun-harness/src/main.rs:82-171` leaks two corpus copies in `/tmp` on any `?` after `copy_tree`
  (its own `missing_pieces` test triggers it), and `process::exit(1)` at `:169`. Use a Drop guard and
  `main() -> Result<ExitCode>`.
- [ ] 80. Tests that share fixed paths or leak state: `pun-cli/tests/rules_e2e.rs:31-45` and `perf.rs:31-40` write ~200
  cache files into the real `~/.cache/pun` per run; `pun-phpstan/tests/schema.rs:13` and `config.rs:149-167` reuse
  fixed temp dirs across concurrent runs; `pun-fixers/src/analyzers/active_preset.rs:22-27` mutates a global;
  `docblock/tests.rs:46-59` swaps the process panic hook. Use per-run temp dirs, `env_clear()` and `catch_unwind`.
- [x] 81. Oracle tests disagree on a missing `php`: `pun-tokens/src/php_compat.rs:356-378` says "skipped" but panics,
  while `tests/parity.rs:10-16` and `tests/api.rs:285-296` return silently and pass. One helper, one policy. Reproduced with `php` off PATH: the two
  `php_compat` tests panicked, `api` and `parity` passed without comparing anything. Policy: the oracle is required
  (CI installs it), so `tests/common/mod.rs` (`php()`, `require_php_cs_fixer()`) fails every PHP-backed test with
  "run scripts/oracle.sh"; the `php_compat` comparisons moved to `tests/php_compat.rs` to share it. Fixed in 02a3868.
- [x] 82. Weak or stale assertions: `pun-format/src/lib.rs:595-603` checks only a `"Parse error: "` prefix under a
  comment about Mago, which lint no longer uses (also `context/reports/COVERAGE.md:38`); `docblock/tests.rs:332-343`
  uses `contains`. Assert the oracle's exact message. The pun-format test now asserts PHP 8.5's exact messages and the
  stale Mago note is gone from COVERAGE.md; the docblock message is left to item 15, whose exact-message test (oracle
  values, `var_export` escaping) lands with that fix. Fixed in 7ab5e7f.

### Splits and smaller items

- [ ] 83. `crates/pun-cli/src/report.rs` (780 lines) holds the Symfony error box, the Node/Prettier probe and semver,
  the terminal summary and three reporters while the other four live in `fix_report.rs`, and it imports
  `pint_error`, which imports it back. Move `abort_box` to `pint_error.rs`, the probe to `prettier.rs`, the reporters
  together.
- [x] 84. `crates/pun-fixers/src/fixers/final_class.rs:3` imports `final_internal_class`, the only fixer-to-fixer
  import, so `PUN_FIXERS_ONLY=final_class` fails to build. Move the shared core into `analyzers/`. Fixed in edb08b1.
- [x] 85. Per-token clones and pre-check work on Laravel-preset paths: `braces_position.rs:148`,
  `array_indentation.rs:85`, `no_alternative_syntax.rs:38`, `method_argument_space.rs:404-405`,
  `multiline_whitespace_before_semicolons.rs:136-137` (also computes an unused indent per `;`),
  `no_trailing_whitespace_in_comment.rs:30`, `binary_operator_spaces.rs:280-281,411`; `Vec`s built in `is_candidate`
  (`magic_constant_casing.rs:33-34`, `no_spaces_after_function_name.rs:47-49`); sort keys recomputed per comparison
  (`ordered_imports.rs:166-189`, `ordered_traits.rs:265-285`); a regex rebuilt per comment (`no_break_comment.rs:166-175`)
  and per parse (`type_expression.rs:267-273`); `insert_slices` splicing back to front (`tokens.rs:625-628`); and
  `woc_ends` quadratic in a whitespace run (`lexer.rs:360-412`). Borrow the token, copy its id, or compute once. Confirmed at 74e2937 (all sites still there). Borrowed or copied the kind instead of cloning, `multiline_whitespace_before_semicolons` finds the indent only for chained calls, the `is_candidate` `Vec`s are gone, `ordered_imports`/`ordered_traits` compute each sort key once, `no_break_comment` builds its pattern in `configure`, `type_expression` keeps its three prefixed patterns in `LazyLock`s, `insert_slices` rebuilds in one pass and `woc_ends` tracks seen ends in a `HashSet`. Group 38-44/61/85, `pun check --test --no-cache` with the Laravel preset on laravel-framework (HEAVY_EXCLUSIVE, profiling build, median of 7): 1.966 s before, 1.829 s after; harness 0 differing files on pinkary, pinkary-dirty, laravel-framework and laravel-framework `--preset laravel`. Fixed in wf/audit-fixers-perf.
- [x] 86. `crates/pun-tokens/src/tokens.rs:42`, `kind.rs:234` and `pun-fixers/src/analyzers/preg.rs:14` carry
  `allow(non_upper_case_globals)` that suppresses nothing; nine `allow`s have no reason (clippy
  `allow_attributes_without_reason`). Delete them or switch to `#[expect(.., reason = ..)]`. Reproduced: the three
  `non_upper_case_globals` allows and `preg.rs`'s `len_without_is_empty` (an unfulfilled `expect` showed it) suppress
  nothing and are gone; the rest are `#[expect(.., reason)]`, or `allow` with a reason where a lint fires in only some
  builds (shared test modules, the generated grammar and its generator). `cargo clippy --workspace --all-targets -- -W
  clippy::allow_attributes_without_reason` reports none. Fixed in ac0ee93.
- [ ] 87. Visibility: 570 `unreachable_pub` hits, mostly the `pub fn fixer()`/`pub struct` items inside private
  fixer modules and pun-cli/pun-analyse internals; `pun-phpstan` makes every module `pub` (`di`, `schema`, `phpfile`
  internals have no outside caller); `docblock/regex.rs:4,13,187` exports three macros as the crate's public API;
  `pun-format/src/lib.rs:83` (`DEPRECATED`), `parse_mode.rs:20`, `tokens.rs:209`. `pub(crate)` them; the workspace
  lint in item 28 keeps them that way.
- [ ] 88. Small fixes: `abstract_no_useless_else_fixer::PRIORITY` used by one of its two users
  (`no_useless_else.rs:17-19` hard-codes 39); `blank_line_before_statement.rs:146-153` looks each key up twice;
  `single_quote.rs:22-23,58` alone bypasses `Preg`; `namespaces_analyzer.rs:76-82` clones the whole cached list to
  find one entry; `build.rs:13-17` formats the path with `{}` (use `{:?}`); `pun-tokens/examples/parity.rs:3` names
  package `pun-tokens` (it is `punphp-tokens`); md5 is hand-written in `fix_report.rs:264-307` (the `md-5` crate gives
  the same bytes; Needs a decision); rustdoc warns about two broken links in `pun-analyse/src/rules/mod.rs:126,364`,
  three unclosed HTML tags in pun-fixers docs and a public doc linking a private item in pun-config.

### Lints

Proposed `[workspace.lints]` for item 28, from this audit's clippy counts (hits outside tests in brackets). Lints with
hundreds of hits that follow a convention are allowed; lints with few, real hits warn.

```toml
[workspace.lints.rust]
unsafe_op_in_unsafe_fn = "deny"
unreachable_pub = "warn"            # 570: mechanical, fix crate by crate

[workspace.lints.clippy]
pedantic = { level = "warn", priority = -1 }
# Conventions: upstream ports keep PHP's names, index loops and casts.
module_name_repetitions = "allow"
must_use_candidate = "allow"        # 157
missing_errors_doc = "allow"        # 36
missing_panics_doc = "allow"        # 29
doc_markdown = "allow"              # 285: PHP names in docs
too_many_lines = "allow"            # 37: ports mirror upstream methods
many_single_char_names = "allow"    # 67
cast_possible_wrap = "allow"        # 71: PHP's signed index loops
cast_sign_loss = "allow"            # 76
cast_possible_truncation = "allow"
cast_precision_loss = "allow"
similar_names = "allow"
unused_self = "allow"               # 29: ported private methods
enum_glob_use = "allow"             # 164: `use Kind::*` is the house style
# Restriction lints worth having:
undocumented_unsafe_blocks = "warn" # 7, all in preg.rs
allow_attributes_without_reason = "warn"  # 9
dbg_macro = "warn"
print_stdout = "warn"               # allow in pun-cli, pun-harness and examples
print_stderr = "warn"               # one real hit: pint_laravel_blade.rs:65
redundant_clone = "warn"            # 4
needless_collect = "warn"           # 4
naive_bytecount = "warn"            # 14, item 56
```

Not proposed: `unwrap_used` (829 outside tests, nearly all upstream throws or checked invariants), `indexing_slicing`
(811) and `as_conversions` (211); add them per crate once the P0 items above are fixed. `use_self` (503) and
`missing_const_for_fn` (118) are auto-fixable if wanted.
