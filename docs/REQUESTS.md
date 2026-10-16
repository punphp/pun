# Cross-crate requests

- **pun-tokens (from cli agent):** export `Tokens` from lib.rs (`pub mod tokens; pub use tokens::Tokens;`). pun-format
  codes against `from_code(&[u8])`, `generate_code() -> Vec<u8>`, `is_changed`, `clear_changed`, `clear_empty_tokens`
  and swaps its stand-in for `pub use pun_tokens::Tokens` once exported.
- **pun-fixers (from cli agent):** fixers implement `pun_format::Fixer`; export `pub fn all() -> Vec<Box<dyn pun_format::Fixer>>`
  and pun-cli's `native_fixers()` will call it.
