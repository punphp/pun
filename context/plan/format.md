# Plan: format (`pun format`)

Status: **walked through with Pushpak** (2026-10-17); answers in "Decisions". This file is the plan for the formatter:
how it works today, the decided design, and the build list (§7). Decisions with dates: ../PLAN.md §2. Shared schema: config.md; output: output.md.

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

**Delegate mode (today, being removed).** Today, if a config turns on a rule Pun hasn't ported, the whole run goes
through real PHP-CS-Fixer in long-lived `php` workers. Decided: Pun runs no upstream PHP (config.md Build 6). An
unported rule in pun.toml is a config error; `pun migrate` names it, skips it, and asks for a PR to punphp/pun.

**Blade.** With the `laravel-blade` rule on, `.blade.php` files go through Prettier and prettier-plugin-blade in Node,
exactly as Pint does it (one long-lived Node worker per thread). It needs Node. Blade is about 2.9 s of Pinkary's 4.2 s
cold run.

## 3. The config

The schema is config.md §3: `[format] preset` (`laravel`, `psr12`, `empty`), `not-name` / `not-path`, and
`[format.rules]` as a delta to the preset, with `group/name` keys (§8) and TOML-native options. Format rules have no
levels: a diff is a diff. They can only be ignored per file, with `// @pun-ignore-file format/<group>/<name>` before
the first statement (rules.md §4); `pun migrate` rewrites `// @php-cs-fixer-ignore` to this form.

Options use Pun's TOML names (Decisions, "Option names"); `pun migrate` translates PHP-CS-Fixer's.

## 4. Risky rules are not format rules

PHP-CS-Fixer marks some fixers "risky": they can change behaviour (`final_class`, `strict_comparison`,
`declare_strict_types`, `no_alias_functions`). Pint runs them like any other rule. Pun moves all 76 of them to
`pun lint` (rules.md §1), so `pun format` is always safe on save. Consequences:

- Pint's laravel preset turns on 4 risky rules and Pinkary's pint.json 10 more. `pun migrate` puts them in
  `[lint.rules]` at `error`, and `--fix` applies every enabled fix, so `pun check --fix` still does everything `pint` did.
- So `pun format` alone is **not** Pint-identical on a project that uses risky rules; `pun check --fix` is. The harness
  proves the second.

## 5. Rules: what exists

