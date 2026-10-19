# Plan: foundations, harness, docs and release

Status: housekeeping parts that every command depends on (old components C0, C10-C14). Last checked against main at
7a76be5 (2026-10-17).

## 1. CI, gates and how threads work (C0)

**Today.** Cargo workspace (10 crates, `punphp-<x>` packages, Mago pinned at `=1.57.0`). CI runs tests, rustfmt,
coverage (100% per crate via `scripts/coverage.sh`), parity and fuzz; cargo-dist release on tags. The oracle is pinned
in `.oracle/` (reference/oracle.md); 7 corpus projects (`corpus/manifest.toml`).

**Build.**
1. Get CI on main green: coverage is red (pun-analyse at 97.59%), and runs get cancelled by the next push.
2. Clippy gate with `-D warnings`: clippy already fails the job on errors and the cleanup pass took warnings to 0,
   so add `-- -D warnings` (ci.yml "ponytail" comment).
3. Update AGENTS.md when the new design lands: rule names, no delegate mode, and "output must match upstream" narrowed
   to exit codes and machine formats.

## 2. Parity harness, corpus and fuzz (C11)

**Today.** `pun-harness format` on 7 projects writes reports/scoreboard.md; `scripts/fuzz-parity.sh`;
`scripts/cli-parity.sh`; exporters for upstream fixtures. Analyse uses `scripts/analyse-corpus.py`.

**Build.**
1. After config.md Build 1, the harness uses the new names (it runs `pun migrate` + `pun check --fix`).
2. Lint mode: `format(pun lint --fix)` against `format(rector process)` per project and per rule.
3. Analyse mode: errors (file, line, identifier) against PHPStan + Larastan per project and preset.
4. `scripts/cli-parity.sh` covers only exit codes and machine formats once Pint's screen goes.
5. Make pun-harness dev-only (`publish = false`, out of dist) and move its fixtures into pun-fixers. Benchmark rig in CI.

## 3. Docs site and brand (C12)

**Today.** Blume site (`docs/`), Sticker Pop design system (`design/`). **Build:** update the pages as each command
lands (`.claude/skills/blume-update-docs`); the generated "Coming from Pint / PHPStan / Rector" page from mapping.toml;
one page per rule from mapping.toml; output samples in the docs use the brand colours.

## 4. Release (C13)

**Today.** cargo-dist config (shell, npm `@punphp/pun`, Homebrew tap, 6 targets), Composer shim, release workflow.
**Build:** after ../PLAN.md §3 Step 4: name checks on crates.io, Packagist, npm and Homebrew, signing, publishing 1.0.

## 5. Later (C10, C14)

- **LSP:** the last step, after 1.0's commands. Nothing is in the repo for it.
- **`pun install`, `pun run`, `pun test`:** the archived plan's phases 5-7. In 1.0 or after is open (../PLAN.md §4).
- **Native Blade formatter** and a Pun reprinting preset: after 1.0 (format.md).
- **An extension system** (Mockery, Livewire, Filament, a project's own rules): after 1.0 (analyse.md §9).
