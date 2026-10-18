# Plan: lint (`pun lint`)

Status: **walked through with Pushpak** (2026-10-17); answers in "Decisions". This file is the plan for the linter:
how it works today, the decided design, and the build list (§7). Decisions with dates: ../PLAN.md §2. Shared
schema: config.md; output: output.md.

## 1. What it is

`pun lint` holds every rule that can change what code does. `pun lint` reports, and `pun lint --fix` applies.
It replaces three things:

| Upstream | What moves into lint |
| --- | --- |
| Rector (+ rector-laravel, rector-phpunit, pest-plugin-rector) | every rule except the Symfony, Doctrine and downgrade sets, which are unsupported (decided) |
| PHP-CS-Fixer / Pint | the 76 risky fixers (decided: risky rules are lint, format.md §4) |
| Mago's linter | its rules, as normal lint rules under Pun names (`mago.*` and `--mago` today) |

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
- Tiers, add-ons and severities are accepted by the config but don't do anything.

## 3. The config

The schema is config.md §3 and §5: `[lint] sets`, `[lint.groups]` and `[lint.rules]` take `"off"`, `"warn"` or
`"error"` (oxlint), options in a table with `level`, `fix = "off"` for report-only, and `[[lint.overrides]]` per path.
Only `error` fails; `--deny-warnings` and `--max-warnings <n>` tighten that, and `-A` / `-W` / `-D` set a level for one
run. `--fix` applies every fix of every enabled rule; safe or unsafe is a label in the output, not a gate. A
PHP-CS-Fixer or Rector name in `[lint.rules]` is an error that names the Pun ID to use.

*Today* pun-config reads this schema (config.md Build 3) and resolves sets, groups and rule lines into
`Config.lint.rules`; pun-cli runs the ones a token fixer backs and `--fix` applies all but `fix = "off"`. Overrides and
the `warn` / `error` difference are not applied at run time yet (lint 1-2).

## 4. Rules: what exists

There are 703 lint rules in crates/pun-config/rules/mapping.toml; 428 are for 1.0 and 275 carry `later = true` (PHPUnit and Pest lint rules, and rules in no set; PLAN.md §2, 2026-10-18). Each has its group, fix safety, sets and upstream
sources. Each maps to exactly one in-scope upstream rule (a Rector rule in one of the planned sets, a
PHP-CS-Fixer fixer that is risky or moved from format, or a strict-rules class), or is Mago-derived (`origin = "mago"`); rules.md §7 has the reasons and
the lists of what was removed and added on 2026-10-18.

