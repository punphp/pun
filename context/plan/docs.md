# Plan: rule docs and migration mapping pages

Status: design settled with Pushpak (2026-10-19), see §7. Build progress is in §8. Modelled on the rule docs of Rector, PHPStan,
PHP-CS-Fixer, Mago, Larastan, Ruff, oxlint, Biome, ESLint and typescript-eslint (research notes and screenshots are in
the project files under docs-design/research/).

All pages are generated from [`crates/pun-config/rules/mapping.toml`](../../crates/pun-config/rules/mapping.toml): 1,249 Pun rules (168 format, 703 lint,
378 analyse; 545 ported today) and 3,047 upstream entries, each pointing at a Pun rule or carrying a drop or
unsupported reason. The prose and examples are new fields in mapping.toml too (§7.3).

## 1. Pages and URLs

| Page | URL | Modelled on |
| --- | --- | --- |
| Rules directory | `/docs/rules` | Ruff / oxlint / typescript-eslint flat table |
| Group page | `/docs/rules/<command>/<group>` | PHPStan `argument.*`, Mago categories |
| Rule page | `/docs/rules/<command>/<group>/<name>` (the rule ID is the path) | Biome + PHP-CS-Fixer + PHPStan identifier page |
| Preset / set page | `/docs/rules/presets/<preset>`, `/docs/rules/sets/<set>` | PHP-CS-Fixer set pages, Rector find-rule by set |
| Migration mapping | `/docs/migrate/from/<tool>` (pint, php-cs-fixer, rector, phpstan, larastan, mago) | Biome `/linter/javascript/sources` |

`pun explain`'s `docs` row and `Renderer::with_rule_docs` then link to the rule page (closes PLAN.md row "decide the
rule docs base URL").

## 2. Rules directory (`/docs/rules`)

- Totals on top: "1,249 rules: 168 format, 703 lint, 378 analyse. 792 fixable." (oxlint).
- A grid of the groups with count and one-line purpose (Mago), each linking to its group page. Ecosystem groups
  (laravel, pest, phpunit, doctrine) and upgrade groups (php-8.4, laravel-12) in their own rows.
- One searchable table: `ID | summary | enabled by | fix`. Search matches the Pun ID **and every upstream name**, so
  pasting `no_alias_functions`, `StrictArraySearchRector` or `method.notFound` finds the Pun rule.
- Filter chips, state kept in the URL so a view is linkable (Rector, typescript-eslint): command, group, preset/set,
  fix (safe / unsafe / mixed / none), ecosystem, upstream tool, "not in any preset" bucket (Rector).
- Fixed icon columns that dim when they don't apply, so rows line up (ESLint).

## 3. Rule page

In this order:

1. **Title** = full ID `lint/style/no-alias-function`, with a copy button.
2. **Badge line as sentences**: "Enabled by the `coding-style` set." / "Fix: unsafe (can change behaviour)." / "Runs
   when Laravel is installed." / "Planned: not in Pun yet." (`ported`, `gated_by`, `intent`).
3. **Replaces** (Biome's "Sources"): every upstream name, linked to its upstream docs: `no_alias_functions`
   (PHP-CS-Fixer, risky), `no-alias-function` (Mago). Merged rules list every source.
4. **Turn it on**: the `pun.toml` snippet for this rule, plus its ignore comment (`// @pun-ignore ...`).
5. **What it does / why**: the summary, then a short paragraph.
6. **Examples**: Avoid / Prefer, or a diff for fixing rules. Analyse rules show the code and Pun's exact message.
7. **Options** (format rules, 128 options today): Pun name, type, allowed values, default, upstream option name.
8. **Linked rules**: `fixed_by` both ways (analyse report fixed by a lint rule), `[meta] conflicts`.
9. **Footer**: other rules in this group, source file, "Edit this page".

## 4. Group and preset/set pages

- Group page: the group's description, counts per command, then the rules as compact cards (ID, summary, badges).
  One group page per command, since `style` means different things in format and lint.
- Preset/set page: what it is for, then every rule it turns on **with the config it applies** (PHP-CS-Fixer), then
  the rules it leaves off. Analyse presets also say which PHPStan levels they cover (basic = 0-1, ...) and list what
  each level adds, which PHPStan itself never documents per identifier.

## 5. Migration mapping (`/docs/migrate/from/<tool>`)

One page per upstream tool, linked from `pun migrate`'s page and each tool's guide. Each page has:

1. **Config translation**: the keys users already have and where they go (`pint.json` `preset` -> `[format]
   preset`, `@PER-CS` expanded, PHPStan `level: 5` -> `recommended` plus on/off lines, `ignoreErrors` ->
   `[[analyse.ignore]]`, Rector `->withPreparedSets(deadCode: true)` -> the `dead-code` set, Larastan neon booleans).
2. **Rule table** (Biome): `upstream name (linked) | Pun rule (linked) | notes`. Notes say `same`, `merged with X`,
   `options carried over / dropped`, or the drop/unsupported reason. Searchable, with a filter for "unsupported only".
3. **Unsupported** section, grouped by reason (out of scope: rector-symfony, rector-doctrine, downgrade sets, PHPStan
   `phpstanApi.*`), each ending with "send a PR" per the no-PHP-fallback decision.
4. **Pun-only rules** (`pun_only`), like Biome's "exclusive rules".

PHPStan is mapped at identifier level (what users write in `ignoreErrors` and `@phpstan-ignore`), with pattern
identifiers (`%s.alwaysFalse`) expanded.

## 6. Content gaps (what mapping.toml doesn't hold yet)

- Analyse rules have no `summary` (0 of 378). Format and lint have all of theirs.
- No rule has examples or a "why" paragraph. Sources: format from the upstream fixtures already in
  `crates/pun-fixers/tests`, lint from Rector/Mago samples (rewritten in Pun's words), analyse from oracle runs.
- Options carry types but not per-option prose.

## 7. Decisions

1. ~~Do planned rules get pages?~~ **Decided 2026-10-19: yes, every rule gets a page; unported ones are marked Planned.**
2. ~~How pages are built?~~ **Decided 2026-10-19: generated MDX.** A script writes one `.mdx` per rule from
   mapping.toml into `docs/content/docs/rules/`; a check fails when they are out of sync.
3. ~~Where examples and prose live?~~ **Decided 2026-10-19: in mapping.toml**, as fields on each rule (why, examples).
4. ~~Mapping pages layout?~~ **Decided 2026-10-19: one page per tool** under `/docs/migrate/from/`, plus a short overview page.
5. ~~Filters at launch?~~ **Decided 2026-10-19: yes**, one table of every rule with search across upstream names and URL-kept filter chips.

## 8. Build

| Step | Status | Verification |
| --- | --- | --- |
| 1. Generator and folder metadata | built | `python3 scripts/gen-rule-docs.py --check`; checked beside the mapping in CI |
| 2. Rule page and content schema | built | `python3 scripts/test-rule-docs.py`: ordered sections, content fields, defaults, reverse links and Planned state |
| 3. Groups, presets and sets | todo | Exact configs, off rules and PHPStan level additions |
| 4. Migration mapping | todo | All six tools, concrete PHPStan identifiers |
| 5. Rules directory | todo | Search and URL-kept filters |
| 6. CLI links | todo | Explain and terminal hyperlinks target rule pages |

The content gaps in §6 remain separate work in PLAN.md §3, not placeholder prose to invent during generation.
