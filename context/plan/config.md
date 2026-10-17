# Plan: config, rule names and presets (pun.toml)

Status: **decided, not built** (2026-10-17). This file gathers the config decisions from format.md, lint.md,
analyse.md, init.md and commands.md into one schema, and lists the work to build it. It is Step 1 of
../PLAN.md §3, and every other plan depends on it. Decisions with dates: ../PLAN.md §2. Rule placement,
naming style and ignore comments: rules.md; the data: crates/pun-config/rules/mapping.toml.

## 1. What it is

pun.toml is the only config Pun reads. Every command stops without it: "No pun.toml found. Run `pun migrate`, or
`pun init` for a new project." `pun migrate` writes it from old configs (plan/migrate.md) and `pun init` writes it for
a new project (plan/init.md). `pun config` prints the fully resolved config, every rule included.

## 2. Today (pun-config, main at 7a76be5)

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
exclude = ["tests/**"]                # added to [files] exclude; not analysed and not scanned for symbols
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
  are one per target (`laravel-13`, `php-8.5`). The format table is format.md §8; lint and analyse follow the same
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
4. **TOML-native option names** for every format rule (format.md Decisions), with migrate translating PHP-CS-Fixer
   options.
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
9. **The shared problem type** (Pushpak, 2026-10-17): rule ID, path, span (byte range, shown as line:col), message,
   help, level, and an optional fix (safety, text edits). One small crate that lint, analyse and the renderer
   (output.md Build 1) all use.

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

All from 2026-10-17 (Pushpak), recorded in ../PLAN.md §2: pun.toml only and stop without it; `group/name` with no
aliases; three format presets; lint sets; oxlint levels; `--fix` applies all, `fix = "off"`; five analyse presets
with `max` on top; analyse on or off; ecosystems from composer.lock; PHP version from composer.json; no PHP
fallback; `pun list-files` like Mago's. The `[files]` section with per-command `exclude` (Mago's model) and the
`[analyse] preset` key (instead of a top-level `preset`) are the assistant's defaults for the schema above.

Build 9, the shared problem type (crates/pun-problem, assistant's calls, 2026-10-17):

- **Columns count bytes**, 1-based, as PHP's `strlen` and Mago's spans do: `é` advances the column by 2. Lines are
  1-based and end at `\n`, `\r\n` or a lone `\r`, as in PHP's scanner. An offset past the end is clamped to the end.
- **Fixes apply in the order given**, so the caller's order is the priority. A fix applies whole or not at all. It
  is deferred to the next round when one of its edits overlaps an edit already applied, or is an insertion at or
  inside one (the order of the inserted text would be ambiguous); adjacent replacements both apply. A fix whose
  edits are out of range, split a UTF-8 character or overlap each other is reported as invalid and never applies.
- **Levels serialize as `error` / `warning`**, safety as `safe` / `unsafe`; the rule ID is the full
  `<command>/<group>/<name>` string and the path is relative to the project root.

Build 5 (wf/composer, 2026-10-17), assistant's calls where the plan is silent:
- `pun_config::composer::detect(root)` returns the `require.php` constraint with its lowest admitted version
  (`php_version()` gives it as `major.minor` for `[php] version`) and the locked Laravel (`laravel/framework`), Pest
  (`pestphp/pest`) and PHPUnit (`phpunit/phpunit`) versions, each marked `dev` when it comes from `packages-dev`.
- Only composer.lock turns an ecosystem on: without a lock none is detected (no fallback to composer.json's
  `require`). `laravel/laravel` and `illuminate/*` are not the framework.
- A locked branch (`dev-main`, `3.x-dev`) keeps its raw version with no numbers; a pre-release (`5.0.0-RC1`) counts as
  its release. A constraint that admits nothing (`>=8.4 <8.3`) gives no PHP version. `>8.4` gives 8.4.1.
- The PHP 8.4 gate in `load` now uses the same reader (`constraint::lowest`); `allows_below` is gone. Wiring the
  detected version into `[php] version` and the ecosystems into lint/analyse is left to the schema step (Build 3).

Build 6 (remove delegate mode), assistant's calls (2026-10-17):
- The unported-rule check runs in pun-cli, against the real fixer registry (`pun_fixers::all`), not mapping.toml's
  `ported` flag, which is stale (`strict_param` says `ported = false`). It covers `[format.rules]`, the preset and
  `[lint.rules]` on every `format` / `lint` / `check` run, so a lint rule stops `pun format` too. Message:
  `<pun.toml path>: rule(s) not supported by Pun yet: `<Pun ID>`, ...` (the upstream name when the rule has no Pun ID,
  such as `@PER-CS`; `preset <name>` instead of the path when no pun.toml was read). Exit 2; other config errors keep
  exit 1 until the schema step (Build 3) decides their code.
- `pun migrate` still reads a `.php-cs-fixer.php` through the project's own PHP-CS-Fixer (one `php -r` run,
  crates/pun-cli/src/php_cs_fixer_rules.php), because the config is PHP code. This reads the old config only; no rule
  ever runs in PHP. Replacing it with a static read is a follow-up for migrate Build 1.

