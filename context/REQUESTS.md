# Cross-crate requests

Open asks from one component to another. Add a line when you need an API you don't own; delete it when it lands.
Closed requests from before 2026-10-17 are in git history (`git log -- docs/REQUESTS.md`).

- **CI (C0):** `cargo fmt --all --check` gates now; clippy still runs with `continue-on-error` (ci.yml "ponytail" comment).
  The cleanup pass took clippy to 0, so drop `continue-on-error` and add `-- -D warnings`.
- **pun-cli -> pun-lsp (C10):** `pun lsp` should become `pun_lsp::run()`. Move `Engine2` / `fix_text` / `isolated` and the
  LSP config reload (`setup` / `stamp`) out of the binary into a library (pun-format or a small engine crate) that
  `pun format`, `--stdin` and pun-lsp share; pun-lsp has only the native subset today (no PHP-CS-Fixer delegation, no
  config reload). Same for `lint::RULES` (the `--mago` rule list), copied in crates/pun-lsp/src/pun.rs. Steps in
  context/reference/lsp.md "Wiring into `pun lsp`".
- **pun-format (from pun-lsp, C2/C10):** `LintError` could carry the line (and column) of a syntax error; pun-lsp parses
  "on line N." out of the message to place the diagnostic.
- **rules/mapping.toml (C3):** `[upstream.pun]` keys are spelled `"Pun/no-only"`, but the fixers (and so every pint.json
  that enabled them) are named `"Pun/no_only"`, `"Pun/sensitive_parameter"`, ... Rename the 17 keys to the fixer names.
  pun-config normalises `-` to `_` in that section until then (`fixer_entries`); crates/pun-cli/tests/mapping.rs checks
  the names.
- **pun-config (from pun-cli agent output, C3/C4):** expose `[rules.lint]` entries (fix safety, summary) so pun-cli's agent
  output stops `include_str!`-ing `../../pun-config/rules/mapping.toml` (report.rs `lint_rule`; breaks `cargo publish`).
