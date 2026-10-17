# Decisions

Every decision the user has made, in one place. Newest first within each group. When a decision changes, edit the
line and note the date; don't add a contradicting line elsewhere. Open questions live in PLAN.md "Open questions".

Gathered on 2026-10-17 from the old docs/TASKS.md ("Decisions", "User decisions"), docs/PLAN.md ("Decisions for you to
confirm", "Update, Oct 17"), docs/RULES.md §10 and project memory.

## Product

- **Home and licence:** github.com/punphp/pun, MIT (LICENSE + THIRD_PARTY_NOTICES.md). Pun is not a shipfastlabs
  project (the old plan's "under shipfastlabs" default is void). Repo stays private for now.
- **Positioning:** Pun is to PHP what Bun is to JavaScript. Laravel first, PHP 8.4+ projects only.
- **Release:** nothing is published or tagged until `pun format`, `pun analyse`, `pun lint` and `pun migrate` all exist.
  The site and docs present that release as 1.0, not a pre-release. Publishing needs the user's go.
- **Brand:** "Sticker Pop" direction, sprinting elephant mascot in sneakers; Fredoka for all non-code text, Martian Mono
  for code (design/README.md).

## Commands and config

- **Command layout:** `pun format` (layout, Pint parity), `pun lint [--fix]` (code rules, Rector parity; Mago rules opt-in
  as `mago.*`), `pun analyse` / `analyze` (type errors, PHPStan + Larastan parity, never edits), `pun check [--fix]`
  (all of them in one pass), `pun migrate` (one-time move onto pun.toml), `pun lsp` (PHPantom + Pun).
  `pun lint --fix` does not format afterwards (Rector doesn't).
- **pun.toml only.** Commands never read pint.json, .php-cs-fixer.php, phpstan.neon or rector.php. Without pun.toml:
  "No pun.toml found. Run `pun migrate`." The old-config readers live only inside `pun migrate` (and the harness). The
  `pint` argv[0] alias is gone; `pun format` output stays Pint-identical.
- **`pun migrate` means config migration**, not code upgrades. It must carry pint.json, .php-cs-fixer.php, rector.php and
  phpstan.neon over completely, and name anything Pun doesn't support yet instead of dropping it (2026-10-17).
- **One job per command** (components/commands.md): format = how it looks, lint = a better way to write it, analyse =
  is it wrong. `pun check` runs lint with fixes, then format, then analyse (2026-10-17).
- **No `pun upgrade` command.** Version upgrades (PHP, Laravel, PHPUnit, Pest) and codemods are lint rules in their own
  groups (2026-10-17). This replaces the old `pun lint --set php85`.
  PHPDoc type tags (`@phpstan-param` ...) are kept by default; `pun migrate --phpdoc` rewrites them (opt-in, apps only).
- **Pun's own rule names** are `group/name` in kebab-case, full ID `<command>/<group>/<name>` (`format/imports/ordered`),
  with one shared group list across format, lint and analyse (2026-10-17, components/format.md §9). No aliases: renames
  are breaking and that's fine until the release. Lint uses topic groups too, not intent groups; each upgrade target is its
  own lint group (`lint/laravel-13/...`). The upstream mapping is kept as data in
  crates/pun-config/rules/mapping.toml (used by migrate, `pun explain` and the docs).
- **No levels** (confirmed 2026-10-17: named presets for new projects; migrate writes explicit rules and, when
  interactive, offers the closest preset with a count of the new problems it would report). Presets are tiers: `essential` < `recommended` (default) < `strict` < `pedantic`, cumulative, one tier
  for lint + analyse; add-ons `laravel` / `pest` / `phpunit`.
- **`pun init`** asks the format preset (laravel / psr12), the analyse preset and the lint preset; lint and analyse
  presets are chosen separately (2026-10-17, components/lint.md §9). Lint's preset is a checklist of sets
  (`[lint] sets = ["dead-code", "modernize"]`), like Rector's prepared sets; analyse keeps the four-step preset. Laravel, Pest and PHPUnit rules turn on
  automatically from composer.lock.
- **Laravel support is built in** (2026-10-17): nothing to install, unlike Larastan or rector-laravel. Every PHPStan
  core rule is supported natively.
- **Agent output:** under a coding agent every command prints compact `path:line:col rule message` lines, not JSON.

## Format (components/format.md, 2026-10-17)

- Risky rules stay in lint; `pun check --fix` equals `pint`.
- Fixers that delete or restructure code (unused imports, useless else, combine issets and 37 more) move to lint
  with safe fixes; format only changes layout, casing and spelling (components/commands.md §5, 2026-10-17).
- Format presets: `laravel`, `psr12`, `empty` only.
- Rule options are designed for TOML, not copied from PHP-CS-Fixer; migrate translates upstream options.
- No pun.toml: stop with "Run `pun migrate`, or `pun init` for a new project" (`pun init` to be built).
- Blade stays on Node + Prettier until a native formatter exists.
- Speed and memory are a priority now.
- Big runs format files in parallel (one worker per core); kept, and measured by the 8-core benchmark.
- Pun's own human output: modern and coloured like oxfmt / Biome / Ruff (approved, components/format.md §10); machine
  formats stay Pint-compatible.
- A Pun reprinting style (`preset = "pun"`) comes later, after 1.0.

## Rules design (RULES.md, all accepted)

1. Risky fixers move from format to lint; `pun migrate` turns them on with auto-fix; Pint parity is proven with
   `pun check --fix`.
2. Short rule names are accepted in config and ignore comments; output always prints the full ID.
3. PHPStan core and Larastan identifiers stay in analyse until v1.
4. Laravel first: Rector's Symfony, Doctrine and downgrade sets are unsupported; migrate lists them and stops.
5. Naming pass on the tentative lint/analyse names: done (542 lint, 255 analyse rules).
6. Rule placement: each concept gets exactly one Pun rule in exactly one command.

## Engineering

- **Mago:** depend on crates.io releases (pinned `=1.57.0`); no maintainer outreach needed.
- **`pun lint --fix` applies every fix of every enabled rule** (2026-10-17), as Rector does: no `--unsafe` flag, no
  `fix = "apply"`. Safe/unsafe stays a label in the output; `fix = "off"` makes a rule report-only. This replaces
  RULES.md §4's fix-safety gating.
- **Severity** (2026-10-17): only `error` fails a run; `warn` prints and never fails. Rules are `error` by default, and
  `pun migrate` writes everything as `error`. Syntax and flags follow oxlint: `"off" | "warn" | "error"` per rule or
  group, `--deny-warnings`, `--max-warnings`, `-A/-W/-D`, and per-path `[[lint.overrides]]`.
- **Lint edits code with text edits** on Mago's syntax tree, not a port of php-parser's printer; format tidies the
  layout afterwards (2026-10-17, components/lint.md).
- **The Mago-based formatter is removed** (`--engine=pun`, `format_pun`): one formatter engine, the PHP-CS-Fixer port.
- **Own lexer** in pun-tokens instead of mapping mago-syntax tokens (reference/tokens.md).
- **Built-in symbols for analyse** come from PHPStan's own reflection, exported once into
  crates/pun-phpstan/data/symbols.json, not from Mago's prelude.
- **pun lsp ships last** (2026-10-17): no LSP work now. When it comes: embed PHPantom in-process, routed at the
  JSON-RPC level (reference/lsp.md).
- **FrankenPHP:** Pun links libphp itself first (Rust SAPI); FrankenPHP is linked in later. Assistant's call, delegated.
- **Embedded PHP (pun-php):** deferred; system `php` stands in for delegate mode and the oracle.
- **Tests:** 100% line coverage per crate, no tautological tests, expected values from the real tools (AGENTS.md).