Build 1-2 (wf/names, 2026-10-17), assistant's calls: the added lint and analyse groups, how lint `sets` and analyse
`preset` were derived, the lint names of the 40 moved rules and the reader's error for old names are recorded in
rules.md Decisions. The top-level `preset` tiers stay accepted (and still apply nothing) until the schema step (Build 3)
removes them; mapping.toml no longer carries `tier`. Moved rules keep running in the same priority-ordered token pass,
so the 11 laravel-preset ones are written by `pun migrate` under `[lint.rules]` with `fix = "apply"`; all 7 corpus
projects stay at 0 differing files.

Build 3 (wf/schema, 2026-10-17), assistant's calls where the plan is silent. The schema is read by
`pun_config::load` (crates/pun-config/src/schema.rs) into `Config.files`, `.lint` and `.analyse`:

- **Values.** A lint rule is `"error"`, `"warn"` or `"off"`, or a table with `level` (default `error`), `fix = "off"`
  and the rule's options; `true` / `false` are an error naming `"error"` / `"off"`. A lint group takes a level only.
  An analyse rule is `"on"`, `"off"` or a table of options (on); every group, not only the ecosystem ones, takes
  `"on"` / `"off"` in `[analyse.rules]`. Options of a rule a fixer runs go to it with snake_case top-level keys; other
  rules keep them as written.
- **Precedence and gates.** Set (at `error`) < group < the rule's own line. Only what a set or preset turns on is gated:
  an ecosystem group's rules need the ecosystem on (locked in composer.lock and not switched off with its group), a
  `php-X.Y` rule needs `[php] version` >= X.Y, a `laravel-N` rule needs Laravel on and a locked version >= N (a locked
  branch has no version, so none). A group's level or a rule's line turns rules on whatever the gates. `gated_by` on a
  rule outside an upgrade group carries no version and gates nothing. `doctrine` is never detected.
- **Rules Pun can't run.** A rule a set or group turns on that Pun can't run yet is skipped; one named in `[lint.rules]`
  stops the run with config 6's message (exit 2), also when no fixer exists for it (before: a config error, exit 1).
- **Files.** `[format] paths` is an error naming `[files] paths`. `[files] exclude` plus `[format] exclude` feed Pint's
  finder as it is (config 7 owns the glob semantics). `[lint] exclude` is accepted (each command's own exclude, §3);
  `[analyse] paths` replaces `[files] paths` for analyse.
- **Removed keys** are errors naming the replacement: the top-level `preset`, `[format] preset = "per" | "symfony"`,
  `fix = "apply" | "safe-only"`, `"info"`, `severity`, and `[analyse] level` or a number as `[analyse] preset` (naming
  the preset that holds that PHPStan level). Messages are pinned in crates/pun-config/src/tests.rs.
- **`[php] version`** must be `major.minor`; without it the lowest version composer.json admits, else 8.4 (the config 3
  follow-up row). composer.lock is read on every load.
- **Migrate** (pun-config's writer, kept minimal for migrate 1): risky and code-deleting fixers are written at
  `"error"` (a table of options without `level` when they have options), Pint's `in` becomes `[files] paths`, a `per`
  or `symfony` preset becomes `empty` plus its rules (their sets are listed as unsupported), and the starter has no
  top-level `preset`.
- **Not at run time yet** (lint 1-2): `[[lint.overrides]]` are resolved but pun-cli's token pass ignores them, and a
  `warn` rule's fix fails `pun lint` / `pun check` like an `error`. `--preset per` on the command line still works
  (Pint's flag; format 1). The no-config message stays "Run `pun migrate`." until `pun init` exists.


Build 7 (wf/files, 2026-10-17), assistant's calls where the plan is silent:

- **Glob semantics are Mago's** (`mago_database::matcher::ExclusionMatcher`, default glob settings): an `exclude` entry
  with `*`, `?`, `[` or `{` is a glob matched against the root-relative path (`*` crosses `/`); any other entry is a
  root-relative path prefix. For `format`, plain entries in `[files] exclude` and `[format] exclude` keep Pint's
  Symfony `exclude()` meaning (a folder name at any depth, or path segments under each `in` dir), so migrated
  pint.json excludes and the Pint corpus are unchanged; only glob entries take Mago's meaning there (also for
  `--stdin-filename`). A pint.json exclude with a glob character now means a glob, not a literal folder name.
- **The `[files]` set** (no `--command`, `lint`, `analyse`) walks like PHP-CS-Fixer's `Finder::create()`: `*.php`
  (`.blade.php` included), no `vendor` folder at any depth, no dot files or VCS folders, symlinked folders not
  followed. Pint's own skips (`storage`, `build`, `node_modules`, `bootstrap/cache`, IDE helpers, Blade without the
  rule, not-name, not-path) are format's only.
- **Path arguments** replace `[files] paths` (and `[analyse] paths`). Outside format a pun.toml exclude also drops an
  explicitly named file (Mago); format keeps Pint's rules (an explicit file bypasses every filter, and with `in` set
  the arguments replace the whole finder). A missing `[files] paths` entry fails with Pint's
  `The "x" directory does not exist.`, a missing argument with `The path "x" is not readable.`; a `[files] paths` entry
  may name a file.