208 format rules in total today (every safe PHP-CS-Fixer fixer plus Pint's two); 40 of them move to lint
(commands.md §4), leaving 168. 144 are ported natively.

| Preset | Rules today | After the 40 move | Not ported |
| --- | ---: | ---: | ---: |
| laravel | 126 | 115 | 0 |
| psr12 | 52 | 50 | 0 |

The unported rules are in no preset: Doctrine annotations, PHPUnit fixers, heredoc and string rewrites, a few PHPDoc
rules, and the rules only `per` and `symfony` used. Until one is ported, `pun migrate` names it and skips it.

## 6. The CLI

Today `pun format` takes every Pint flag (`--test`, `--bail`, `--repair`, `--dirty`, `--diff=<branch>`, `--config`,
`--preset`, `--format`, `--output-to-file`, `--stdin-filename`, `--cache-file`, `-v`, `-q`, `--parallel`) plus
`--staged`, `--since=<ref>`, `--watch`, `--stdin`, `--no-cache` and `--explain`. The flags stay. Decided changes:
`--native-only` goes with delegate mode, the human output becomes Pun's own (§9), and exit codes and the machine
formats (`--format=json|txt|checkstyle|junit|xml|gitlab|github`) stay Pint-compatible. Under a coding agent it prints
compact lines (output.md §3).

## 7. Build

Done for the old design: lexer, transformers, every laravel and psr12 fixer, CLI flags, cache, watch, migrate from
pint.json, harness and fuzzer. Speed and memory come first (decided). The steps after config.md Build 1-6 (new names, the
40 rules moved to lint, TOML options, no delegate mode):

1. **Presets cut to three:** remove `per` and `symfony` from `--preset` and pun.toml (migrate expands them).
   *Done when* `--preset per` is an error naming the replacement, and psr12/laravel corpus runs still match.
2. **The new output** (output.md Build 1-3): file lines, the summary, run errors; Pint's human screen removed; machine
   formats (`--format=json|checkstyle|junit|gitlab|github`) unchanged.
   *Done when* `scripts/cli-parity.sh` passes on exit codes and machine formats, and the human samples in §9 are pinned.
3. **Fewer allocations per token:** the inline `Content` type (24 bytes, 22 inline, else boxed) plus the three clone
   fixes (braces_position and array_indentation token clones, `rhs.to_vec()` in php_parser.rs, per-token lowercasing in
   native_type_declaration_casing). *Done when* cold laravel-framework improves by 5% or more; otherwise drop it.
4. **Memory target** next to the speed one: peak RSS per corpus project in reports/scoreboard.md via `scripts/bench.py`.
5. **8-core benchmark in CI:** laravel-framework under 1 s cold.
6. **Pinkary's CI swap:** `vendor/bin/pint` replaced by Pun. *Done when* Pinkary's CI is green on Pun.
7. **Port the rest of the 208 format rules** that aren't native yet, in usage order, each with upstream fixtures.
   Until a rule is ported, migrate names it as unsupported.

After 1.0: the native Blade formatter (Blade stays on Node + Prettier until then) and a Pun reprinting preset.

## 8. Grouped rule names (approved)

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
"ported" means it runs natively in Rust (unported rules are named and skipped by migrate). Rules in neither preset
stay available one by one. The 40 rules in commands.md §4 leave this table for lint names when they move. Generated from crates/pun-config/rules/mapping.toml; the code rename lands in mapping.toml with
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


## 9. Pun's own terminal output (approved)

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

**Progress:** a single spinner line with a file counter, only on a terminal and only when the run takes over a
second (output.md §1).

## Decisions

Answered 2026-10-17 (Pushpak), over three rounds:

- **Risky rules stay in lint.** `pun format` is behaviour-safe; `pun check --fix` is what equals `pint`.
- **Grouped names:** every rule is `group/name`, full ID `format/group/name`, designed for maintenance and growth; the
  table is §8. **No aliases**: breaking renames are fine until the release.
- **Options are redesigned for TOML**, not copied from PHP-CS-Fixer. `pun migrate` translates upstream options.
- **Three presets only: `laravel`, `psr12`, `empty`.** `per` and `symfony` are dropped as presets (their rules stay
  available one by one; migrate expands a pint.json that uses them).
- **No pun.toml: stop**, with "Run `pun migrate`, or `pun init` for a new project".
- **Blade stays on Node + Prettier** for now; native Blade after 1.0.
- **No PHP fallback for unported rules.** Pun never runs PHP-CS-Fixer or Pint. `pun migrate` names each unsupported
  rule, skips it, and asks for a PR to punphp/pun:
  ```
  ⚠ phpdoc_tag_casing  not supported by Pun yet; skipped
    Want it? Send a PR to https://github.com/punphp/pun
  ```
  Delegate mode (pun-delegate, `--native-only`, `--explain`'s delegated part) and embedded PHP (pun-php) are removed.
  Custom fixer classes in `.php-cs-fixer.php` are unsupported the same way.
- **A Pun style** (`preset = "pun"`, a whole-file reprint at a line width) is wanted, after 1.0.
- **Pun's own terminal output** (§9), modelled on oxfmt, Biome and Ruff. Exit codes and machine formats stay
  Pint-compatible; scripts/cli-parity.sh and tests/output.rs then cover only those.
- **Speed and memory are a priority now**, with a memory target next to the speed one.
- **Big runs stay parallel** (one worker per core), measured by the 8-core benchmark (§7).

**Option names** (config 4, wf/options, 2026-10-17), assistant's calls where "redesigned for TOML" is silent. The data
is `[options."format/<group>/<name>"]` in crates/pun-config/rules/mapping.toml (all 156 options of the 82 fixers
behind format rules, checked against the pinned PHP-CS-Fixer's definitions in crates/pun-config/tests/fixer_options.json,
exported by scripts/export-fixer-options.php); the reader and writer are crates/pun-config/src/options.rs.

- **Kebab-case names and values**, like every other pun.toml key: PHP-CS-Fixer's `snake_case` option names and enum
  values have `_` written as `-`. Values that are PHP keywords (`include_once`, `require_once`) and free values (tag
  names, type names, operators, text) keep their spelling.
- **Few renames, one vocabulary:** `sort_algorithm` and `ordered_interfaces`' `order` become `sort`, with `alpha` as
  `alphabetical` (format.md §8's example); `imports_order` becomes `order`; `null_adjustment` becomes `null-position`
  with `first` / `last`; `ascend` / `descend` become `ascending` / `descending`; header_comment's `PHPDoc` becomes
  `phpdoc`. Everything else keeps PHP-CS-Fixer's word, so its docs still explain the option.
- **`null`, which TOML lacks:** where upstream's `null` means "leave it alone" (binary_operator_spaces, yoda_style,
  global_namespace_import, phpdoc_line_span, doctrine_annotation_spaces), Pun writes `"ignore"`; types_spaces'
  `space_multiple_catch = null` (follow `space`) is `"same-as-space"`. `imports_order` and header_comment's
  `validator`, whose default is `null`, write it by leaving the option out. No other option takes `null`.
- **Shorthand** (format.md §8's `arrays.syntax = "short"`): a rule with exactly one option takes that option's value
  directly, unless the option is a bool (ambiguous with on/off) or a table (ambiguous with the options table).
  `pun migrate` and `pun config` write the shorthand whenever it applies.
- **Errors:** pun.toml errors are Pun's, and name the Pun option when an upstream name is used. `pun migrate` stops on
  options PHP-CS-Fixer itself rejects, with PHP-CS-Fixer's message (`[array_syntax] Invalid configuration: ...`),
  because Pint would stop on them too. PHP's `[]` is read as an empty table for table-valued options.
- Lint rules that are fixers (risky and code-deleting) keep PHP-CS-Fixer's option names, kebab-cased; giving them Pun
  names is a follow-up in lint's plan.
