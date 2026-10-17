# Component: format (`pun format`)

Status: **under review with Pushpak** (2026-10-17; first answers in "Decisions"). This file is the plan for the formatter: how it works today, what
is left, and the questions we still have to settle. Answers go into "Decisions" below and into DECISIONS.md. PLAN.md C1,
C2, C5 and C6 point here.

## 1. What it is

`pun format` rewrites the layout of PHP files exactly the way Pint does. It never changes what the code does. For
any project, `pun check --fix` writes the same bytes as `pint`; `pun format` alone runs only the safe subset (see
"Risky rules" below).

Measured (2-core box, profiling build, reports/scoreboard.md):

| Run | Pint | Pun cold | Pun warm |
| --- | --- | --- | --- |
| laravel/framework, its own pint.json (3,027 files) | 190 s | 3.7 s | 0.07 s |
| Pinkary, PHP only | | 0.33 s | 0.03 s |
| Pinkary with Blade | 13.4 s | 4.2 s | 0.2 s |

Parity: 0 differing files on all 7 corpus projects, with the laravel and psr12 presets, plus a mutation fuzzer
(including CRLF and mixed line endings) at 0 differences.

## 2. How it works

Pint is a wrapper around PHP-CS-Fixer, which is a token fixer: it doesn't reprint the file, it edits only the tokens a
rule objects to. A pretty-printer like Mago's can never match that byte for byte, so Pun ports PHP-CS-Fixer itself.

For each file:

1. **Lex** into PHP tokens identical to PHP's `token_get_all` (own lexer in pun-tokens, checked against PHP).
2. **Transform**: retag tokens into PHP-CS-Fixer's custom kinds (array brackets, nullable `?`, union `|`, named-argument
   colons, attribute brackets and so on).
3. **Lint**: check the file parses with PHP's own grammar; files with syntax errors are reported and skipped, as Pint does.
4. **Run the fixers** in PHP-CS-Fixer's order (priority, then name), each exactly once, on the shared token list. No
   repeat-until-stable loop, because upstream has none. Each fixer is a function-by-function port of the upstream PHP.
5. **Print** the tokens back. Unchanged tokens keep their original bytes.

