# Pun plan

The global plan: what Pun is going to be, in what order we build it, and where each part's detailed plan lives.
Rewritten 2026-10-17 after walking through every command with Pushpak. Every decision with its date is in
DECISIONS.md; the task board is TASKS.md. The phase plan before the reset is `archive/PLAN-2026-10-17.md`.

Nothing in the new design is built yet unless a plan file says so under "Today". Building starts when Pushpak says go.

## 1. The plan files

One file per command or shared part, all in `plan/`. Each has the same shape: what it is, how it works today, the
decided design, **Build** (the ordered work list with a "done when" for each step), and Decisions.

| File | What it covers | Old component |
| --- | --- | --- |
| [plan/commands.md](plan/commands.md) | Who does what (format, lint, analyse), the 40 rules that move to lint, and how `pun check` runs | C4 |
| [plan/config.md](plan/config.md) | pun.toml, rule names, groups, presets, sets, levels, composer.lock detection | C3 |
| [plan/output.md](plan/output.md) | The terminal output every command shares, colours, machine formats, agent mode | C4 |
| [plan/format.md](plan/format.md) | `pun format`: the token layer, the fixers, Blade, speed | C1, C2, C5 |
| [plan/lint.md](plan/lint.md) | `pun lint`: the engine on Mago's tree, levels, sets, the 542 rules | C8 |
| [plan/analyse.md](plan/analyse.md) | `pun analyse`: Mago's analyser, PHPStan rules, Larastan, Pest, presets, baseline, cache | C7 |
| [plan/migrate.md](plan/migrate.md) | `pun migrate`: pint.json, .php-cs-fixer.php, rector.php, phpstan.neon, unsupported items | C9 |
| [plan/init.md](plan/init.md) | `pun init`: the three questions, the first check | C3, C4 |
| [plan/foundations.md](plan/foundations.md) | CI and gates, the parity harness, docs site, release, what comes after 1.0 | C0, C10-C14 |

## 2. What Pun is (decided)

**Commands, one job each.**
- `pun format`: how the code looks. Layout, casing and spelling only. It never changes behaviour and never deletes code.
- `pun lint [--fix]`: is there a better way to write this? Rector, PHP-CS-Fixer's risky fixers, Mago's lint rules, and
  version upgrades as lint groups (there is no `pun upgrade`).
- `pun analyse`: is this wrong? PHPStan, Larastan and Pest's PHPStan plugin. It never edits.
- `pun check [--fix]`: all three in one pass (lint fixes, then format, then analyse). This is the command for CI and
  agents.
- `pun migrate`: moves pint.json, .php-cs-fixer.php, rector.php and phpstan.neon onto pun.toml completely. It names
  every item Pun doesn't support, skips it, and asks for a PR to punphp/pun. It never copies a baseline.
- `pun init`: a new project's pun.toml from three questions (format preset, lint sets, analyse preset).
- `pun config`: prints the resolved config. `pun explain <rule>`: a rule's docs and upstream names.
  `pun list-files [--command format|lint|analyse] [-0]`: the files a command would check, like Mago's (config.md §7).
- The LSP is the last step, after 1.0's commands. Nothing is in the repo for it now.

**Config: pun.toml only.** Without one, every command stops with "Run `pun migrate`, or `pun init` for a new project".

| | Format | Lint | Analyse |
| --- | --- | --- | --- |
| Preset | `laravel` (default), `psr12`, `empty` | a checklist of sets (`sets = ["dead-code", "modernize"]`) | `basic`, `essential`, `recommended` (default), `strict`, `max` |
| Rule names | `format/<group>/<name>` | `lint/<group>/<name>`; upgrades get their own group (`lint/laravel-13/...`) | `analyse/<group>/<name>` |
| Levels | none: a diff is a diff | `"off"`, `"warn"`, `"error"` per rule or group, as oxlint; only `error` fails | on or off; every finding is an error |
| Fixes | always applied | `--fix` applies every fix of every enabled rule; `fix = "off"` makes a rule report-only | never |
| Options | TOML-native names, not copied from PHP-CS-Fixer | the same | the same |

- One group list is shared by all three commands. There are no aliases; renames are fine until 1.0.
- Laravel, Pest and PHPUnit support is built in and turns on by itself from composer.lock, with version-specific rules.
  There is no extension system before 1.0.
- `[php] version` comes from composer.json; the config value is only an override.

**No PHP fallback.** Pun never runs PHP-CS-Fixer, Pint, Rector or PHPStan for a rule it doesn't have. Delegate mode is
removed and no PHP is embedded. System PHP is only used by tests, to compare against the real tools.

**Engines.**
- Format: the PHP-CS-Fixer token-fixer port. On a project migrated from pint.json, `pun check --fix` writes the same
  bytes as `pint`.
- Lint: rules return text edits on byte ranges found through Mago's syntax tree, and format tidies the layout after.
- Analyse: Mago's analyser for inference, with Pun rules mapped one to one from PHPStan's identifiers. Laravel and Pest
  are built-in Mago plugins. Messages may be reworded.

**Output.** Pun's own coloured output for every command (plan/output.md), in the brand colours. Problem blocks with
code by default, `--concise` for one line each. `pun check` prints one status line per step, like Vite+. GitHub
annotations are automatic in Actions. Pint's human screen is dropped; exit codes and machine formats stay Pint and
PHPStan compatible. Agent mode keeps its compact lines.

