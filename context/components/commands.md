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

   *Answered: (b), see §5.*
2. **One-time changes (version upgrades, codemods).** Answered: they stay in lint, as rule groups (no `pun upgrade`
   command). See "Decisions".
3. **Modernisation rules** (Rector's PHP sets as `prefer-*` rules, such as `prefer-match` and `prefer-readonly-property`)
   keep code on the newest syntax for the configured PHP version. They stay in lint (follows from 2).
4. **Lint reports without a fix** (`no-eval`, `no-debug-calls`, 68 rules). Should these stay in lint, or move to
   analyse because they only report?
   *Recommend: lint. The test is what a rule needs (one file vs the whole project), not whether it fixes. ESLint,
   Biome and Ruff all keep report-only rules in lint.*

## 4. How `pun check` works

`pun check` is the one command for CI, pre-commit and agents: lint, format and analyse in one process, reading and
parsing each file once.

**`pun check` (report only):**
1. **Find files** with the `[format]` / `[lint]` / `[analyse]` paths, and read the versions from composer.json and
   composer.lock.
2. **Index pass (parallel).** Parse every file once into Mago's syntax tree and collect its classes, functions and
   constants into one project index. Analyse needs this before it can check any file. Cached, so a warm run only
   re-parses changed files.
3. **Check pass (parallel, per file).** On the same parse: run the lint rules, run the format rules on the tokens (no
   writes), and run analyse against the index.
4. **Report.** One block per file, problems sorted by line, each tagged with its full rule ID and `[fix:safe]` /
   `[fix:unsafe]` when lint can fix it. When analyse and lint report the same thing (the 18 linked pairs), only the
   lint line prints. A file that only needs formatting prints one line.
5. **Summary and exit code.** For example `3 files need formatting · 12 lint problems (9 fixable) · 2 analyse errors ·
   214 files in 0.4 s`. The exit code is 1 if anything is at `error`, or anything needs formatting.

**`pun check --fix`:**
1. **Lint fixes**: safe fixes, plus rules with `fix = "apply"`, plus unsafe fixes with `--unsafe`. They run in a loop
   until no rule has more to fix, because one fix can expose the next.
2. **Format** the result, so whatever lint inserted gets the project's layout.
3. **Write** each changed file once.
4. **Analyse** the final code, re-indexing only the files that changed, and report what is left. Analyse never edits.
5. **Report** what was fixed per file, then what remains, as in the report-only run.

The order is lint, then format, then analyse, because each step works on the previous step's output. Lint changes code,
format tidies it, and analyse checks what you will actually commit.

**Pint parity.** On a project migrated from pint.json, `pun check --fix` writes the same bytes as `pint`. CS-Fixer's
risky fixers stay token fixers and run inside step 2 in PHP-CS-Fixer's priority order, as they do today.

**The single commands** stay for editors and for running one step: `pun format` on save, `pun lint --fix`, and
`pun analyse` for PHPStan-style runs.

## 5. Which format rules move to lint (follows from Q1)

**The test.** Format may only:
- change whitespace, line breaks and indentation;
- change casing;
- change how the same construct is spelled (`array()` to `[]`, quotes, cast spelling, `else if` to `elseif`,
  `\Foo` to an import);
- sort lists whose order means nothing (imports, union types, `implements` lists, PHPDoc tags).

Anything else moves to lint with a safe fix:
- removing code;
- adding or removing PHPDoc;
- rewriting one construct into another;
- changing a declaration or the order of class members.

**Result:** 40 of the 208 format rules move. 11 of them are in the laravel preset and 2 in psr12, so format keeps 168
rules: laravel 115, psr12 50. This list is for review; say if any rule is on the wrong side.

| Why it moves | Rule today (proposed format name) | laravel | psr12 |
| --- | --- | :-: | :-: |
| removes code | `no-unused-imports` (`imports/no-unused`) | ✓ | ✓ |
| removes code | `no-useless-else` (`control-flow/no-useless-else`) |  |  |
| removes code | `no-useless-return` (`control-flow/no-useless-return`) | ✓ |  |
| removes code | `no-empty-statement` (`statements/no-empty`) | ✓ |  |
| removes code | `no-superfluous-elseif` (`control-flow/no-superfluous-elseif`) |  |  |
| removes code | `no-useless-nullsafe-operator` (`operators/no-useless-nullsafe`) |  |  |
| removes code | `no-null-property-initialization` (`classes/no-null-property-default`) |  |  |
| removes code | `no-redundant-readonly-property` (`classes/no-redundant-readonly`) |  |  |
| removes code | `lambda-not-used-import` (`functions/no-unused-closure-use`) | ✓ |  |
| removes code | `general-attribute-remove` (`attributes/remove`) |  |  |
| removes or adds PHPDoc | `no-superfluous-phpdoc-tags` (`phpdoc/no-superfluous-tags`) | ✓ |  |
| removes or adds PHPDoc | `general-phpdoc-annotation-remove` (`phpdoc/remove-annotation`) |  |  |
| removes or adds PHPDoc | `general-phpdoc-tag-rename` (`phpdoc/rename-tag`) | ✓ |  |
| removes or adds PHPDoc | `phpdoc-no-access` (`phpdoc/no-access`) | ✓ |  |
| removes or adds PHPDoc | `phpdoc-no-package` (`phpdoc/no-package`) | ✓ |  |
| removes or adds PHPDoc | `phpdoc-no-empty-return` (`phpdoc/no-empty-return`) |  |  |
| removes or adds PHPDoc | `phpdoc-no-useless-inheritdoc` (`phpdoc/no-useless-inheritdoc`) | ✓ |  |
| removes or adds PHPDoc | `phpdoc-add-missing-param-annotation` (`phpdoc/add-missing-param`) |  |  |
| removes or adds PHPDoc | `phpdoc-to-comment` (`phpdoc/to-comment`) |  |  |
| removes or adds PHPDoc | `phpdoc-tag-no-named-arguments` (`phpdoc/tag-no-named-arguments`) |  |  |
| removes or adds PHPDoc | `phpdoc-type-annotations-only` (`laravel/phpdoc-type-annotations-only`) |  |  |
| rewrites one construct into another | `combine-consecutive-issets` (`control-flow/combine-issets`) |  |  |
| rewrites one construct into another | `combine-consecutive-unsets` (`control-flow/combine-unsets`) |  |  |
| rewrites one construct into another | `simplified-if-return` (`control-flow/simplified-if-return`) |  |  |
| rewrites one construct into another | `simplified-null-return` (`control-flow/simplified-null-return`) |  |  |
| rewrites one construct into another | `return-assignment` (`control-flow/no-return-assignment`) |  |  |
| rewrites one construct into another | `ternary-to-null-coalescing` (`operators/ternary-to-coalesce`) |  |  |
| rewrites one construct into another | `assign-null-coalescing-to-coalesce-equal` (`operators/coalesce-equal`) |  |  |
| rewrites one construct into another | `switch-continue-to-break` (`control-flow/switch-continue-to-break`) |  |  |
| rewrites one construct into another | `return-to-yield-from` (`control-flow/yield-from`) |  |  |
| changes declarations or their order | `protected-to-private` (`classes/protected-to-private`) |  |  |
| changes declarations or their order | `self-static-accessor` (`classes/self-static-accessor`) | ✓ |  |
| changes declarations or their order | `stringable-for-to-string` (`classes/stringable-for-to-string`) |  |  |
| changes declarations or their order | `ordered-class-elements` (`classes/ordered-members`) |  | ✓ |
| changes declarations or their order | `phpunit-method-casing` (`phpunit/method-casing`) | ✓ |  |
| changes declarations or their order | `phpunit-data-provider-method-order` (`phpunit/data-provider-order`) |  |  |
| adds test metadata | `phpunit-attributes` (`phpunit/attributes`) |  |  |
| adds test metadata | `phpunit-internal-class` (`phpunit/internal-class`) |  |  |
| adds test metadata | `phpunit-size-class` (`phpunit/size-class`) |  |  |
| adds test metadata | `phpunit-test-class-requires-covers` (`phpunit/require-covers`) |  |  |

What changes for users:
- `pun format` and format-on-save never delete or restructure code.
- `pun check --fix` still writes Pint's bytes, because these stay token fixers and run in the same priority-ordered
  pass as format.
- `pun migrate` writes the ones a pint.json enables under `[lint.rules]`. They are safe, so `--fix` applies them with
  no `fix = "apply"`.
- Moving `simplified-null-return` to lint also settles its conflict with `require-consistent-return` inside one
  command.
- These rules then take lint names and groups, so the §9 format table in format.md loses these 40 rows.

## Decisions

Answered 2026-10-17 (Pushpak):

- **One job per command.** format = how it looks, lint = a better way to write it, analyse = is it wrong. The three
  questions in §2 decide where every rule goes.
- **`pun check` runs lint (with fixes), format and analyse together** (§4).
- **No `pun upgrade` command.** Version upgrades (PHP, Laravel, PHPUnit, Pest) and codemods are lint rules in their
  own groups, named the same way as format rules (`group/name`).
- **`pun migrate` must carry an old config over completely**: pint.json, .php-cs-fixer.php, rector.php and
  phpstan.neon. Anything Pun doesn't support yet is named explicitly to the user, never dropped silently.
- **No levels.** PHPStan's levels don't come over; rules are turned on by name and by presets.
- **Every PHPStan core rule is supported natively.**
- **Fixers that delete or restructure code move from format to lint** (Q1 (b), decision card). The list is in §5.
- **Laravel support is built in.** Nothing to install or import, unlike Larastan or rector-laravel.
