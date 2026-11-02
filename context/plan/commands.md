# Plan: who does what (format, lint, analyse) and `pun check`

Status: **walked through with Pushpak** (2026-10-17); answers in "Decisions". Pushpak asked for this: today's PHP tools each
promise one job and then do several, and Pun makes each command do exactly the job its name promises. This file
explains what each upstream tool is for, where it oversteps, the split Pun uses, and how `pun check` runs (build
list: §5).

## 1. What each tool is for, and what it also does

| Tool | Its real job | What it also does (the overlap) |
| --- | --- | --- |
| **PHP-CS-Fixer / Pint** | Code style: spaces, line breaks, casing, quotes, import order | 76 "risky" fixers that change behaviour: `strict_comparison` (`==` to `===`), `declare_strict_types`, `final_class`, `modernize_strpos`, `psr_autoloading` (renames classes). It also does some refactoring: removes unused imports, useless `else` and dead code |
| **Rector** | Automated refactoring: rewrite code to a newer or better form | Also: version upgrades (PHP 8.4, Laravel 12, PHPUnit 11), configurable codemods (rename this method everywhere), type inference that adds types (which needs PHPStan), coding style (48 of its rules duplicate CS-Fixer's format rules), and dead-code removal |
| **PHPStan / Larastan** | Find bugs using types: undefined methods, wrong argument types, impossible conditions | Also: syntax-only bans that need no types (strict-rules' `empty.notAllowed`, `equal.notAllowed`), whitespace checks (`whitespace.bom`), and coding policy (strict-rules) |
| **Mago** (for reference) | All three in one binary: formatter, linter, analyser | Its formatter reprints the whole file, so it can't match Pint byte for byte |

So the same idea often lives in three or four tools at once. Strict comparison is the worked example (rules.md §2):

| Tool | Rule | What it does |
| --- | --- | --- |
| PHP-CS-Fixer | `strict_comparison` | rewrites every `==` (risky) |
| Rector | `UseIdenticalOverEqualWithSameType` | rewrites `==` only when both sides have the same type |
| PHPStan strict-rules | `equal.notAllowed` | reports every `==` |
| Mago | `identity-comparison` | reports every `==` |
| **Pun** | `lint/…/no-loose-comparison` | one rule: reports every `==`, and `--fix` rewrites all of them; the ones without proof that both sides have the same type are labelled unsafe |

Across all tools, the mapping found 105 such overlap clusters and 109 + 82 merges (rules.md §6). Pun ends up with one rule
per idea, in one command.

## 2. Pun's split: three questions

Each command answers one question and makes one promise.

| Command | Question | Promise | Needs | Edits |
| --- | --- | --- | --- | --- |
| `pun format` | How does the code look? | Never changes what the code does | the file's tokens | always, safe on save |
| `pun lint` | Is there a better way to write this? | Every change is labelled safe or unsafe; `--fix` applies all of them (`fix = "off"` keeps a rule report-only) | one file's syntax tree (types optional, to make more fixes safe) | with `--fix` |
| `pun analyse` | Is this wrong? | Never touches your files | types and symbols from the whole project | never |

`pun check` runs all three in one pass and is what CI and agents run. `pun migrate` only moves config onto pun.toml.

**How a rule is placed** (rules.md §1, the first match wins):
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
- **When lint or format can fix what analyse finds, analyse drops it** (Pushpak, 2026-11-02): PHPStan's `method.unused`
  is `lint/…/no-unused-private-method`, and `pun analyse` no longer reports it. Analyse keeps only reports nothing fixes.

## 3. How `pun check` works

`pun check` is the one command for CI, pre-commit and agents: lint, format and analyse in one process, reading and
parsing each file once.

**`pun check` (report only):**
1. **Find files** from `[files]` plus each command's own `exclude` (config.md §3), and read the versions from
   composer.json and composer.lock.
2. **Index pass (parallel).** Parse every file once into Mago's syntax tree and collect its classes, functions and
   constants into one project index. Analyse needs this before it can check any file. Cached, so a warm run only
   re-parses changed files.
3. **Check pass (parallel, per file).** On the same parse: run the lint rules, run the format rules on the tokens (no
   writes), and run analyse against the index.
4. **Report, grouped by step** (output.md Decisions): one status line each for format, lint and analyse, with the
   step's problems under it, each tagged with its full rule ID and `[fix:safe]` / `[fix:unsafe]` when lint can fix it.
   When analyse and lint report the same thing (the 18 linked pairs), only the lint problem prints.
5. **Summary and exit code.** For example `3 files need formatting · 12 lint problems (9 fixable) · 2 analyse errors ·
   214 files in 0.4 s`. The exit code is 1 if anything is at `error`, or anything needs formatting.

**`pun check --fix`:**
1. **Lint fixes**: every fix of every enabled rule, except rules with `fix = "off"` (decided: no `--unsafe`). They run in a
   loop
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

## 4. Which format rules move to lint

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
- `pun migrate` writes the ones a pint.json enables under `[lint.rules]` at `error`, and `--fix` applies them.
- Moving `simplified-null-return` to lint also settles its conflict with `require-consistent-return` inside one
  command.
- These rules then take lint names and groups, so the §8 format table in format.md loses these 40 rows.

## 5. Build (`pun check`)

`pun check` grows as the commands land; each step keeps `pun check --fix` equal to Pint on pint.json projects.

1. **The 40 rules move** (config.md Build 2): format stops running them, lint does.
2. **Grouped output** (output.md Build 5) over today's format and lint steps.
3. **One parse per file:** Mago's tree shared by lint and analyse, tokens for format, an index pass before checking.
4. **Analyse joins** when `pun analyse` exists (analyse.md Build 1): report-only after lint fixes and format.
5. **`--fix` loop:** lint fixes until stable, format, write each file once, re-index changed files, analyse.
   *Done when* `pun check --fix` equals running `pun lint --fix`, `pun format` and `pun analyse` in order on every
   corpus project.

## Decisions

Answered 2026-10-17 (Pushpak):

- **One job per command.** format = how it looks, lint = a better way to write it, analyse = is it wrong. The three
  questions in §2 decide where every rule goes.
- **`pun check` runs lint (with fixes), format and analyse together** (§3).
- **No `pun upgrade` command.** Version upgrades (PHP, Laravel, PHPUnit, Pest) and codemods are lint rules in their
  own groups, named the same way as format rules (`group/name`).
- **`pun migrate` must carry an old config over completely**: pint.json, .php-cs-fixer.php, rector.php and
  phpstan.neon. Anything Pun doesn't support yet is named explicitly to the user, never dropped silently.
- **No levels.** PHPStan's levels don't come over; rules are turned on by name and by presets.
- **Every PHPStan core rule is supported natively.**
- **Fixers that delete or restructure code move from format to lint**, so format only changes layout, casing and
  spelling. The list is in §4.
- **Laravel support is built in.** Nothing to install or import, unlike Larastan or rector-laravel.
- **Modernisation rules** (Rector's PHP sets as `prefer-*` rules) are lint rules, gated by the PHP version.
- **Report-only rules stay in lint** (`no-eval`, `no-debug-calls`, 68 rules): placement goes by what a rule needs
  (one file or the whole project), not by whether it fixes (assistant's default, follows from §2).
