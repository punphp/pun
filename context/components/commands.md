# Who does what: format, lint, analyse

Status: **under review with Pushpak** (opened 2026-10-17). He asked for this: today's PHP tools each promise one job and
then do several, and Pun should make each command do exactly the job its name promises. This file explains what each
upstream tool is for, where it oversteps, and the simple split Pun uses. It sits above format.md, lint.md and the
future analyse.md: those plan each command, and this file decides which rule belongs to which command.

## 1. What each tool is for, and what it also does

| Tool | Its real job | What it also does (the overlap) |
| --- | --- | --- |
| **PHP-CS-Fixer / Pint** | Code style: spaces, line breaks, casing, quotes, import order | 76 "risky" fixers that change behaviour: `strict_comparison` (`==` to `===`), `declare_strict_types`, `final_class`, `modernize_strpos`, `psr_autoloading` (renames classes). It also does some refactoring: removes unused imports, useless `else` and dead code |
| **Rector** | Automated refactoring: rewrite code to a newer or better form | Also: version upgrades (PHP 8.4, Laravel 12, PHPUnit 11), configurable codemods (rename this method everywhere), type inference that adds types (which needs PHPStan), coding style (48 of its rules duplicate CS-Fixer's format rules), and dead-code removal |
| **PHPStan / Larastan** | Find bugs using types: undefined methods, wrong argument types, impossible conditions | Also: syntax-only bans that need no types (strict-rules' `empty.notAllowed`, `equal.notAllowed`), whitespace checks (`whitespace.bom`), and coding policy (strict-rules) |
| **Mago** (for reference) | All three in one binary: formatter, linter, analyser | Its formatter reprints the whole file, so it can't match Pint byte for byte |

So the same idea often lives in three or four tools at once. Strict comparison is the worked example (RULES.md §2):

| Tool | Rule | What it does |
| --- | --- | --- |
| PHP-CS-Fixer | `strict_comparison` | rewrites every `==` (risky) |
| Rector | `UseIdenticalOverEqualWithSameType` | rewrites `==` only when both sides have the same type |
| PHPStan strict-rules | `equal.notAllowed` | reports every `==` |
| Mago | `identity-comparison` | reports every `==` |
| **Pun** | `lint/…/no-loose-comparison` | one rule: reports every `==`, safely fixes the same-type ones, and fixes the rest with `--unsafe` |

Across all tools, the mapping found 105 such overlap clusters and 109 + 82 merges (RULES.md §9). Pun ends up with one rule
per idea, in one command.

## 2. Pun's split: three questions

Each command answers one question and makes one promise.

| Command | Question | Promise | Needs | Edits |
| --- | --- | --- | --- | --- |
| `pun format` | How does the code look? | Never changes what the code does | the file's tokens | always, safe on save |
| `pun lint` | Is there a better way to write this? | Every change is labelled safe or unsafe; unsafe changes only happen when you ask for them | one file's syntax tree (types optional, to make more fixes safe) | with `--fix` |
| `pun analyse` | Is this wrong? | Never touches your files | types and symbols from the whole project | never |

`pun check` runs all three in one pass and is what CI and agents run. `pun migrate` only moves config onto pun.toml.

**How a rule is placed** (RULES.md §1, the first match wins):
1. It changes only layout and keeps behaviour → **format**.
2. It edits code in any other way, or it reports something one file can decide → **lint**.
3. It needs types or other files to decide → **analyse**.

**Where this cleans up each tool:**
- **CS-Fixer's risky fixers go to lint.** `pun format` can no longer change behaviour, so format-on-save is always
  safe. (Decided; `pun check --fix` is what equals `pint`.)
- **Rector's style rules go to format.** The 48 Rector rules that duplicate a CS-Fixer format rule map onto that rule.
  The rest of Rector is lint.
- **PHPStan's syntax-only bans go to lint**, where no types are needed (strict-rules' 16 bans become 9 lint rules).
  PHPStan core's own identifiers stay in analyse until 1.0 so `pun analyse` matches `phpstan analyse` on day one (decided).
- **When analyse finds something lint can fix**, the two rules are linked: `analyse/…/unused-private-method` reports it
  and `lint/…/no-unused-private-method` fixes it. `pun check` shows it once (18 such pairs).

## 3. Where the line is still blurry (the questions)

1. **Safe refactors inside format.** CS-Fixer runs some fixers that keep behaviour but delete or move code you wrote:
   `no_unused_imports`, `no_useless_else`, `no_useless_return`, `no_empty_statement`, `ordered_class_elements`. Pint
   calls them formatting, so today they are format rules. By the three questions they are lint (Ruff, for example,
   keeps unused imports in lint).
   - **(a) Keep them in format.** `pun format` alone stays closest to Pint.
   - **(b) Move them to lint with safe fixes.** `pun format` becomes pure layout. `pun check --fix` still equals `pint`,
     because it runs both.

   *Recommend: (b). Pint parity is already proven on `pun check --fix`, so nothing is lost, and format then means only
   "how it looks". If you agree, I'll count exactly which of the 208 format rules move.*
2. **One-time changes inside lint.** Rector mixes rules you keep on forever (`prefer-early-return`) with rules you run
   once: version upgrades (Laravel 11 to 12, PHPUnit 10 to 11), configurable codemods (`rename-method` with a mapping),
   and upgrades that are wrong to run twice (`no-cache-minutes` multiplies by 60). Today these are lint rules that no
   tier turns on (59 rules), and PHP upgrades are `pun lint --set php85` (decided).
   - **(a) Keep them in lint** as opt-in rules and sets.
   - **(b) A separate `pun upgrade` command** for one-time changes (`pun upgrade laravel 12`, `pun upgrade php 8.5`,
     `pun upgrade --rename OldClass=NewClass`). Lint keeps only rules that are always on.

   *Recommend: (b). Lint then means "rules you keep on". Upgrades are something you run once and commit, like
   `npx @next/codemod` or `ng update`.*
3. **Modernisation rules** (Rector's PHP sets as `prefer-*` rules, such as `prefer-match` and `prefer-readonly-property`)
   keep code on the newest syntax for the configured PHP version. Should these stay in lint even if (2b) happens?
   *Recommend: yes. They are ongoing ("use the newest form"), not one-time.*
4. **Lint reports without a fix** (`no-eval`, `no-debug-calls`, 68 rules). Should these stay in lint, or move to
   analyse because they only report?
   *Recommend: lint. The test is what a rule needs (one file vs the whole project), not whether it fixes. ESLint,
   Biome and Ruff all keep report-only rules in lint.*

## Decisions

None yet.
