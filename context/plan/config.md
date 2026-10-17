# Plan: config, rule names and presets (pun.toml)

Status: **decided, not built** (2026-10-17). This file gathers the config decisions from format.md, lint.md,
analyse.md, init.md and commands.md into one schema, and lists the work to build it. It is Step 1 of PLAN.md §3, and
every other plan depends on it. Decisions with dates: DECISIONS.md. Rule placement data: RULES.md and
crates/pun-config/rules/mapping.toml.

## 1. What it is

pun.toml is the only config Pun reads. Every command stops without it: "No pun.toml found. Run `pun migrate`, or
`pun init` for a new project." `pun migrate` writes it from old configs (plan/migrate.md) and `pun init` writes it for
a new project (plan/init.md). `pun config` prints the fully resolved config, every rule included.

## 2. Today (pun-config, main at 0800a99)

- `[php]`, `[format]` (preset, paths, exclude, not-name, not-path, rules) and `[lint.rules]`.
- Rule IDs are `format.<name>` / `lint.<name>`, with aliases from before the naming pass.
- A top-level `preset` takes the four tiers `essential`, `recommended`, `strict`, `pedantic`; it is validated but
  applies nothing.
- Lint rules take `"error"`, `"warn"`, `"info"`, `"off"` or a table with `fix = "apply" | "safe-only" | "off"`; the
  level is treated as on or off.
- Format presets include `per` and `symfony`. Options use PHP-CS-Fixer's names in kebab-case.
- mapping.toml has `[rules.<command>]` with group, fix, severity, tier, presets, ported, aliases, and
  `[upstream.<tool>]` for every upstream rule.

## 3. The decided schema

```toml
[php]
version = "8.4"                       # override only; the default comes from composer.json's "require.php"

[files]                               # shared by every command, like Mago's [source]
paths = ["app", "tests"]              # what to check when no paths are given
exclude = ["build", "storage/**"]

[format]
preset = "laravel"                    # laravel | psr12 | empty
exclude = ["app/Generated/**"]        # added to [files] exclude, for format only
not-name = ["*.blade.php"]            # Pint's notName and notPath, under TOML names
not-path = ["app/Legacy.php"]

[format.rules]                        # only what differs from the preset
"imports/ordered" = { sort = "alphabetical" }
"arrays/syntax" = false

[lint]
sets = ["dead-code", "modernize"]     # the checklist; see §5

[lint.groups]                         # a level for a whole group
style = "warn"
laravel-13 = "error"                  # turn an upgrade on

[lint.rules]                          # a rule's own line wins over its group
"laravel/no-debug-calls" = "error"
"style/prefer-early-return" = { level = "warn", fix = "off" }
"types/no-loose-comparison" = "off"

[[lint.overrides]]
files = ["tests/**"]
rules = { "laravel/no-debug-calls" = "off" }

[analyse]
preset = "recommended"                # basic | essential | recommended | strict | max
exclude = ["tests/**"]                # added to [files] exclude, for analyse only
baseline = "pun-baseline.toml"

[analyse.rules]
"types/missing-iterable-value-type" = "off"
"undefined/method" = { magic = true } # options in a table mean "on, with these options"
```

`pun init` writes only `[format] preset`, `[lint] sets` and `[analyse] preset`. Everything else is optional.

