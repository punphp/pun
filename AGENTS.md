# Pun — agent notes

Plan: docs/PLAN.md. Home: github.com/punphp/pun (npm: @punphp/pun). Current scope: Phase 0 + Phase 1 (`pun format`, byte-identical to Pint).

- Rust toolchain: `source ~/.cargo/env`. Machine has 2 cores / 7 GB: build only your crate (`cargo test -p <crate>`), never `cargo clean`.
- Crate ownership is per agent. Don't edit another agent's crate; if you need an API from it, read it and adapt, or note the request in docs/REQUESTS.md.
- Root Cargo.toml: you may append to `[workspace.dependencies]`; don't reorder or remove.
- Commit only your own paths: `git add <your paths> Cargo.lock && git commit -m ...`. If `.git/index.lock` exists, wait and retry.
- Oracle = real tools via system PHP 8.5 (no Docker, no embedded libphp yet). Pinned oracle installs live in `.oracle/` (gitignored).
- Acceptance project: /home/forge/projects/pinkary.com (read-only — never modify it; copy into corpus/ if you need a writable checkout).
- Use `pcre2` crate where upstream uses preg_*; PHP string semantics (byte strlen, ASCII strtolower) live in pun-tokens `php_compat`.