Around that: file discovery copies Pint's finder rules (pun-workspace), files run in parallel, and a content-hash cache
skips files already formatted (keyed on Pun's version, PHP version, rules and Blade lockfiles).

**Delegate mode (being removed, Q7).** Today, if a config turns on a rule Pun hasn't ported, the whole run goes
through real PHP-CS-Fixer in long-lived `php` workers. Decided 2026-10-17: Pun runs no upstream PHP. A rule Pun doesn't
support is named by `pun migrate` as unsupported, and the user is asked to send a PR to punphp/pun.

**Blade.** With the `laravel-blade` rule on, `.blade.php` files go through Prettier and prettier-plugin-blade in Node,
exactly as Pint does it (one long-lived Node worker per thread). It needs Node. Blade is about 2.9 s of Pinkary's 4.2 s
cold run.

## 3. The config

Pun reads only `pun.toml`. `pun migrate` writes it from pint.json (or `.php-cs-fixer.php`); without one, every command
stops with "No pun.toml found. Run `pun migrate`." unless you pass `--preset` or `--no-config`.

```toml
[php]
version = "8.4"                 # the PHP version rules target (default 8.4)

[format]
preset = "laravel"              # Pint's presets: laravel (default), psr12, per, symfony, empty
paths = ["app", "tests"]        # what to format when no paths are given
exclude = ["build"]             # folders to skip (Pint's "exclude")
not-name = ["*.blade.php"]      # file names to skip (Pint's "notName")
not-path = ["app/Legacy.php"]   # paths to skip (Pint's "notPath")

[format.rules]                  # only what differs from the preset
concat-space = { spacing = "one" }
ordered-imports = { sort-algorithm = "alpha" }
array-syntax = false            # turn a preset rule off
```

- A rule's value is `true`, `false`, or a table of its options. Option names are PHP-CS-Fixer's, written in kebab-case
  (`sort-algorithm` for `sort_algorithm`); option values are upstream's unchanged (`"alpha"`, `"one"`).
- `pun migrate` writes `[format.rules]` as a delta to the preset, so the file stays short.
- `pun config` prints the resolved config.

**Rule names.** A format rule's ID is `format.<name>`, where the name is the kebab-case form of the PHP-CS-Fixer name
Pint users already know: `array_syntax` is `format.array-syntax`, `ordered_imports` is `format.ordered-imports`. Inside
`[format.rules]` the `format.` part is implied. Writing the old snake_case name is an error that names the Pun ID, so a
copied rule is never silently ignored. Pint's own rules are `format.laravel-blade` and
`format.phpdoc-type-annotations-only`. All names and their upstream sources are data in
crates/pun-config/rules/mapping.toml.

**Ignore comments.** `// @pun-ignore-file format.single-quote` before the first statement skips that rule for the
file. Format rules can only be ignored per file. `pun migrate` rewrites `// @php-cs-fixer-ignore` to this form.

## 4. Risky rules are not format rules

PHP-CS-Fixer marks some fixers "risky": they can change behaviour (`final_class`, `strict_comparison`,
`declare_strict_types`, `no_alias_functions`). Pint runs them like any other rule. Pun moves all 76 of them to
`pun lint` (decided earlier, RULES.md §1), so `pun format` is always safe on save. Consequences:

- Pint's laravel preset turns on 4 risky rules and Pinkary's pint.json 10 more. `pun migrate` puts them in
  `[lint.rules]` with `fix = "apply"`, so `pun check --fix` still does everything `pint` did.
- So `pun format` alone is **not** Pint-identical on a project that uses risky rules; `pun check --fix` is. The harness
  proves the second.

## 5. Rules: what exists

208 format rules in total (every safe PHP-CS-Fixer fixer plus Pint's two). 144 are ported natively; the other 64 run
through delegate mode.

| Preset | Rules in it | Not ported |
| --- | --- | --- |
| laravel | 126 | 0 |
| psr12 | 52 | 0 |
| per | 63 | 3 (`attribute-block-no-spaces`, `operator-linebreak`, `ordered-types`) |
| symfony | 151 | 20 |

The other unported rules are opt-in only (in no preset): Doctrine annotations, PHPUnit fixers, heredoc and string
rewrites, a few PHPDoc rules.

## 6. Pint's CLI, flag for flag

Every Pint flag works the same and prints the same output (summary, `-v` diffs, error boxes, colours, JSON, txt,
checkstyle, JUnit, XML and GitLab reporters, exit codes): `--test`, `--bail`, `--repair`, `--dirty`, `--diff=<branch>`,
`--config`, `--preset`, `--format`, `--output-to-file`, `--stdin-filename`, `--cache-file`, `-v`, `-q`, `--parallel`.
Pun adds `--staged`, `--since=<ref>`, `--watch`, `--stdin`, `--no-cache`, `--explain` and `--native-only`. Under a coding
agent it prints compact `path rule, rule` lines instead (PLAN.md C4).

## 7. Updated plan

Done, nothing to do: lexer, transformers, all laravel and psr12 fixers, CLI parity, cache, watch, delegate mode
(including `@pun-ignore-file`), migrate from pint.json, harness and fuzzer.

Left, reordered after the first answers:

0. **Speed and memory first** (Q10): items 2 and 3 below come before everything else.
1. **Prove it on Pinkary's CI.** Swap `vendor/bin/pint` for Pun in Pinkary's CI. This was the old v0.1 exit gate and
   is still open.
2. **Speed: fewer allocations per token.** A cold laravel-framework run makes 45M allocations. The plan is an inline
   token content type (98% of tokens are 22 bytes or less) plus three small clone/copy fixes. Land only if cold time
   improves by 5% or more. A first attempt was stopped and lost (it lived outside git).
3. **8-core benchmark in CI** to show laravel-framework under 1 s cold (about 1 s by extrapolation today).
4. ~~Port the per preset's 3 rules~~ Not needed: only laravel, psr12 and empty stay as presets (Q4).
5. **Native Blade formatter** (Blade parser plus Prettier-identical HTML printing), so Blade needs no Node. Large;
   after the first release unless you say otherwise (Q6).
6. ~~Embedded PHP~~ Not needed: no PHP fallback (Q7).

## 8. Questions

My recommendation is marked. Answer by number; one word is enough.

1. **Risky rules in lint.** `pun format` stays safe, and `pun check --fix` is what equals `pint`. Keep that, or make
   `pun format` alone equal Pint (risky rules back in format)? *Recommend: keep.*
2. **Rule names.** Kebab-case of the PHP-CS-Fixer names (`array-syntax`), recognisable to Pint users. Keep, or rename
   format rules to Pun's own vocabulary like lint and analyse were? *Recommend: keep.*
3. **Options.** Upstream option names in kebab-case, values unchanged. Keep, or redesign options too? *Recommend: keep.*
4. **Presets.** laravel and psr12 are fully native; per (3 missing) and symfony (20 missing) go through PHP. Which must
   be fully native for the release: laravel + psr12 only, add per, or all four? *Recommend: laravel + psr12 + per.*
5. **No pun.toml.** Today Pun stops and says "run `pun migrate`". Pint instead falls back to the laravel preset. Keep
   the hard stop, or fall back to laravel like Pint? *Recommend: keep the stop.*
6. **Blade.** Keep Pint's Node + Prettier path for the release and build native Blade after? *Recommend: yes, after.*
7. **Delegate mode needs system PHP.** Acceptable for the release, given laravel and psr12 never delegate?
   *Recommend: yes.*
8. **Pun's own style.** The Mago-based "Pun style" formatter was removed. Do you ever want a Pun style beyond Pint
   presets (for example a `pun` preset), or is Pint parity the only target? *Recommend: Pint parity only for now.*
9. **Human output.** Keep Pint's exact terminal output (boxes, colours, wording) for humans, or give Pun its own
   look while keeping Pint's machine formats? *Recommend: keep Pint's exact output.*
10. **Speed work (items 2-3).** Worth doing before lint and analyse, or park it, since Pun is already about 50x Pint?
    *Recommend: park it until after lint and analyse.*

## Decisions

Answered 2026-10-17 (Pushpak):

1. **Risky rules stay in lint.** `pun format` is behaviour-safe; `pun check --fix` is what equals `pint`.
2. **Rule names stay kebab-case**, but must be made consistent, with an origin-style convention like `laravel/*`,
   `symfony/<kebab-case>`. Exact scheme being clarified (see Q2 follow-up below).
3. **Options are redesigned for Rust/TOML**, not copied from PHP-CS-Fixer. `pun migrate` translates upstream options.
4. **Three presets only: `laravel`, `psr12`, `empty`.** `per` and `symfony` are dropped as presets (their rules stay
   available one by one).
5. **No pun.toml: stop**, with "Run `pun migrate`, or `pun init` for a new project". So `pun init` is new work.
6. **Blade stays on Node + Prettier** for now; native Blade later.
7. **No PHP fallback for unported rules** (answered 2026-10-17). Pun never runs PHP-CS-Fixer or Pint for a rule it
   doesn't support. `pun migrate` names each unsupported rule while migrating, says it is skipped, and asks the user
   to send a PR to punphp/pun:
   ```
   ⚠ phpdoc_tag_casing  not supported by Pun yet; skipped
     Want it? Send a PR to https://github.com/punphp/pun
   ```
   Delegate mode (pun-delegate, `--native-only`, `--explain`'s delegated part) and embedded PHP (pun-php) are removed.
   Custom fixer classes in `.php-cs-fixer.php` are unsupported the same way.
8. **Pun style: open**, explained again in the thread.
9. **Human output: open**, explained again in the thread.
10. **Speed and memory are a priority now**, not parked.

Second round (Pushpak, 2026-10-17):

- **Q2: grouped names (option b), designed for maintenance and growth.** Approved (third round, below).
- **Q8: a Pun style is wanted in the future**, as its own preset next to Pint's. Not now.
- **Q9: Pun gets its own modern, coloured terminal output**, modelled on today's Rust tools (oxfmt, Biome, Ruff).
  Approved (§10). Machine formats (`--format=json`, checkstyle, junit, gitlab) stay as they are.

Third round (Pushpak, 2026-10-17):

- **Names approved:** every rule is `group/name`, full ID `format/group/name`; the full table is in §9.
- **No aliases.** Breaking renames are fine until the release.
- **Big runs use parallel processing.** Already true (files are formatted in parallel, one worker per core); it stays
  that way and is measured by the 8-core benchmark (§7).

### Follow-ups these create

- Q3: design each rule's options as TOML (names, types, enums); the old PHP-CS-Fixer options become migrate input only.
- Q4: remove `per` and `symfony` from `--preset` and pun.toml; migrate expands a pint.json using them into explicit rules.
- Q5: add `pun init` (writes a starter pun.toml for a new project) and the new error text.
- Q2: rename every format rule to `group/name` per the §9 table (mapping.toml, pun-config, fixtures, docs), no aliases.
- Q9: new human reporter (§10); scripts/cli-parity.sh and tests/output.rs then cover only the machine formats.
- Q8: a future `preset = "pun"` (whole-file reprint at a line width); park until after 1.0.
- Q10: the allocation work (§7 item 2) and the 8-core benchmark move up, with a memory target next to the speed one.


## 9. Grouped rule names (Q2, approved)

**Shape.** Every rule is `<group>/<name>`, and its full ID is `<command>/<group>/<name>`:
`format/imports/ordered`, `format/phpdoc/align`, `format/laravel/blade`. Biome uses the same idea (`lint/style/useConst`).

**Rules that keep it maintainable:**

1. **Groups are a fixed list in mapping.toml**, each with a one-line description. Adding a group is a deliberate change;
   `scripts/check-rule-mapping.py` rejects a rule whose group isn't in the list.
2. **Two kinds of group.** *Topic* groups say what part of the code a rule touches. *Ecosystem* groups hold rules that
   only make sense for one framework or tool: `laravel`, `phpunit`, `pest`, `doctrine`. A rule goes in an ecosystem group
   when it's useless without that ecosystem, otherwise in a topic group. That gives `laravel/*` and `symfony`-style
   groups their own place without mixing them into the topics.
3. **The same group names in all three commands**, so `imports` means the same thing in format, lint and analyse
   (`format/imports/ordered`, `lint/imports/no-unused`, `analyse/imports/undefined-class`).
4. **The name doesn't repeat the group** (`imports/ordered`, not `imports/ordered-imports`), is kebab-case, and is
   unique within its group. Format names describe the construct (`arrays/syntax`); `no-` only when the rule removes
   something (`whitespace/no-trailing`).
5. **No aliases.** Nothing is released yet, so renames are breaking changes and that's fine until 1.0. Only one name
   exists per rule; `pun migrate` maps the upstream (Pint / PHP-CS-Fixer) names onto it.

**Format groups** (13 today, cleaned up): `arrays`, `attributes`, `casing`, `casts`, `classes`, `comments`,
`control-flow`, `functions`, `imports`, `operators`, `phpdoc`, `statements`, `strings`, `tags`, `types`, `whitespace`,
plus the ecosystem groups `laravel`, `phpunit`, `doctrine`. The vague `syntax` group (43 rules) is split up among
these.

**Examples:**

| Today | Proposed |
| --- | --- |
| `ordered-imports` | `imports/ordered` |
| `no-unused-imports` | `imports/no-unused` |
| `phpdoc-align` | `phpdoc/align` |
| `array-syntax` | `arrays/syntax` |
| `no-trailing-whitespace` | `whitespace/no-trailing` |
| `lowercase-keywords` | `casing/keywords` |
| `single-quote` | `strings/single-quote` |
| `yoda-style` | `control-flow/yoda` |
| `cast-spaces` | `casts/spacing` |
| `laravel-blade` | `laravel/blade` |
| `phpdoc-type-annotations-only` (Pint's own) | `laravel/phpdoc-type-annotations-only` |
| `phpunit-method-casing` | `phpunit/method-casing` |
| `doctrine-annotation-braces` | `doctrine/annotation-braces` |

**In pun.toml**, TOML's dotted keys make the group read naturally, and a whole group can be switched off:

```toml
[format.rules]
imports.ordered = { sort = "alphabetical" }
phpdoc.align = false
arrays.syntax = "short"          # rules with one option take the value directly

[format.groups]
doctrine = false                 # every rule in the group
```

**In ignore comments and output**, the full ID: `// @pun-ignore-file format/strings/single-quote`, and
`app/User.php  imports/ordered, imports/no-unused` in a report.

**All 208 format rules.** The full rename, grouped. "laravel" and "psr12" mark the presets that turn a rule on;
"ported" means it runs natively in Rust (the rest go through system PHP in delegate mode). Rules in neither preset stay
available one by one. Generated from crates/pun-config/rules/mapping.toml; the code rename lands in mapping.toml with
exactly these names.

| New name | Today | laravel | psr12 | ported |
| --- | --- | :-: | :-: | :-: |
| **arrays** (12) | | | | |
| `arrays/indentation` | `array-indentation` | ✓ |  | ✓ |
| `arrays/index-brace` | `normalize-index-brace` | ✓ |  | ✓ |
| `arrays/list-syntax` | `list-syntax` | ✓ |  | ✓ |
| `arrays/no-multiline-double-arrow` | `no-multiline-whitespace-around-double-arrow` | ✓ |  | ✓ |
| `arrays/no-space-around-offset` | `no-spaces-around-offset` | ✓ |  | ✓ |
| `arrays/no-space-before-comma` | `no-whitespace-before-comma-in-array` | ✓ |  | ✓ |
| `arrays/no-space-in-empty` | `no-whitespace-in-empty-array` |  |  |  |
| `arrays/no-trailing-comma-single-line` | `no-trailing-comma-in-singleline` | ✓ |  | ✓ |
| `arrays/space-after-comma` | `whitespace-after-comma-in-array` | ✓ |  | ✓ |
| `arrays/syntax` | `array-syntax` | ✓ |  | ✓ |
| `arrays/trailing-comma-multiline` | `trailing-comma-in-multiline` | ✓ |  | ✓ |
| `arrays/trim-spaces` | `trim-array-spaces` | ✓ |  | ✓ |
| **attributes** (4) | | | | |
| `attributes/empty-parentheses` | `attribute-empty-parentheses` |  |  |  |
| `attributes/no-block-spaces` | `attribute-block-no-spaces` |  |  |  |
| `attributes/ordered` | `ordered-attributes` |  |  |  |
| `attributes/remove` | `general-attribute-remove` |  |  |  |
| **casing** (9) | | | | |
| `casing/class-reference` | `class-reference-name-casing` |  |  | ✓ |
| `casing/constants` | `constant-case` | ✓ | ✓ | ✓ |
| `casing/integer-literals` | `integer-literal-case` | ✓ |  | ✓ |
| `casing/keywords` | `lowercase-keywords` | ✓ | ✓ | ✓ |
| `casing/magic-constants` | `magic-constant-casing` | ✓ |  | ✓ |
| `casing/magic-methods` | `magic-method-casing` | ✓ |  | ✓ |
| `casing/native-functions` | `native-function-casing` | ✓ |  | ✓ |
| `casing/native-types` | `native-type-declaration-casing` | ✓ |  | ✓ |
| `casing/static-reference` | `lowercase-static-reference` | ✓ | ✓ | ✓ |
| **casts** (5) | | | | |
| `casts/lowercase` | `lowercase-cast` | ✓ | ✓ | ✓ |
| `casts/no-short-bool` | `no-short-bool-cast` | ✓ |  | ✓ |
| `casts/no-unset` | `no-unset-cast` | ✓ |  | ✓ |
| `casts/short-scalar` | `short-scalar-cast` | ✓ | ✓ | ✓ |
| `casts/spacing` | `cast-spaces` | ✓ |  | ✓ |
| **classes** (14) | | | | |
| `classes/definition` | `class-definition` | ✓ | ✓ | ✓ |
| `classes/member-separation` | `class-attributes-separation` | ✓ |  | ✓ |
| `classes/modifier-keywords` | `modifier-keywords` | ✓ | ✓ | ✓ |
| `classes/multiline-promoted-properties` | `multiline-promoted-properties` |  |  |  |
| `classes/no-blank-line-after-opening` | `no-blank-lines-after-class-opening` | ✓ | ✓ | ✓ |
| `classes/no-null-property-default` | `no-null-property-initialization` |  |  |  |
| `classes/no-redundant-readonly` | `no-redundant-readonly-property` |  |  |  |
| `classes/ordered-interfaces` | `ordered-interfaces` | ✓ |  | ✓ |
| `classes/ordered-members` | `ordered-class-elements` |  | ✓ | ✓ |
| `classes/protected-to-private` | `protected-to-private` |  |  | ✓ |
| `classes/self-static-accessor` | `self-static-accessor` | ✓ |  | ✓ |
| `classes/single-member-per-statement` | `single-class-element-per-statement` | ✓ | ✓ | ✓ |
| `classes/single-trait-per-statement` | `single-trait-insert-per-statement` |  | ✓ | ✓ |
| `classes/stringable-for-to-string` | `stringable-for-to-string` |  |  |  |
| **comments** (7) | | | | |
| `comments/align-multiline` | `align-multiline-comment` |  |  | ✓ |
| `comments/header` | `header-comment` |  |  |  |
| `comments/multiline-delimiters` | `multiline-comment-opening-closing` |  |  |  |
| `comments/no-empty` | `no-empty-comment` |  |  |  |
| `comments/no-trailing-whitespace` | `no-trailing-whitespace-in-comment` | ✓ | ✓ | ✓ |
| `comments/single-line-spacing` | `single-line-comment-spacing` | ✓ |  | ✓ |
| `comments/single-line-style` | `single-line-comment-style` | ✓ |  | ✓ |
| **control-flow** (23) | | | | |
| `control-flow/braces` | `control-structure-braces` | ✓ | ✓ | ✓ |
| `control-flow/combine-issets` | `combine-consecutive-issets` |  |  | ✓ |
| `control-flow/combine-unsets` | `combine-consecutive-unsets` |  |  | ✓ |
| `control-flow/continuation-position` | `control-structure-continuation-position` | ✓ | ✓ | ✓ |
| `control-flow/elseif` | `elseif` | ✓ | ✓ | ✓ |
| `control-flow/empty-loop-body` | `empty-loop-body` |  |  |  |
| `control-flow/empty-loop-condition` | `empty-loop-condition` |  |  |  |
| `control-flow/include` | `include` | ✓ |  | ✓ |
| `control-flow/no-alternative-syntax` | `no-alternative-syntax` | ✓ |  | ✓ |
| `control-flow/no-break-comment` | `no-break-comment` |  | ✓ | ✓ |
| `control-flow/no-return-assignment` | `return-assignment` |  |  |  |
| `control-flow/no-superfluous-elseif` | `no-superfluous-elseif` |  |  | ✓ |
| `control-flow/no-unneeded-braces` | `no-unneeded-braces` | ✓ |  | ✓ |
| `control-flow/no-unneeded-parentheses` | `no-unneeded-control-parentheses` | ✓ |  | ✓ |
| `control-flow/no-useless-else` | `no-useless-else` |  |  | ✓ |
| `control-flow/no-useless-return` | `no-useless-return` | ✓ |  | ✓ |
| `control-flow/simplified-if-return` | `simplified-if-return` |  |  |  |
| `control-flow/simplified-null-return` | `simplified-null-return` |  |  |  |
| `control-flow/switch-case-colon` | `switch-case-semicolon-to-colon` | ✓ | ✓ | ✓ |
| `control-flow/switch-case-space` | `switch-case-space` | ✓ | ✓ | ✓ |
| `control-flow/switch-continue-to-break` | `switch-continue-to-break` |  |  | ✓ |
| `control-flow/yield-from` | `return-to-yield-from` |  |  |  |
| `control-flow/yoda` | `yoda-style` | ✓ |  | ✓ |
| **functions** (9) | | | | |
| `functions/argument-spacing` | `method-argument-space` | ✓ | ✓ | ✓ |
| `functions/backtick-to-shell-exec` | `backtick-to-shell-exec` |  |  | ✓ |
| `functions/chain-indentation` | `method-chaining-indentation` | ✓ |  | ✓ |
| `functions/declaration` | `function-declaration` | ✓ | ✓ | ✓ |
| `functions/no-alias-construct-call` | `no-alias-language-construct-call` | ✓ |  | ✓ |
| `functions/no-space-after-name` | `no-spaces-after-function-name` | ✓ | ✓ | ✓ |
| `functions/no-unused-closure-use` | `lambda-not-used-import` | ✓ |  | ✓ |
| `functions/return-type-spacing` | `return-type-declaration` | ✓ | ✓ | ✓ |
| `functions/single-line-throw` | `single-line-throw` |  |  |  |
| **imports** (14) | | | | |
| `imports/blank-line-after` | `single-line-after-imports` | ✓ | ✓ | ✓ |
| `imports/blank-line-after-namespace` | `blank-line-after-namespace` | ✓ | ✓ | ✓ |
| `imports/blank-line-between-groups` | `blank-line-between-import-groups` | ✓ | ✓ | ✓ |
| `imports/blank-lines-before-namespace` | `blank-lines-before-namespace` | ✓ | ✓ | ✓ |
| `imports/clean-namespace` | `clean-namespace` | ✓ |  | ✓ |
| `imports/fully-qualified-strict-types` | `fully-qualified-strict-types` | ✓ |  | ✓ |
| `imports/global-namespace` | `global-namespace-import` |  |  | ✓ |
| `imports/group` | `group-import` |  |  |  |
| `imports/no-leading-namespace-whitespace` | `no-leading-namespace-whitespace` | ✓ |  | ✓ |
| `imports/no-leading-slash` | `no-leading-import-slash` | ✓ | ✓ | ✓ |
| `imports/no-unneeded-alias` | `no-unneeded-import-alias` | ✓ |  | ✓ |
| `imports/no-unused` | `no-unused-imports` | ✓ | ✓ | ✓ |
| `imports/ordered` | `ordered-imports` | ✓ | ✓ | ✓ |
| `imports/single-per-statement` | `single-import-per-statement` | ✓ | ✓ | ✓ |
| **operators** (18) | | | | |
| `operators/binary-spacing` | `binary-operator-spaces` | ✓ | ✓ | ✓ |
| `operators/coalesce-equal` | `assign-null-coalescing-to-coalesce-equal` |  |  |  |
| `operators/concat-spacing` | `concat-space` | ✓ |  | ✓ |
| `operators/increment-style` | `increment-style` | ✓ |  | ✓ |
| `operators/linebreak` | `operator-linebreak` |  |  |  |
| `operators/new-expression-parentheses` | `new-expression-parentheses` |  |  |  |
| `operators/new-parentheses` | `new-with-parentheses` | ✓ | ✓ | ✓ |
| `operators/no-space-around-arrow` | `object-operator-without-whitespace` | ✓ |  | ✓ |
| `operators/no-space-around-double-colon` | `no-space-around-double-colon` | ✓ | ✓ | ✓ |
| `operators/no-useless-concat` | `no-useless-concat-operator` |  |  |  |
| `operators/no-useless-nullsafe` | `no-useless-nullsafe-operator` |  |  |  |
| `operators/not-with-space` | `not-operator-with-space` |  |  |  |
| `operators/not-with-successor-space` | `not-operator-with-successor-space` | ✓ |  | ✓ |
| `operators/standardize-increment` | `standardize-increment` |  |  |  |
| `operators/standardize-not-equals` | `standardize-not-equals` | ✓ |  | ✓ |
| `operators/ternary-spacing` | `ternary-operator-spaces` | ✓ | ✓ | ✓ |
| `operators/ternary-to-coalesce` | `ternary-to-null-coalescing` |  |  |  |
| `operators/unary-spacing` | `unary-operator-spaces` | ✓ | ✓ | ✓ |
| **phpdoc** (35) | | | | |
| `phpdoc/add-missing-param` | `phpdoc-add-missing-param-annotation` |  |  |  |
| `phpdoc/align` | `phpdoc-align` | ✓ |  | ✓ |
| `phpdoc/annotation-without-dot` | `phpdoc-annotation-without-dot` |  |  |  |
| `phpdoc/indent` | `phpdoc-indent` | ✓ |  | ✓ |
| `phpdoc/inline-tag` | `phpdoc-inline-tag-normalizer` | ✓ |  | ✓ |
| `phpdoc/line-span` | `phpdoc-line-span` |  |  |  |
| `phpdoc/no-access` | `phpdoc-no-access` | ✓ |  | ✓ |
| `phpdoc/no-alias-tag` | `phpdoc-no-alias-tag` |  |  |  |
| `phpdoc/no-blank-line-after` | `no-blank-lines-after-phpdoc` | ✓ |  | ✓ |
| `phpdoc/no-duplicate-types` | `phpdoc-no-duplicate-types` |  |  |  |
| `phpdoc/no-empty` | `no-empty-phpdoc` | ✓ |  | ✓ |
| `phpdoc/no-empty-return` | `phpdoc-no-empty-return` |  |  |  |
| `phpdoc/no-package` | `phpdoc-no-package` | ✓ |  | ✓ |
| `phpdoc/no-superfluous-tags` | `no-superfluous-phpdoc-tags` | ✓ |  | ✓ |
| `phpdoc/no-useless-inheritdoc` | `phpdoc-no-useless-inheritdoc` | ✓ |  | ✓ |
| `phpdoc/order` | `phpdoc-order` | ✓ |  | ✓ |
| `phpdoc/order-by-value` | `phpdoc-order-by-value` |  |  |  |
| `phpdoc/param-order` | `phpdoc-param-order` |  |  |  |
| `phpdoc/remove-annotation` | `general-phpdoc-annotation-remove` |  |  |  |
| `phpdoc/rename-tag` | `general-phpdoc-tag-rename` | ✓ |  | ✓ |
| `phpdoc/return-self-reference` | `phpdoc-return-self-reference` |  |  | ✓ |
| `phpdoc/scalar` | `phpdoc-scalar` | ✓ |  | ✓ |
| `phpdoc/separation` | `phpdoc-separation` | ✓ |  | ✓ |
| `phpdoc/single-line-var-spacing` | `phpdoc-single-line-var-spacing` | ✓ |  | ✓ |
| `phpdoc/summary` | `phpdoc-summary` |  |  | ✓ |
| `phpdoc/tag-casing` | `phpdoc-tag-casing` |  |  |  |
| `phpdoc/tag-no-named-arguments` | `phpdoc-tag-no-named-arguments` |  |  |  |
| `phpdoc/tag-type` | `phpdoc-tag-type` | ✓ |  | ✓ |
| `phpdoc/to-comment` | `phpdoc-to-comment` |  |  |  |
| `phpdoc/trim` | `phpdoc-trim` | ✓ |  | ✓ |
| `phpdoc/trim-blank-lines` | `phpdoc-trim-consecutive-blank-line-separation` |  |  | ✓ |
| `phpdoc/types` | `phpdoc-types` | ✓ |  | ✓ |
| `phpdoc/types-order` | `phpdoc-types-order` |  |  |  |
| `phpdoc/var-order` | `phpdoc-var-annotation-correct-order` |  |  |  |
| `phpdoc/var-without-name` | `phpdoc-var-without-name` | ✓ |  | ✓ |
| **statements** (16) | | | | |
| `statements/blank-line-before` | `blank-line-before-statement` | ✓ |  | ✓ |
| `statements/braces-position` | `braces-position` | ✓ | ✓ | ✓ |
| `statements/declare-equal` | `declare-equal-normalize` | ✓ | ✓ | ✓ |
| `statements/declare-parentheses` | `declare-parentheses` | ✓ |  | ✓ |
| `statements/explicit-indirect-variable` | `explicit-indirect-variable` |  |  |  |
| `statements/multiline-semicolon` | `multiline-whitespace-before-semicolons` | ✓ |  | ✓ |
| `statements/no-empty` | `no-empty-statement` | ✓ |  | ✓ |
| `statements/no-mixed-echo-print` | `no-mixed-echo-print` | ✓ |  | ✓ |
| `statements/no-space-before-semicolon` | `no-singleline-whitespace-before-semicolons` | ✓ |  | ✓ |
| `statements/numeric-literal-separator` | `numeric-literal-separator` |  |  |  |
| `statements/octal-notation` | `octal-notation` |  |  |  |
| `statements/one-per-line` | `no-multiple-statements-per-line` | ✓ | ✓ | ✓ |
| `statements/semicolon-after-instruction` | `semicolon-after-instruction` |  |  |  |
| `statements/single-line-empty-body` | `single-line-empty-body` | ✓ |  | ✓ |
| `statements/space-after-semicolon` | `space-after-semicolon` | ✓ |  | ✓ |
| `statements/space-around-construct` | `single-space-around-construct` | ✓ | ✓ | ✓ |
| **strings** (9) | | | | |
| `strings/complex-variable` | `simple-to-complex-string-variable` |  |  |  |
| `strings/explicit-variable` | `explicit-string-variable` |  |  |  |
| `strings/heredoc-closing-marker` | `heredoc-closing-marker` |  |  |  |
| `strings/heredoc-indentation` | `heredoc-indentation` |  |  |  |
| `strings/heredoc-to-nowdoc` | `heredoc-to-nowdoc` | ✓ |  | ✓ |
| `strings/implicit-backslashes` | `string-implicit-backslashes` |  |  |  |
| `strings/multiline-to-heredoc` | `multiline-string-to-heredoc` |  |  |  |
| `strings/no-binary` | `no-binary-string` | ✓ |  | ✓ |
| `strings/single-quote` | `single-quote` | ✓ |  | ✓ |
| **tags** (5) | | | | |
| `tags/blank-line-after-opening` | `blank-line-after-opening-tag` | ✓ | ✓ | ✓ |
| `tags/echo-syntax` | `echo-tag-syntax` |  |  |  |
| `tags/full-opening` | `full-opening-tag` | ✓ | ✓ | ✓ |
| `tags/linebreak-after-opening` | `linebreak-after-opening-tag` | ✓ |  | ✓ |
| `tags/no-closing` | `no-closing-tag` | ✓ | ✓ | ✓ |
| **types** (6) | | | | |
| `types/compact-nullable` | `compact-nullable-type-declaration` | ✓ | ✓ | ✓ |
| `types/declaration-spacing` | `type-declaration-spaces` | ✓ |  | ✓ |
| `types/nullable-for-null-default` | `nullable-type-declaration-for-default-null-value` | ✓ |  | ✓ |
| `types/nullable-syntax` | `nullable-type-declaration` | ✓ |  | ✓ |
| `types/ordered` | `ordered-types` |  |  |  |
| `types/union-spacing` | `types-spaces` | ✓ |  | ✓ |
| **whitespace** (9) | | | | |
| `whitespace/encoding` | `encoding` | ✓ | ✓ | ✓ |
| `whitespace/final-newline` | `single-blank-line-at-eof` | ✓ | ✓ | ✓ |
| `whitespace/indentation-type` | `indentation-type` | ✓ | ✓ | ✓ |
| `whitespace/inside-parentheses` | `spaces-inside-parentheses` | ✓ | ✓ | ✓ |
| `whitespace/line-ending` | `line-ending` | ✓ | ✓ | ✓ |
| `whitespace/no-blank-line-whitespace` | `no-whitespace-in-blank-line` | ✓ | ✓ | ✓ |
| `whitespace/no-extra-blank-lines` | `no-extra-blank-lines` | ✓ | ✓ | ✓ |
| `whitespace/no-trailing` | `no-trailing-whitespace` | ✓ | ✓ | ✓ |
| `whitespace/statement-indentation` | `statement-indentation` | ✓ | ✓ | ✓ |
| **laravel** (2) | | | | |
| `laravel/blade` | `laravel-blade` |  |  | ✓ |
| `laravel/phpdoc-type-annotations-only` | `phpdoc-type-annotations-only` |  |  | ✓ |
| **phpunit** (7) | | | | |
| `phpunit/attributes` | `phpunit-attributes` |  |  |  |
| `phpunit/data-provider-order` | `phpunit-data-provider-method-order` |  |  |  |
| `phpunit/fqcn-annotation` | `phpunit-fqcn-annotation` |  |  |  |
| `phpunit/internal-class` | `phpunit-internal-class` |  |  |  |
| `phpunit/method-casing` | `phpunit-method-casing` | ✓ |  | ✓ |
| `phpunit/require-covers` | `phpunit-test-class-requires-covers` |  |  |  |
| `phpunit/size-class` | `phpunit-size-class` |  |  |  |
| **doctrine** (4) | | | | |
| `doctrine/annotation-array-assignment` | `doctrine-annotation-array-assignment` |  |  |  |
| `doctrine/annotation-braces` | `doctrine-annotation-braces` |  |  |  |
| `doctrine/annotation-indentation` | `doctrine-annotation-indentation` |  |  |  |
| `doctrine/annotation-spaces` | `doctrine-annotation-spaces` |  |  |  |


## 10. Pun's own terminal output (Q9, approved)

Today `pun format` prints Pint's screen exactly. The proposal replaces that for humans with the style of current Rust
tools: Biome and oxfmt (summary line with a time, coloured diffs, code frames for errors) and Ruff (one line per file,
an exact count at the end). This section is my design, based on how those tools behave; I haven't copied any of their
output byte for byte.

**`pun format`** (fixes):
```
 ✔ app/Models/User.php      imports/ordered, imports/no-unused
 ✔ app/Http/Kernel.php      whitespace/no-trailing
Formatted 2 files, 482 unchanged · 484 files in 0.21s
```

**`pun format --test`** (check only, exits 1):
```
 ✖ app/Models/User.php      imports/ordered
 ✖ app/Http/Kernel.php      whitespace/no-trailing
2 of 484 files need formatting · run `pun format` to fix · 0.18s
```

**`-v`** adds a coloured diff under each file, with line numbers and a few lines of context (green added, red removed,
dim context). **Syntax errors** get a code frame like rustc and Biome:
```
 ✖ app/Broken.php:12:1  syntax error, unexpected token "}"
    11 │     return $x
  > 12 │ }
       │ ^
```

**Rules for the look:** green ✔ for fixed, red ✖ for needs-fixing or errors, yellow for warnings, dim grey for
unchanged and timings; rule IDs in a fixed column; one summary line that always has the count and the time; nothing at
all for unchanged files unless `-v`. Colour is automatic on a terminal and off when piped, with `NO_COLOR` and
`--color=always|never`. Under a coding agent the compact agent format is unchanged.

**What stays Pint-compatible:** exit codes, and the machine formats `--format=json|checkstyle|junit|gitlab|xml|txt`, so
CI integrations keep working.

**Open detail:** keep a progress indicator for big runs (Pint shows dots)? *Recommend: a single spinner line with a
file counter, only on a terminal and only when the run takes over a second.*