| | Count |
| --- | ---: |
| Lint rules | 703 |
| Ported (run natively today) | 50 (risky CS-Fixer fixers plus Pun's ports of Mago rules) |
| Its one upstream rule: Rector core / rector-laravel / rector-phpunit / pest-plugin-rector | 264 / 80 / 79 / 59 |
| Its one upstream rule: PHP-CS-Fixer fixer (76 risky, 39 code rules moved from format) / Pint / strict-rules class | 115 / 1 / 17 |
| Mago-derived (no other source) / Pun-only | 87 / 1 |
| Also carrying a Mago mapping | 43 |
| Fix safety: safe / unsafe / mixed / report only | 310 / 307 / 7 / 79 |
| Sets: dead-code / modernize / code-quality / types / early-return / privatization / naming / coding-style / none | 78 / 37 / 238 / 87 / 6 / 5 / 0 / 117 / 138 |
| Ecosystem groups: laravel (+ laravel-12) / phpunit / pest | 89 (+1) / 111 / 62 |

Groups today are intent groups, as in Biome: `correctness`, `suspicious`, `style`, `complexity`, `performance`,
`security`, plus `dead-code`, `types`, `modernize`, `laravel`, `testing` and `refactor` (configurable codemods such as
Rector's `RenameMethod`). Decided: they become topic groups shared with format (format.md §8), and intent stays as
data.

Unsupported on purpose: the rector-symfony (116), rector-doctrine (59) and rector-downgrade-php (92) rules. `pun migrate`
names each one as unsupported and skips it (migrate.md §3).

## 5. Rector's CLI, flag for flag

| Rector | Pun |
| --- | --- |
| `rector process --dry-run` | `pun lint` |
| `rector process` | `pun lint --fix` (doesn't format afterwards, as Rector doesn't; `pun check --fix` does both) |
| `--config rector.php` | `--config pun.toml`; rector.php is read only by `pun migrate` |
| `--only=SomeRector` | `--only <rule>` (to build) |
| `--output-format console/json/github/gitlab/junit` | `--format`, same formats as `pun format` (format.md §9) |
| `--no-diffs`, `--clear-cache`, `--no-progress-bar` | the same flags `pun format` already has |
| `rector list-rules` | `pun explain <rule>` (config.md Build 8) |

## 6. How lint edits code (decided: text edits)

Rector parses with nikic/php-parser, changes the syntax tree, and prints the changed nodes back with php-parser's
format-preserving printer. Matching Rector's bytes would mean porting that printer and its quirks.

The alternative is what Ruff, Biome, oxlint and Mago do:
- each rule finds a node in Mago's syntax tree (already a Pun dependency) and returns text edits on byte ranges;
- the engine applies the non-overlapping edits and loops until no rule has more;
- `pun format` (or `pun check --fix`) then fixes the layout of what was inserted.

This needs no printer. The result is the same code change as Rector's, but the whitespace inside an inserted node can
differ until format runs. Parity is then checked as `format(pun lint --fix)` equals `format(rector process)`, using
Rector's own fixtures and the corpus.

The same Mago syntax tree is what `pun analyse` uses, so `pun check` can parse each file once for lint and analyse.
Type-aware rules (`add-return-type` and others) ask pun-analyse for types, as Rector asks PHPStan.

## 7. Build

Done: the rule catalogue (703 rules with names, groups, safety and upstream mapping), the 31 token-path rules, and
`pun check --fix` equal to Pint. The steps, after config.md Build 1-6 (names, sets, levels and overrides in pun.toml):

1. **The engine on Mago's syntax tree**, starting as a spike alongside PLAN.md Step 1 (Pushpak, 2026-10-17). A rule visits the tree and returns problems (span, message, help) and text
   edits on byte ranges. Fixes apply in a loop until no rule has more, with overlapping edits deferred to the next
   round. One parse per file, files in parallel, a cache keyed like format's.
   *Done when* two sample rules (one safe, one unsafe) pass Rector's fixtures through `format(pun lint --fix)`.
2. **Levels, sets and overrides at run time:** `off`/`warn`/`error`, group levels, `[[lint.overrides]]`,
   `--deny-warnings`, `--max-warnings`, `-A`/`-W`/`-D`, `fix = "off"`, and the exit code (1 only for `error`).
   *Done when* each flag and key has a test with a hand-derived expected result.
3. **Per-line problems for the token path,** so CS-Fixer's risky rules report a line and a frame, not just a file.
4. **Ignore comments:** `@pun-ignore` on a line, the next line or a region (rules.md §4), and `no-unused-ignore`.
5. **Output** (output.md Build 1-3): problem blocks, `fixable` labels, `[fix:unsafe]`, the summary, `--concise`.
6. **Mago's rules as normal lint rules** under their Pun names; the `--mago` flag and the `mago-lint` feature removed.
7. **Rules in usage order,** each passing Rector's own fixtures: Pinkary's rector.php, then dead-code and modernize
   (init's default sets), then code-quality, types, early-return, privatization, naming, coding-style, then
   rector-laravel and the PHP 8.4+ and Laravel 12+ upgrade groups (Pest's Rector plugin and PHPUnit after 1.0).
   *Done when* the 428 rules for 1.0 are native (the 1.0 bar); the 275 `later` rules follow after 1.0.
8. **Type-aware rules** query analyse's index (analyse.md Build 3) instead of PHPStan.
9. **`pun migrate` for rector.php** (migrate.md Build 4).

## 8. `pun init` and lint presets (Pushpak's user journey, 2026-10-17)

**A new, blank Laravel project:** `pun init` asks three things and writes pun.toml.
1. **Format preset:** `laravel` or `psr12`.
2. **Analyse preset:** `basic`, `essential`, `recommended` (default), `strict` or `max`, each including the one below
   (analyse.md decisions).
3. **Lint sets:** below.

Laravel, Pest and PHPUnit are not questions. Pun reads composer.lock and turns their rules on by itself, including the
rules for the locked versions (decided).

**A project coming from Pint, Rector or PHPStan:** `pun migrate` writes the exact rules the old config runs. In
interactive mode it then offers the closest preset, with a count of the new problems the preset would report.

### What Rector offers (research)

Rector has no ladder. Its config is a set of independent toggles, `withPreparedSets(...)`, plus `withPhpSets()`
(newest syntax for the composer.json PHP version), plus ecosystem sets. Mapped onto Pun's lint rules
(rules/mapping.toml):

| Rector set | Pun lint rules | What it does |
| --- | ---: | --- |
| `deadCode` | 45 | removes unused and unreachable code |
| `codeQuality` | 55 | simplifies code (for example an `if` that returns true or false becomes one `return`) |
| `codingStyle` | 13 | style choices (for example `sprintf` instead of string interpolation) |
| `typeDeclarations` (+ docblocks) | 8 (72 Rector rules merged into `add-return-type` and similar) | adds native types it can prove |
| `earlyReturn`, `instanceOf`, `privatization`, `naming` | 8, 3, 4, 6 | each one style decision |
| `withPhpSets()` | about 100 (`php53` to `php85`) | newest syntax your PHP allows |
| rector-laravel `laravel-code-quality`, `laravel-collection`, `laravel-testing` | 24, 4, 4 | Laravel idioms |
| rector-phpunit `phpunit-code-quality` | 40 | PHPUnit idioms |
| pest-plugin-rector `coding-style` | 19 | Pest idioms |
| version sets (`laravel110`, `phpunit100`, ...) | 1-7 each | upgrades; in Pun these are the upgrade groups (`lint/laravel-13/...`) |

221 lint rules have no Rector source: CS-Fixer's risky and code fixers, strict-rules' syntax-only classes and Mago's rules.
Rector also has gradual levels for adoption (`withDeadCodeLevel(n)`, `withTypeCoverageLevel(n)`), which turn on the
first n rules of one set.

### Lint presets are sets you tick, not a ladder (decided)

`pun init` question 3 is a checklist, with a default ticked:

```
Which lint rules?            (space to toggle)
 [x] dead-code               remove unused and unreachable code
 [x] modernize               newest syntax your PHP and Laravel versions allow
 [ ] code-quality            simplify code
 [ ] types                   add native types Pun can prove
 [ ] early-return            prefer early returns
 [ ] privatization           make what can be private, private
 [ ] naming                  rename variables after their type
 [ ] coding-style            style choices
```

In pun.toml:

```toml
[lint]
sets = ["dead-code", "modernize", "code-quality"]
```

- **Sets are tags on rules**, separate from the topic group in the name. A rule can be in several sets.
- **The Laravel, Pest and PHPUnit parts of each set turn on by themselves** when composer.lock has the package. For
  example, `code-quality` in a Laravel app includes rector-laravel's code-quality rules.
- **Migrate maps 1:1:** `withPreparedSets(deadCode: true, codeQuality: true)` becomes
  `sets = ["dead-code", "code-quality"]`, and single rules become `[lint.rules]`.
- **CS-Fixer's risky rules and Mago's rules join the set they fit** (`no-loose-comparison` goes in `types`,
  `no-debug-calls` in `dead-code`), or stay opt-in by name.

The alternative is the same four-step ladder as analyse (`[lint] preset = "strict"`). It is simpler to pick, but it
bundles unrelated choices: wanting typed code doesn't mean wanting variables renamed.

## Decisions

Answered 2026-10-17 (Pushpak):

- **Text edits.** Rules return text edits on byte ranges found through Mago's syntax tree, and format tidies the
  layout afterwards. There is no php-parser printer port. Parity is `format(pun lint --fix)` equal to
  `format(rector process)`, on Rector's fixtures and the corpus.
- **Topic groups**, the same group list as format and analyse (`lint/imports/no-unused`,
  `lint/laravel/no-request-all`); intent (correctness, suspicious, style) stays as data. Each upgrade target gets its
  own group (`lint/laravel-13/...`, `lint/php-8.5/...`), so one line turns a whole upgrade on.
- **Sets for new projects, explicit rules for migrated ones.** Lint presets are a checklist of sets (§8):
  `[lint] sets = ["dead-code", "modernize", ...]`, with dead-code and modernize ticked by default in `pun init`.
  This replaced the first answer (a four-tier ladder shared with analyse). `pun migrate` writes the exact rules the old
  setup ran; Rector's prepared sets migrate 1:1, and in interactive mode it offers the closest sets with a count of the
  new problems they would report.
- **`pun init` asks three things:** the format preset (laravel / psr12), the lint sets and the analyse preset.
- **Laravel, Pest and PHPUnit turn on automatically** from composer.lock, version rules included; their group can be
  switched off in pun.toml. Nothing to install.
- **`--fix` applies every fix of every enabled rule**, as Rector does. There is no `--unsafe` flag. Safe/unsafe
  stays as a label in the output (`[fix:unsafe]`) so reviewers know which changes to read closely. A rule that should
  only report takes `fix = "off"`. `fix = "apply"` and `"safe-only"` go away.
- **Only `error` fails the run; `warn` is printed and never fails.** Every enabled rule is `error` unless set to
  `warn`; `pun migrate` sets everything to `error`, because PHPStan and Rector fail on anything they report.
- **Severity syntax and behaviour follow oxlint:**
  - Each rule takes `"off"`, `"warn"` or `"error"`. Options go in a table with `level`:
    `"imports/ordered" = { level = "warn", sort = "alphabetical" }`. `"info"` is dropped.
  - A whole group takes a level too, as oxlint does with categories: `[lint.groups] style = "warn"`. A rule's own line
    wins over its group's.
  - `--deny-warnings` makes warnings fail the run, and `--max-warnings <n>` fails above n warnings.
  - `-A` / `-W` / `-D <rule or group>` (allow / warn / deny) set a level for one run from the command line.
  - `[[lint.overrides]]` with `files = ["tests/**"]` changes levels for some paths, like oxlint's `overrides`.
- **Every lint rule for 1.0 runs natively before 1.0** (428 of 703, the rest after 1.0; Symfony, Doctrine and downgrade sets, upgrade sets below PHP
  8.4 and Laravel 12, and rules Rector deprecated stay unsupported).
  Porting order follows usage (§7 item 7), but 1.0 waits for all of them.
- **Mago's rules become normal lint rules** and `pun lint --mago` goes away (assistant's default, not objected to).
- **`pun lint --fix` doesn't format**; `pun check --fix` does lint, then format (commands.md §3).
- **Unported rules are never delegated.** `pun migrate` names and skips them; a hand-written pun.toml that names one
  is a config error (exit 2).
- **Upgrades are lint rules in their own groups**, not a separate command.

Lint spike (Build 1, 2026-10-17; report: ../reports/lint-spike.md):

- **Assistant's call: the rule shape.** A rule is called for every node with its ancestor stack and returns reports
  (span, message, help, optional `pun_problem::Fix`); it declares a lowest PHP version like Rector's
  `MinPhpVersionInterface`. The fix loop stops after 100 rounds (Ruff's limit); a round whose fixes break the syntax
  leaves the file unchanged and is an error naming the rules.
- **Assistant's call: the sample rules** are `CombinedAssignRector` (unsafe) and `RemoveUnusedVariableInCatchRector`
  (safe), both in Pinkary's rector.php sets, with provisional IDs `lint/style/prefer-compound-assignment` and
  `lint/php-8.0/no-unused-catch-variable` until config 1 fixes the groups. The second is out of scope since the
  one-to-one mapping (its Rector rule is only in the php80 set); pun-lint keeps it as a local ID until the perf rounds
  end, then it goes.
- **Assistant's call: keep meaning over Rector's bytes.** Where Rector's printer drops parentheses that change what
  the code does (`$a = $a + ($b or $c)` becomes `$a += $b or $c`), Pun keeps them.
- **Assistant's call: the Rector oracle** is rector-src at the tag Pinkary's lock pins (fixtures, `.oracle/rector`)
  plus the released rector/rector locked by `scripts/oracle-rector/composer.lock` (runnable, `.oracle/rector-bin`).

- **Assistant's call (lint 7 batch 2): where the new code goes in parentheses.** A rule builds its code from the
  source text of the nodes it keeps and adds parentheses where php-parser's precedence table (`pInfixOp`,
  `pPrefixOp`) puts them, also around an expression it moves into a new place (`'x' . sprintf($a ? $b : 'c')` becomes
  `'x' . ($a ? $b : 'c')`; Rector prints it bare). It leaves them out on the right of the same associative operator
  (`'x' . sprintf($a . $b)` is `'x' . $a . $b`, as Rector prints it).
- **Assistant's call (lint 7 batch 2): one statement into several.** In a single-statement body (`if ($a) stmt;`)
  Pun puts braces around the new statements, as Rector's reprint of the `if` does. In a colon body (`if ($a): ...
  endif;`) Pun keeps the colon syntax, where Rector reprints the whole `if` with braces. A statement ending in `?>`
  is left alone there.
- **Assistant's call (lint 7 batch 2): `in_array($x, array_values($y))`.** Pun replaces only the argument's value, and
  leaves the call alone when either argument is spread or `array_values()` has none (Rector crashes or drops the
  spread).