- **Output.** `-0` and `--format=json` print no agent summary; the agent summary is
  `pun list-files: N files[, <command> set] (Ts)` followed by the full, uncapped list. `--format` takes only `json`;
  `--config` is accepted like on every other command.
- **Not wired yet:** `pun lint` and `pun check` still run over format's (Pint's) file set in one token pass, so
  `[lint] exclude` only shows in `pun list-files --command lint`. Wiring it belongs with the lint engine (lint 1-2).

Build 8 (wf/config-explain, 2026-10-17), assistant's calls where the plan is silent:

- **`pun config` prints pun.toml's own sections and keys**: `[php] version`, `[files]`, `[format]` (preset, exclude,
  not-name, not-path, rules), `[lint]` (sets, exclude, rules, `[[lint.overrides]]`) and `[analyse]` (preset, paths,
  exclude, baseline, rules). Rules are keyed `<group>/<name>` under their command's table, as `pun migrate` writes them
  (a rule with no Pun ID, such as `@PER-CS`, keeps its upstream name); groups and sets are expanded into the rules they
  turn on, so `[lint.groups]` is not printed. Each command's `exclude` is the full list it applies (`[files] exclude`
  plus its own). Two resolved-only keys stay: `ecosystems` (lint and analyse) and `[analyse] switches`. Pint's
  `finder` block and `sources` are gone; the TOML starts with `# Resolved from <files>.` instead (no comment when
  nothing was read), and agent mode keeps its summary line.
- **`pun explain <rule>`** takes a full ID or `<group>/<name>` (no two commands share one today), then an upstream key
  as written in any `[upstream.<tool>]`, then the loose match pun.toml's errors use (other spellings, old `format.` /
  `lint.` names). Anything but an ID starts with "`<name>` is `<id>` in Pun." and exits 0. An upstream key with no rule
  is an error with its `drop` / `unsupported` reason, or naming its non-rule target (PHPStan parameters such as
  `checkUnionTypes`); an unknown name is an error. Errors exit 1.
- **Its text**: the ID, the summary line when mapping.toml has one, then rows `group` (with the `[meta.groups]` line),
  `presets` / `sets` / `preset` by command (`none (opt-in by name or group)` when empty), `intent` when set, `fix`
  (`none` without one), `fixed by` for analyse rules that have it, `upstream` (every `[upstream.*]` row mapping to the
  ID, as `name (tool)` in file order; `none (Pun's own rule)`), and `docs`.
- **Docs link**: the docs site has no rule pages and no decided base URL (the output follow-up row in ../PLAN.md §3),
  so `docs` links the rule's row in mapping.toml on GitHub (`.../crates/pun-config/rules/mapping.toml#L<line>`,
  `pun_config::MAPPING_URL`). Switch it to the rule page once that row decides the base.

Build 4 (wf/options, 2026-10-17): the option names, the `null` spellings, the one-option shorthand and the errors are
assistant's calls recorded in format.md Decisions ("Option names"). Lint rules that are fixers keep PHP-CS-Fixer's
option names, kebab-cased, until lint's own option pass.
