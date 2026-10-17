# Component: lint (`pun lint`)

Status: **under review with Pushpak** (opened 2026-10-17). This file is the plan for the linter: what it is, how it
works today, what is left, and the questions we still have to settle. Answers go into "Decisions" below and into
DECISIONS.md. PLAN.md C8 points here. Same shape as format.md.

## 1. What it is

`pun lint` holds every rule that can change what code does. `pun lint` reports, and `pun lint --fix` applies.
It replaces three things:

| Upstream | What moves into lint |
| --- | --- |
| Rector (+ rector-laravel, rector-phpunit, pest-plugin-rector) | every rule except the Symfony, Doctrine and downgrade sets, which are unsupported (decided) |
| PHP-CS-Fixer / Pint | the 76 risky fixers (decided: risky rules are lint, format.md §4) |
| Mago's linter | its rules, opt-in as `mago.*` today |

The goal is the same changes as `rector process` for the rules a project uses. With the format rules on top,
`pun check --fix` writes the same bytes as `rector process && pint`.

## 2. How it works today

There are two separate paths, and neither is the final design.

1. **The token path** (`pun lint`, `pun check`). `[lint.rules]` that a token fixer backs run through the formatter's
   pipeline (format.md §2), with lint fixers instead of format fixers (crates/pun-cli/src/run.rs, `Stage::Lint`).
   - Without `--fix`, the run reports the files the rules would change, the way `pun format --test` does.
   - With `--fix`, only rules marked `fix = "apply"` are written. The rest are listed as remaining.
   - `pun check --fix` runs format and lint fixers together in PHP-CS-Fixer's priority order, so it matches Pint.
   - This covers the ported risky CS-Fixer fixers and the 17 Pun rules ported from Mago.
2. **The Mago path** (`pun lint --mago`). This runs 5 of Mago's Laravel rules (`no-request-all`,
   `prefer-casts-method` and three more) through mago-linter and prints `path:line:col: level [rule] message`.
   - It only reports; these rules have no fix.
   - It is behind the `mago-lint` cargo feature, because mago-linter is about 65k lines to compile
     (crates/pun-cli/src/lint.rs).

**What is missing for real lint:**
- No Rector rule runs at all.
- There are no per-line diagnostics on the token path (a rule reports a file, not a line).
- There is no line-level ignore comment yet (only `@pun-ignore-file`).
- Tiers, add-ons, severities and `--unsafe` are accepted by the config but don't do anything.

## 3. The config

```toml
preset = "recommended"          # the lint + analyse tier: essential | recommended | strict | pedantic
                                # accepted today, applies nothing yet

[lint.rules]
require-final-class = { fix = "apply" }   # reported, and applied by --fix
no-loose-comparison = "warn"              # reported, never applied
prefer-early-return = "error"
no-short-ternary = "off"
```

| Value | Meaning | Implemented |
| --- | --- | --- |
| `"error"` / `"warn"` / `"info"` | turn on at that severity | on/off yes; the severity itself is ignored |
| `"off"` | turn off | yes |
| `{ fix = "apply" }` | report, and apply with `--fix` even if unsafe | yes |
| `{ fix = "safe-only" }` | apply only the safe part | parsed, treated as "don't apply" |
| `{ fix = "off" }` | report, never apply | yes |
| other keys in the table | rule options (kebab-case, as in format) | yes |

A PHP-CS-Fixer or Rector name in `[lint.rules]` gets an error that names the Pun ID to use.
`pun migrate` writes the risky rules a pint.json enabled with `fix = "apply"`, because Pint ran them unattended.

**Designed but not built (RULES.md §4-§5):**
- **Four cumulative tiers** select lint and analyse rules together.
- **Add-ons** `laravel`, `pest` and `phpunit` add their ecosystem's rules.
- **Severities:** `warn` fails only with `--error-on-warnings`.
- **Fix safety:** `--fix` applies safe fixes; unsafe ones need `--fix --unsafe` (as in Biome and Ruff).

## 4. Rules: what exists

