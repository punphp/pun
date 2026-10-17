# Lint spike (lint.md Build 1)

Date: 2026-10-17. Crate: `crates/pun-lint` (package `punphp-lint`). Upstream: Rector 2.6.1, the version
Pinkary's composer.lock pins (`.oracle/rector`, `.oracle/rector-bin`).

**Verdict: the text-edit decision holds.** Both sample rules pass every Rector fixture, and on 3,376 corpus files Pun's
output is byte-identical to `rector process`, before format runs. No printer was needed.

## What was built

- **The engine** (`pun_lint::Linter`). A `Rule` is called for every node of Mago's syntax tree (parents before
  children) with the node's ancestors. It returns `Report`s: a span, a message, an optional help line and an optional
  `pun_problem::Fix` made of text edits on byte ranges. `check` parses once and runs every rule over that one tree.
  `fix` applies fixes with `pun_problem::apply_fixes` in rounds. A fix that overlaps an earlier one waits for the next
  round. The rounds stop when no fix applies, or after 100 rounds (Ruff's limit). A fix that breaks the syntax leaves
  the file as it was and returns an error. A rule declares its lowest PHP version, as Rector's
  `MinPhpVersionInterface` does.
- **Two rules, both in Pinkary's rector.php sets:**

  | Pun rule | Rector rule | Set | Fix |
  | --- | --- | --- | --- |
  | `lint/php-8.0/no-unused-catch-variable` | `RemoveUnusedVariableInCatchRector` | `withPhpSets()` (php80) | safe |
  | `lint/style/prefer-compound-assignment` | `CombinedAssignRector` | `codeQuality` | unsafe (`$a[f()] = $a[f()] + 1` evaluates `f()` once after the fix) |

  The group parts of these IDs are provisional until config 1 lands the topic groups.
- **Tests.**
  - All 8 of the two rules' Rector fixtures, copied verbatim, pass as `format(lint --fix) == format(expected)` with the
    laravel preset (`tests/rector.rs`). They also pass as `lint --fix == expected` before format.
  - Three oracle files cover what the fixtures miss: property names, `A::$e`, variable variables, `include`, every
    shape of `compact()`, nested and top-level statement lists, comments and parentheses. Their expected output is what
    `rector process` wrote (`tests/oracle/`).
  - The engine tests (`tests/engine.rs`) use hand-derived values.
  - Line coverage is 100% (737 of 737 lines, measured with `-C instrument-coverage` and llvm-cov 1.99.1, because
    `scripts/coverage.sh` does not run on macOS).

## Corpus check

These numbers come from Pinkary and laravel-framework at their manifest commits: 3,376 PHP files, 18.2 + 0.9 MB,
excluding `vendor/` and Blade templates. Each tool got its own copy. Pun ran `examples/fix_dir` with both rules at PHP
8.4. Rector ran `rector process` with the same two rules and `withPhpVersion(80400)`.

- **Rector changed 9 files, and Pun changed the same 9.** `diff -r` between the two copies is empty. The changes were 7
  compound assignments (`+=`, `-=`, `.=`, one on a static property inside `return`) and 2 catch variables.
- **Pinkary needs no fix**, because it already runs Rector.
- **Syntax errors:** both tools refuse the same file, laravel-framework's `tests/Foundation/Fixtures/bad-syntax-strategy.php`.

## Cost per file

These timings come from `cargo run --release -p punphp-lint --example cost -- <dir>`. Each run uses one thread on an
Apple Silicon laptop shared with other agents, and the table shows the second of two runs.

| | Pinkary (347 files) | laravel-framework (3,029 files) |
| --- | ---: | ---: |
| Parse only | 35 µs | 82 µs |
| `check` (parse + 2 rules) | 46 µs | 109 µs |
| `fix` (rounds until settled) | 46 µs | 109 µs |

- **Rules:** the two rules add about 25–30% on top of the parse.
- **Fixing:** `fix` on a file with nothing to fix costs the same as `check` (one parse). A changed file costs one more
  parse per round, and every corpus file settled in one round.
- **Against Rector:** Rector took 197 s wall clock for the same 3,376 files and two rules. Pun took 0.35 s on one
  thread.

## What worked

- **Text edits are enough for both rules.** Deleting a catch variable is one edit, and so is replacing ` = $a + ` with
  ` += `. Mago's spans are byte-exact, so the edits land on the bytes Rector's format-preserving printer would touch.
  That is why even the pre-format output matches.
- **Porting function by function works.** For example, `StmtsManipulator::isVariableUsedInNextStmt`,
  `ExprUsedInNodeAnalyzer`, `CompactFuncCallAnalyzer` and `AssignAndBinaryMap` each map onto a few lines of Rust over
  `Node::visit_children`.
- **`pun_problem` covers the engine's needs as it stands.** `Fix`, `Edit`, `Safety`, `Problem` and `apply_fixes` (with
  its applied and deferred lists) needed no changes.
- **The fix loop needs more than one round.** A nested `try` whose inner and outer catches both name `$e` needs two
  rounds, the same result Rector reaches in one traversal.

## What Mago's tree lacks, compared with php-parser as Rector uses it

- **Mago keeps parentheses that php-parser drops.** Mago has a `Parenthesized` node and php-parser has none. Rules
  must call `syntax::unparenthesize` wherever Rector would see through them. This is easy to forget, and every future
  rule needs it.
- **Mago has no node comparator.** Rector's `NodeComparator::areNodesEqual` compares printed nodes without comments.
  The spike compares the token streams of the two spans and skips whitespace and comments (`syntax::same_code`). This
  matches for every case tested (fixtures, oracle files, corpus), but it is an approximation. Spellings that print the
  same in php-parser but are different tokens (for example two literal forms of one value) would compare unequal here.
  A real port needs a structural comparator shared by all rules.
- **Mago has no parent pointers or `stmts` attribute.** The engine passes the ancestor stack, and
  `syntax::statements` collects a node's statement children. A rule that works on `StmtsAwareInterface` has to find
  its statement list by hand. One example is a try block whose catches count as "next statements".
- **Mago has no name resolution.** `compact` is matched by its written name. That is correct for Rector's
  `isName` on these fixtures, and `Foo\compact` is not `compact` in either tool. Rules that need resolved class names
  will need Mago's name resolver (`mago-names`) or analyse's index.
- **Mago has no removal map.** php-parser's printer strips the whitespace token next to a removed node. Text edits
  must do the same by hand: the catch rule widens its deletion over the whitespace before `$e`. This is the main place
  where "same bytes as Rector before format" takes care, and format covers it anyway.
- **Mago has no scope or type information.** Neither sample rule needs any. Type-aware rules wait for lint.md Build 8.

## Where Pun knowingly differs from Rector

- **Low-precedence operators.** For `$a = $a + ($b or $c)`, Rector prints `$a += $b or $c`. PHP reads that as
  `($a += $b) or $c`, which changes what the code does. Pun keeps the parentheses and writes `$a += ($b or $c)`. No
  Rector fixture covers this case. It is recorded as an assistant's call in lint.md.

## Not done in the spike (lint.md Build 1 items left for "lint 1-2")

- Files in parallel, a cache keyed like format's, and a `pun lint` command.
- Levels: every problem is `Level::Error`.
- A `pun check --fix` pipeline that formats after lint. The tests call `pun_format::Pipeline` directly.
