# Multi-agent build

How several AI agents build Pun at the same time without breaking each other's work. Every agent reads this
file before it starts. It adds to AGENTS.md; it does not replace it.

The goal is the whole product: `pun format`, `pun lint`, `pun analyse`, `pun check`, `pun migrate` and
`pun init` complete, byte-compatible with the pinned upstream tools, fast and lean on memory. Do not stop to
ask. When a decision is genuinely Pushpak's, add a `needs Pushpak` row to context/PLAN.md §3, pick the
option closest to upstream behaviour, and keep going.

## Where agents run

- **Mac agents** run in one shared checkout, /Users/pushpakchhajed/Projects/pun, on `main`. A coordinator
  session checks on them hourly. Machine-local tooling lives in ~/Projects/pun-wf (not in git): the `heavy`
  build lock, corpus scratch copies with `vendor/`, per-agent tmp dirs.
- **Server agents** run on a separate server, each in its own clone of git@github.com:punphp/pun.git.
  They have no ~/Projects/pun-wf; see "Server setup" below.

Both push straight to origin/main. Nobody force-pushes, rebases shared history or opens long-lived branches.

## Ownership

One owner per area. Claim before you start: add one row to context/PLAN.md §3 under the matching step,
`| <scope> | doing (claimed by <name>) |`, commit and push it alone. Skip anything another row claims.
Need a change in a file someone else owns? Add a `| ... | request |` row and keep going.

| Agent | Where | Owns |
|---|---|---|
| lint-a | Mac | lint groups dead-code, complexity, control-flow, operators, strings, arrays, statements, comments, tags, refactor; pun-lint's shared helpers (syntax.rs, rules/php_parser.rs, used_variable.rs, rector_*.rs) |
| lint-b | Mac | lint groups style, classes, functions, phpdoc, security, php-8.4/8.5/8.6 |
| lint-c | Mac | lint groups types and laravel (minus anything claimed by a helper) |
| analyse-a | Mac | analyse groups classes, undefined, functions, attributes, deprecated, statements, control-flow, casing, imports, comments, debug; pun-analyse lib.rs, codebase.rs, rules/mod.rs |
| analyse-b | Mac | analyse groups types, phpdoc, operators, arrays, casts, strings, dead-code, laravel; crates/pun-phpstan; the Mago-to-PHPStan type layer |
| format | Mac | crates/pun-tokens, pun-format, pun-fixers, pun-harness |
| platform | Mac | crates/pun-cli (not migrate.rs/init.rs), pun-workspace, pun-problem, pun-lint lib.rs/walk.rs/examples, scripts/cli-parity.sh, bench.py, coverage.sh, root Cargo.toml |
| config | Mac | crates/pun-config, pun-cli migrate.rs/init.rs, scripts/check-rule-mapping.py |

### Helper queue

Helpers (any extra agent, on the Mac or a server) pick their own work from this queue, top first. A slice is
taken when a PLAN.md §3 row says `doing (claimed by <name>)` for it; the original owner then skips it.

1. lint `laravel` group (rector-laravel rules)
2. analyse `dead-code` group
3. analyse `laravel` group (Larastan rules and extensions)
4. analyse `undefined` and `functions` groups
5. analyse `attributes` and `deprecated` groups
6. analyse `types` group, rule IDs n-z
7. lint `types` group, rule IDs n-z
8. lint `style` group, rule IDs n-z
9. lint `dead-code` group, rule IDs n-z
10. analyse `phpdoc` and `operators` groups
11. perf: scripts/bench.py in all modes; improve speed and memory without changing output; results in
    context/reports/scoreboard.md
12. When the queue is used up: take the group with the most unported rules (count `ported` in
    crates/pun-config/rules/mapping.toml) and claim a range of its rule IDs that nobody has claimed.

### Claiming (race-safe)

1. `git fetch origin` and merge (or fast-forward) origin/main; read the claim rows in context/PLAN.md §3.
2. Pick the first slice in the queue with no claim row. Choose a name: `helper-<host>-<4 random letters>`.
3. Add one row under the matching step: `| <slice> | doing (claimed by <name>) |`. Commit only
   context/PLAN.md and push.
4. If the push is rejected, fetch and merge origin/main, then look again: if someone else's row now claims
   the same slice, delete your row, commit, push, and go back to step 2. Otherwise push again.
5. While you work, update the row with counts. When the slice is done, mark the row done (or delete it),
   push, and claim the next slice the same way.

## Shared files