There are 542 lint rules in crates/pun-config/rules/mapping.toml. Each has its group, fix safety, default severity,
tier, add-on and upstream sources. The naming pass is done: verb prefixes (`no-`, `prefer-`, `add-`, `require-`),
109 merged rules, and a shared vocabulary (RULES.md §3).

| | Count |
| --- | ---: |
| Lint rules | 542 |
| Ported (run natively today) | 31 (risky CS-Fixer fixers plus Pun's ports of Mago rules) |
| From Rector core / rector-laravel / rector-phpunit / pest-plugin-rector | 271 / 82 / 57 / 20 (some rules have several sources) |
| From Mago (only Mago / shared with others) | 65 / 119 |
| Fix safety: safe / unsafe / mixed / report only | 252 / 219 / 3 / 68 |
| Tier: essential / recommended / strict / pedantic / opt-in only | 53 / 166 / 157 / 107 / 59 |
| Add-on: laravel / phpunit / pest | 76 / 61 / 18 |

Groups today are intent groups, as in Biome: `correctness`, `suspicious`, `style`, `complexity`, `performance`,
`security`, plus `dead-code`, `types`, `modernize`, `laravel`, `testing` and `refactor` (configurable codemods such as
Rector's `RenameMethod`). Q3 asks whether these become topic groups to match format §9.

Unsupported on purpose: the rector-symfony (116), rector-doctrine (59) and rector-downgrade-php (92) rules. `pun migrate`
lists them and stops.

## 5. Rector's CLI, flag for flag

| Rector | Pun |
| --- | --- |
| `rector process --dry-run` | `pun lint` |
| `rector process` | `pun lint --fix` (doesn't format afterwards, as Rector doesn't; `pun check --fix` does both) |
| `--config rector.php` | `--config pun.toml`; rector.php is read only by `pun migrate` |
| `--only=SomeRector` | `--only <rule>` (to build) |
| `--output-format console/json/github/gitlab/junit` | `--format`, same formats as `pun format` (format.md §10) |
| `--no-diffs`, `--clear-cache`, `--no-progress-bar` | the same flags `pun format` already has |
| `rector list-rules` | `pun explain` / `pun rules` (C3, to build) |

## 6. The big design point: how lint edits code

Rector parses with nikic/php-parser, changes the syntax tree, and prints the changed nodes back with php-parser's
format-preserving printer. Matching Rector's bytes means porting that printer and its quirks (PLAN.md C8 called it "the
hard core of the component").

The alternative is what Ruff, Biome, oxlint and Mago do:
- each rule finds a node in Mago's syntax tree (already a Pun dependency) and returns text edits on byte ranges;
- the engine applies the non-overlapping edits and loops until no rule has more;
- `pun format` (or `pun check --fix`) then fixes the layout of what was inserted.

This needs no printer. The result is the same code change as Rector's, but the whitespace inside an inserted node can
differ until format runs. Parity is then checked as `format(pun lint --fix)` equals `format(rector process)`, using
Rector's own fixtures and the corpus.

The same Mago syntax tree is what `pun analyse` uses, so `pun check` can parse each file once for lint and analyse.
Type-aware rules (`add-return-type` and others) ask pun-analyse for types, as Rector asks PHPStan.

## 7. Updated plan

Done: the rule catalogue (542 rules: names, groups, tiers, safety, upstream mapping), `[lint.rules]` with `fix`, the
31 token-path rules, `pun check --fix` equal to Pint, and migrate from pint.json for the risky rules.

Left, in order (pending the answers below):

1. **Lint engine** on Mago's syntax tree with range edits (§6, Q1). It must report per line, with rustc-style frames
   (format.md §10), and apply fixes in a loop until stable.
2. **Diagnostics for the token path** too, so CS-Fixer's risky rules also report a line, not just a file.
3. **Config that does something:** tiers, add-ons (detected from composer.json, Q5), severities and `--unsafe`.
4. **Ignore comments** `@pun-ignore` on a line, the next line or a region (RULES.md §7), and `no-unused-ignore`.
5. **Rules in usage order:** Pinkary's rector.php first, then the rules the `recommended` tier plus the `laravel`
   add-on turn on, then rector-laravel, Pest, PHPUnit, and the PHP 8.4+ sets. Each must pass Rector's own fixtures.
6. **Mago's rules join normal lint rules** under their Pun names, and the `--mago` flag and feature go away (Q6).
7. **`pun migrate` for rector.php:** paths, skip, rules, sets, prepared sets and PHP sets (C9).
8. **Speed and memory** from the start: one parse per file shared with analyse, files in parallel, and a cache, as in
   format.

## 8. Questions

1. **How lint edits code (§6).** Range edits on Mago's tree with parity checked after formatting, or a port of
   php-parser's printer so the output matches Rector byte for byte?
   *Recommend: range edits. A printer port is months of work, and nobody runs Rector without a formatter after it.*
2. **What counts as done for 1.0.** Should every Rector rule in the tiers be native, or only the ones the `recommended`
   tier and the `laravel` add-on turn on, plus what Pinkary uses?
   *Recommend: recommended + laravel + Pinkary first; the strict and pedantic rules follow after 1.0.*
3. **Lint groups.** Format §9 says the same group names in all three commands. Lint groups are intent groups today
   (`suspicious`, `style`, as in Biome). Should lint names become topic groups like format (`lint/imports/no-unused`,
   `lint/laravel/no-request-all`), with intent kept as data? Or should lint keep intent groups (`lint/suspicious/no-eval`),
   as Biome does?
   *Recommend: topic groups, so `imports` or `laravel` means the same thing everywhere. The tiers already sort rules by
   how strict they are.*
4. **Tiers.** Keep four tiers (`essential` < `recommended` < `strict` < `pedantic`) selecting lint and analyse
   together, with `recommended` as the default?
   *Recommend: yes, as designed (RULES.md §5).*
5. **Add-ons.** Turn `laravel`, `pest` and `phpunit` on automatically from composer.json, with a way to turn them off?
   *Recommend: yes. Pun is Laravel-first and the user shouldn't have to say so.*
6. **Mago rules.** Fold Mago's rules into the normal lint rules (they already have Pun names) and drop `--mago`?
   *Recommend: yes. One way to turn a rule on.*
7. **Unsafe fixes.** Should `--fix` apply only safe fixes, with unsafe ones needing `--unsafe` or `fix = "apply"`
   per rule?
   *Recommend: yes (Biome and Ruff). `pun migrate` keeps today's behaviour by writing `fix = "apply"` for what Rector
   and Pint already applied.*
8. **Severities.** Should `warn` be printed but fail the run only with `--error-on-warnings`?
   *Recommend: yes (Biome).*
9. **Unported Rector rules.** If a migrated rector.php uses a rule Pun hasn't ported, should Pun run real Rector for
   it through system PHP (like format's delegate mode), or skip it with a warning? This ties to format Q7.
   *Recommend: skip with a warning and list it in `pun migrate`'s report. Delegating to Rector would mean a second
   parse and a second fix loop with different rules.*
10. **`pun lint --fix` and formatting.** Keep the rule that `pun lint --fix` doesn't format afterwards, so
    `pun check --fix` is the one command that does both?
    *Recommend: keep it.*

## Decisions

Answered 2026-10-17 (Pushpak):

- **Q1: text edits.** Rules return text edits on byte ranges found through Mago's syntax tree, and format tidies the
  layout afterwards. There is no php-parser printer port. Parity is `format(pun lint --fix)` equal to
  `format(rector process)`, on Rector's fixtures and the corpus.
- **Q3: topic groups**, the same group list as format and analyse (`lint/imports/no-unused`,
  `lint/laravel/no-request-all`); intent (correctness, suspicious, style) stays as data. Each upgrade target gets its
  own group (`lint/laravel-13/...`, `lint/php-8.5/...`), so one line turns a whole upgrade on.
- **Q10: `pun lint --fix` doesn't format**; `pun check --fix` does lint, then format (commands.md §4).
- **Q9: unported rules are named, not delegated.** `pun migrate` and every run say which configured rules Pun doesn't
  support yet (commands.md decisions).
- **Upgrades are lint rules in their own groups**, not a separate command (commands.md decisions).
- **Laravel is built in**, so the `laravel` add-on is not something to install (Q5 still asks how it turns on).
