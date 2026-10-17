# Cross-crate requests

Open asks from one component to another. Add a line when you need an API you don't own; delete it when it lands.
Closed requests from before 2026-10-17 are in git history (`git log -- docs/REQUESTS.md`).

- **CI (C0):** `cargo fmt --all --check` gates now; clippy fails the job on errors but warnings don't gate (ci.yml "ponytail"
  comment). The cleanup pass took clippy to 0, so add `-- -D warnings`.
- **pun-cli -> pun-lsp (C10):** `pun lsp` should become `pun_lsp::run()`. Move `Engine2` / `fix_text` / `isolated` and the
  LSP config reload (`setup` / `stamp`) out of the binary into a library (pun-format or a small engine crate) that
  `pun format`, `--stdin` and pun-lsp share; pun-lsp has only the native subset today (no PHP-CS-Fixer delegation, no
  config reload). Same for `lint::RULES` (the `--mago` rule list), copied in crates/pun-lsp/src/pun.rs. Steps in
  context/reference/lsp.md "Wiring into `pun lsp`".
- **pun-format (from pun-lsp, C2/C10):** `LintError` could carry the line (and column) of a syntax error; pun-lsp parses
  "on line N." out of the message to place the diagnostic.
- **pun-config tiers (from migrate agent, C3):** pun.toml's top-level `preset` (the lint + analyse tier, RULES.md §5) is
  accepted and checked (`Tier` in lib.rs) but applies nothing. `pun migrate` writes `preset = "recommended"` in the
  starter pun.toml, and the harness runs laravel-laravel and larastan (no pint.json) on that starter, so applying the
  tier must not add apply-fixes there, or the starter must change.