**1.0 ships everything at once.**
- Every lint rule (542) is native.
- Every PHPStan core rule is native, and Larastan is built in with seven improvements (typed config, routes, views,
  translations, env keys, Inertia pages, fresh model data).
- Pest is built in, with the PHPUnit typing it needs. Deprecation rules and strict-rules come as normal rules.
- A result cache. Speed and memory come first in every command.

## 3. Build order

Each step names the plan file that holds its detail and the gate that ends it. Steps 1 and 2 come first because every
command depends on the names, the config and the output. After step 2, three tracks run in parallel threads.

### Step 1. The new names and config (plan/config.md, Build 1-6)

Rename every rule to `<command>/<group>/<name>`, move the 40 code-deleting format rules to lint, write the new pun.toml
schema (shared `[files]`, presets, sets, levels, groups, overrides), add `pun list-files`, read the PHP version and ecosystems from composer.json and
composer.lock, and remove delegate mode.
**Gate:** the 7-project corpus still gives 0 differing files with `pun migrate` + `pun check --fix`, and
`scripts/check-rule-mapping.py` passes on the new names.

### Step 2. The shared output (plan/output.md, Build 1-5)

One renderer in pun-cli for file lines, problem blocks, the summary line, run errors, brand colours with fallbacks,
GitHub annotations and `--concise`. `pun format` moves onto it first, and Pint's human screen goes.
**Gate:** every human output sample in the plan files is pinned by a test; machine formats and exit codes still pass
`scripts/cli-parity.sh`.

### Step 3, in three parallel tracks

**Track A: format, migrate, init** (plan/format.md, plan/migrate.md, plan/init.md)
1. Format on the new design: TOML option names, presets cut to laravel, psr12 and empty, speed and memory work.
2. `pun migrate` for pint.json and .php-cs-fixer.php on the new names, with the unsupported list and the PR request.
3. `pun init`.
**Gate:** the Pinkary CI swap from `vendor/bin/pint` to Pun, and the 8-core benchmark (laravel-framework under 1 s cold).

**Track B: lint** (plan/lint.md)
1. The engine on Mago's tree, with per-line problems, levels, sets, overrides and ignore comments.
2. Mago's rules and CS-Fixer's risky rules on the engine; `--mago` removed.
3. Rules in usage order until all 542 are native, then `pun migrate` for rector.php.
**Gate:** `format(pun lint --fix)` equals `format(rector process)` on Rector's fixtures and the corpus, for every rule.

**Track C: analyse** (plan/analyse.md)
1. `pun analyse` wired up: the command, `[analyse]`, the five presets, output, baseline.
2. Level 0 finished, then type inference, then levels 1 to 10, strict-rules and deprecation rules.
3. Larastan built in with the seven improvements; Pest built in.
4. The result cache; `pun migrate` for phpstan.neon.
**Gate:** the same errors as PHPStan + Larastan (file, line, identifier) on the corpus, at every preset.

### Step 4. `pun check` in one pass (plan/commands.md §4 and its Build)

One parse per file shared by lint, format and analyse; lint fixes, format, then analyse; the grouped output. Each
command joins as soon as its track has it, so this step closes when the last one joins.
**Gate:** `pun check --fix` on a pint.json project equals `pint`, and on every corpus project it equals running the
three commands one after the other.

### Step 5. 1.0 (plan/foundations.md)

Docs pages for every command and rule, `pun explain`, the generated "Coming from" page, AGENTS.md updated for the new
output, name checks, signing and publishing.

## 4. Where the code stands today

Last checked against main at 0800a99 (2026-10-17).

| Part | State |
| --- | --- |
| Tokens and format (pun-tokens, pun-format, pun-fixers, pun-workspace) | Done for the old design: Pint-identical on 7 projects, 144 of 208 format rules native, cache, watch, `--dirty`/`--staged`/`--since` |
| Config (pun-config) | Old schema: `format.<name>` IDs with aliases, tiers, `[format]` and `[lint.rules]` |
| Output (pun-cli) | Pint's screen and reporters, agent mode; none of the new renderer |
| Lint (pun-cli lint) | 31 of 542 rules: CS-Fixer's risky token fixers and 17 `Pun/*` rules; `--mago` behind a feature |
| Analyse (pun-phpstan, pun-analyse) | Config, ignores, baselines, symbols, type model; level 0 in progress (54 identifiers); no CLI command |
| Migrate (pun-cli, pun-config) | pint.json and .php-cs-fixer.php to the old schema; interactive picker; PHPStan, Rector and Mago "not supported yet" |
| Delegate (pun-delegate) | Works on system PHP; to be removed (Step 1) |
| Init | Not started |

## 5. Open questions

1. **Parallel threads.** One thread per track, each on its own branch with a PR into main, instead of agents sharing
   one box and git index. This needs AGENTS.md's "Work with other agents" rewritten (plan/foundations.md).
2. **After 1.0.** Are `pun install`, `pun run` and `pun test` (the archived plan's phases 5-7) part of 1.0 or after it?
3. **`--remove-tools` in migrate.** Should migrate offer to remove Pint, PHPStan and Rector from composer.json? The
   default is no: the future `pun-migrate` agent skill does workflow changes.
