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

**Delegate mode.** If a config turns on a rule Pun hasn't ported, the whole run goes through real PHP-CS-Fixer in
long-lived `php` workers, so the output is still Pint's. `--explain` shows what ran where; `--native-only` skips the
unported rules. This needs a system `php` and Pint or PHP-CS-Fixer in `vendor/`.

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
6. **Embedded PHP** so delegate mode needs no system PHP (PLAN.md C6). Only matters for unported rules (Q7).

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
7. **System PHP for unported rules: open**, to discuss separately.
8. **Pun style: open**, explained again in the thread.
9. **Human output: open**, explained again in the thread.
10. **Speed and memory are a priority now**, not parked.

### Follow-ups these create

- Q2: decide the naming scheme, then rename in mapping.toml (keeping old names as aliases).
- Q3: design each rule's options as TOML (names, types, enums); the old PHP-CS-Fixer options become migrate input only.
- Q4: remove `per` and `symfony` from `--preset` and pun.toml; migrate expands a pint.json using them into explicit rules.
- Q5: add `pun init` (writes a starter pun.toml for a new project) and the new error text.
- Q10: the allocation work (§7 item 2) and the 8-core benchmark move up, with a memory target next to the speed one.
