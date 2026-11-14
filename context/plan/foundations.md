# Plan: foundations, harness, docs and release

Status: housekeeping parts that every command depends on (old components C0, C10-C14). Last checked against main at
54df451 (2026-11-08).

## 1. CI, gates and how threads work (C0)

**Today.** Cargo workspace (11 crates, `punphp-<x>` packages, all `publish = false`, every one on the shared
`[workspace.lints]`; Mago pinned at `=1.57.0`). CI runs tests, rustfmt, clippy with `-D warnings`, coverage (100% per
crate via `scripts/coverage.sh`), parity and fuzz; cargo-dist release on tags. CI is disabled on GitHub. The oracle is pinned
in `.oracle/` (reference/oracle.md); 7 corpus projects (`corpus/manifest.toml`).

**Build.**
1. Get CI on main green: coverage is red (pun-analyse at 97.59%), and runs get cancelled by the next push.
2. Update AGENTS.md when the new design lands: rule names, no delegate mode, and "output must match upstream" narrowed
   to exit codes and machine formats.

## 2. Parity harness, corpus and fuzz (C11)

**Today.** `pun-harness format` on 7 projects writes reports/scoreboard.md; `scripts/fuzz-parity.sh`;
`scripts/cli-parity.sh`; exporters for upstream fixtures. Analyse uses `scripts/analyse-corpus.py`.

**Build.**
1. After config.md Build 1, the harness uses the new names (it runs `pun migrate` + `pun check --fix`).
2. Lint mode: `format(pun lint --fix)` against `format(rector process)` per project and per rule.
3. Analyse mode: errors (file, line, identifier) against PHPStan + Larastan per project and preset.
4. `scripts/cli-parity.sh` covers only exit codes and machine formats once Pint's screen goes.
5. Benchmark rig in CI. (pun-harness is dev-only, out of dist, and its fixtures live in pun-fixers/tests/fixtures.)

## 3. Docs site and brand (C12)

**Today.** Blume site (`docs/`), Sticker Pop design system (`design/`). **Build:** update the pages as each command
lands (`.claude/skills/blume-update-docs`); the generated "Coming from Pint / PHPStan / Rector" page from mapping.toml;
one page per rule from mapping.toml; output samples in the docs use the brand colours.

## 4. Release (C13)

**Today.** Pun ships only as the Composer package `punphp/pun`, the way Mago ships. `composer.json` sits at the repo
root, `composer/bin/pun` is the shim, and `.gitattributes` export-ignores everything else, so a tag's zipball holds
only `composer.json`, the shim, the README and the licenses. On first run the shim downloads `punphp-<target>.tar.gz` from the tag's GitHub release, checks its
`.sha256` and caches the binary in Composer's bin directory (`vendor/bin/pun-<version>-<target>`); it supports Linux (musl) and macOS on x86_64 and arm64.
cargo-dist builds those four archives on a tag push (`dist-workspace.toml`: no dist installers, no source tarball).
The release also carries `install.sh`, a standalone installer (Pushpak, 2026-11-14): `.github/workflows/installer.yml`
stamps the top-level `install.sh` template with the version and each archive's SHA-256 (`scripts/stamp-installer.py`),
and the host job attests the archives and `install.sh` with GitHub artifact attestations, which the installer checks
through `gh` when it is signed in. No crate, npm package or Homebrew tap is published.
Betas are tagged `v0.1.0-beta.N`; cargo-dist marks them pre-releases, and Composer installs them only with
`punphp/pun:@beta`. The shim's `PUN_VERSION` follows the workspace version.
**Build:** the first beta tag and its Composer install check, Packagist submission with the GitHub hook, then signing
and publishing 1.0 after ../PLAN.md §3 Step 4.

## 5. Later (C10, C14)

- **LSP:** the last step, after 1.0's commands. Nothing is in the repo for it.
- **`pun install`, `pun run`, `pun test`:** the archived plan's phases 5-7. In 1.0 or after is open (../PLAN.md §4).
  The package manager's plan is packages.md.
- **Native Blade formatter** and a Pun reprinting preset: after 1.0 (format.md).
- **An extension system** (Mockery, Livewire, Filament, a project's own rules): after 1.0 (analyse.md §9).