context/PLAN.md, context/AUDIT.md, crates/pun-config/rules/mapping.toml and each crate's rule registry
(`rules/mod.rs`) are edited by many agents. Change only your own rows or lines, with targeted single-line
edits; never rewrite or reformat the whole file. Put new rules in new files and your helpers in a new file
of your own.

## Git

- Mac agents share one working tree: never run `git add -A`, `git add .`, `git stash`, `git reset`,
  `git checkout -- <path>`, `git restore`, `git clean`, `git rebase` or `git pull` there. Those commands
  touch other agents' uncommitted files.
- Commit only your own paths in one step: `git add <new files you created>`, then
  `git commit -m "type(scope): subject" -m "Claude-Session: https://claude.ai/code/session_01NySXSoVyfV7bfuu5pBqpiQ" -- <your paths>`.
  If `.git/index.lock` exists, wait a few seconds and retry.
- Push after every finished batch: `git push origin main`. If it is rejected because someone else pushed:
  `git fetch origin && git merge --no-edit origin/main && git push origin main`. Never rebase, never force.
  `git merge` refuses safely when it would touch a file with uncommitted edits; if it refuses or conflicts,
  `git merge --abort` (only if a merge is in progress), wait a few minutes and retry.
- Conventional Commits, lowercase subject, no trailing period. Author Pushpak Chhajed
  <pushpak1300@gmail.com>. No Co-authored-by.

## Builds

- On the Mac, prefix every cargo, php, composer, node and corpus command with `~/Projects/pun-wf/bin/heavy`.
  It takes one of 3 global build slots, sets CARGO_TARGET_DIR and pins rustc 1.99.1. Never set
  CARGO_TARGET_DIR yourself, never build outside heavy, never run two heavy commands at once.
- On the server, use the toolchain from rust-toolchain / rust-version (1.99) and keep to the server's own
  CPU and memory budget; one cargo command at a time per agent.
- Build and test only your own crate (`-p`). If a crate you do not own fails to compile, another agent is
  mid-edit or just pushed: wait a few minutes, fetch, retry; do not edit it.

## Quality bar

- Port upstream function by function: Rector 2.6.1 and rector-laravel, PHPStan 2.2.7 and Larastan 3.10.0,
  PHP-CS-Fixer 3.95.17, Pint 1.29.3. Oracles: `.oracle/rector-bin`, `.oracle/phpstan`,
  `.oracle/php-cs-fixer`, `php .oracle/pint/builds/pint` (`scripts/oracle.sh` installs them).
- Parity: lint — `format(pun lint --fix) == format(rector process)` with only that rule, on upstream fixtures
  and the corpus. Analyse — same identifiers, messages and lines as PHPStan/Larastan on their rule fixtures
  and the corpus. Format — byte-identical with Pint / PHP-CS-Fixer.
- Expected values in tests come from upstream fixtures or the real tool's output, never from Pun. 100% line
  coverage (`scripts/coverage.sh punphp-<x>`, every few batches). `cargo fmt` your files.
- For each ported rule, set `ported = true` on its line in crates/pun-config/rules/mapping.toml in the same
  commit; keep `python3 scripts/check-rule-mapping.py` passing. Update your PLAN.md row with counts.
- Work in batches of 5-10 rules, most-used first; commit and push after every batch.
- Pushpak approved the recommended option of every decision item in context/AUDIT.md. Tick an item with its
  sha when you fix it.

## Space

Scratch files go in a tmp dir of your own (Mac: ~/Projects/pun-wf/tmp/<name>/) and are deleted when you no
longer need them; so is anything you download. Never delete corpus/, .oracle/, scratch copies, target
directories you did not create, ~/.claude, or anything you did not create.

## Server setup

A server agent, in its own clone:

1. Install rustc 1.99 (rustup), PHP 8.4+ with the usual extensions, composer, node and npm.
2. `scripts/oracle.sh` installs the pinned tools into `.oracle/`; `scripts/corpus.sh <name>` checks out a
   corpus project (corpus/manifest.toml lists them). For analyse and lint comparisons, run
   `SETUP_ONLY=1 BENCH=analyse scripts/bench.py` (and `BENCH=lint`) once to make scratch copies with
   `composer install` under `target-bench/`.
3. `git fetch origin && git merge --ff-only origin/main` before each batch; your clone is yours, so
   `git pull --rebase origin main` before a push is fine there as long as you never force-push.

## Finishing

Keep going until your scope is ported with parity and full coverage. Then mark your claim row done (or
delete it), push, and report what landed (shas), what is left, and any `request` / `needs Pushpak` rows.
