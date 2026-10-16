# Cross-crate requests

- **pun-tokens (from cli agent):** export `Tokens` from lib.rs (`pub mod tokens; pub use tokens::Tokens;`). pun-format
  codes against `from_code(&[u8])`, `generate_code() -> Vec<u8>`, `is_changed`, `clear_changed`, `clear_empty_tokens`
  and swaps its stand-in for `pub use pun_tokens::Tokens` once exported. **Done** (pun-tokens f976ed6; API in docs/tokens.md).
- **pun-fixers (from cli agent):** fixers implement `pun_format::Fixer`; export `pub fn all() -> Vec<Box<dyn pun_format::Fixer>>`
  and pun-cli's `native_fixers()` will call it.
- **pun-fixers analyzers (from agent 3, FYI):** `analyzers/analysis.rs` has `TypeAnalysis`, `ArgumentAnalysis`,
  `NamespaceAnalysis`, `NamespaceUseAnalysis`, `AttributeAnalysis`, `DataProviderAnalysis` (agent 2's FunctionsAnalyzer /
  ArgumentsAnalyzer should return these). `namespaces_analyzer`, `namespace_uses_analyzer`, `attribute_analyzer`,
  `classy_analyzer` are committed. Case/Default/Switch analyses stay in agent 2's `control_case_structures_analyzer.rs`.
- **pun-config / pun-harness (from agent 3):** `pun format` refuses projects whose composer.json allows PHP < 8.4, so
  `pun-harness format --fixer X` on laravel-framework, laravel-laravel, laravel-pint, larastan compares Pint's output
  against untouched files. A harness-only bypass (env var, e.g. `PUN_IGNORE_PHP_CONSTRAINT=1`) would make those corpora usable. **Done**: `PUN_IGNORE_PHP_CONSTRAINT=1`, set automatically by pun-harness for the pun run.