**Files.** `[files] paths` and `exclude` apply to every command; each command's own `exclude` is added on top, so
format, lint and analyse can see different file sets (Mago's model). `vendor/` is never checked, but analyse reads it
for symbols. `pun list-files` shows the result (§7).

## 4. Rule names and groups

- **Shape:** `<command>/<group>/<name>` (`format/imports/ordered`, `lint/dead-code/no-unused-private-method`,
  `analyse/undefined/method`). Inside a `[<command>.rules]` table the command part is implied.
- **One group list for all three commands**, fixed in mapping.toml with a one-line description each. Topic groups
  (`imports`, `arrays`, `phpdoc`, `types`, ...) and ecosystem groups (`laravel`, `pest`, `phpunit`). Upgrade groups
  are one per target (`laravel-13`, `php-8.5`). The format table is format.md §9; lint and analyse follow the same
  rules.
- **No aliases.** Old Pun names, PHP-CS-Fixer names and PHPStan identifiers in a `rules` table are an error that names
  the Pun ID to use. Only `pun migrate` reads upstream names.
- Intent (correctness, suspicious, style) and fix safety stay as data in mapping.toml, shown in output and docs.

## 5. Presets, sets and levels

- **Format presets:** `laravel`, `psr12`, `empty`. `per` and `symfony` are removed; their rules stay available one by
  one, and migrate expands a pint.json that used them into explicit rules.
- **Lint sets:** `dead-code`, `modernize`, `code-quality`, `types`, `early-return`, `privatization`, `naming`,
  `coding-style` (Rector's prepared sets). Sets are tags on rules, and a rule can be in several. The Laravel, Pest and
  PHPUnit part of each set turns on by itself from composer.lock. Rules in no set are opt-in by name or group.
- **Lint levels, as oxlint:** `"off"`, `"warn"`, `"error"` per rule or group. A rule enabled by a set is `error` unless
  set to `warn`. Only `error` fails. `--deny-warnings` makes warnings fail, `--max-warnings <n>` fails above n.
  `-A` / `-W` / `-D <rule or group>` set a level for one run. `[[lint.overrides]]` changes levels per path.
- **Lint fixes:** `--fix` applies every fix of every enabled rule. `fix = "off"` keeps a rule report-only.
  `fix = "apply"` and `"safe-only"` are removed, and so is the top-level `preset`.
- **Analyse presets** by PHPStan level band, each including the one below: `basic` (0-1), `essential` (2-3),
  `recommended` (4-5, default), `strict` (6-8), `max` (9-10). The four type switches (`union-types`, `nullables`,
  `explicit-mixed`, `implicit-mixed`) come with the presets. Analyse rules are on or off; there is no `warn`.
- **Ecosystems:** Laravel, Pest and PHPUnit turn on from composer.lock, with their versions gating version rules. A
  project switches one off with its group (`[lint.groups] laravel = "off"`, `[analyse.rules]` the same way).

## 6. Build

Each step ends with the existing gates green (`cargo test -p punphp-config`, the corpus harness at 0 differing files).

1. **New names in mapping.toml.** Give every rule its `<command>/<group>/<name>` ID and group from the fixed list,
   drop `aliases`, add the group list and the lint `sets` tags, replace `tier` with the five analyse presets (analyse)
   and set tags (lint). Update `scripts/check-rule-mapping.py` to reject unknown groups and duplicate names.
   *Done when* the script passes and every one of the 1,005 rules has exactly one new ID.
2. **Move the 40 code-deleting format rules to lint** (commands.md §5): their `[rules.format]` rows become
   `[rules.lint]` rows. The laravel preset's behaviour is unchanged because migrate writes them as lint rules.
   *Done when* `pun migrate` + `pun check --fix` still equals Pint on all 7 corpus projects.
3. **The new pun.toml schema in pun-config:** `[format] preset` limited to three, `[lint] sets`, `[lint.groups]`,
   `[lint.rules]` with `level` and `fix = "off"`, `[[lint.overrides]]`, `[analyse]` with preset, paths, baseline and
   rules. Errors for removed keys and upstream names that name the replacement.
   *Done when* each key has a test with an expected resolved config derived by hand, and each error has its message
   pinned.
4. **TOML-native option names** for every format rule (format.md Q3), with migrate translating PHP-CS-Fixer options.
   *Done when* every option in mapping.toml has a Pun name and type, and the Pint corpus still matches.
5. **composer.json and composer.lock detection:** the PHP version from `require.php` (lowest allowed version), and
   Laravel, Pest and PHPUnit with their locked versions.
   *Done when* tests cover a constraint list (`^8.2|^8.3`, `>=8.4`, `8.4.*`), a missing lock, and each ecosystem.
6. **Remove delegate mode:** delete pun-delegate, `--native-only` and the delegated part of `--explain`, and update
   AGENTS.md and reference/oracle.md. An unported rule in pun.toml is a config error naming the rule.
   *Done when* no crate depends on pun-delegate and the coverage gate passes.
7. **`[files]` and per-command `exclude`** in pun-workspace's finder, and **`pun list-files`** (§7).
   *Done when* the §7 example gives exactly the listed sets, and Pint's finder rules still hold for format.
8. **`pun config`** prints the new resolved schema, and **`pun explain <rule>`** prints a rule's summary, group, sets,
   fix safety, upstream names and docs link from mapping.toml.

## 7. `pun list-files` (Pushpak, 2026-10-17, after Mago's `list-files`)

Prints the files Pun would check with the current pun.toml, one per line, so you can see that `paths` and `exclude`
do what you think. It replaces the hidden `pun format --list-files` debugging flag.

```
pun list-files [--command format|lint|analyse] [-0] [paths...]
```

- Without `--command`: the `[files]` set (paths minus the shared excludes). With `--command analyse`: that command's
  set, with its own `exclude` applied. `--command format` also applies Pint's finder rules (not-name, not-path,
  `.blade.php` only with the Blade rule on).
- `-0` / `--zero-terminate` ends each name with a NUL byte for `xargs -0`.
- Paths are relative to the project root and sorted. Files analyse only reads for symbols (`vendor/`) are not listed.
- Agent mode prints the same list with a one-line summary first; `--format=json` prints an array.
- Example: with `[files] paths = ["app", "tests"]`, `exclude = ["storage/**"]` and `[analyse] exclude = ["tests/**"]`,
  `pun list-files` lists app and tests, and `pun list-files --command analyse` lists only app.

## Decisions

All from 2026-10-17 (Pushpak), recorded in DECISIONS.md: pun.toml only and stop without it; `group/name` with no
aliases; three format presets; lint sets; oxlint levels; `--fix` applies all, `fix = "off"`; five analyse presets
with `max` on top; analyse on or off; ecosystems from composer.lock; PHP version from composer.json; no PHP fallback; `pun list-files` like Mago's.
The `[files]` section with per-command `exclude` (Mago's model) and the `[analyse] preset` key (instead of a top-level `preset`) is the assistant's default for the schema above.
