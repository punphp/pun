# Component: terminal output for every command

Status: **proposal, under review with Pushpak** (opened 2026-10-17). Pushpak wants one consistent output for format,
lint, analyse, check, migrate and init. The approved starting point is format.md §10 (✔/✖ per file, a summary line
with counts and time, colour, rustc-style code frames). This file extends it to every command. Nothing is built.
Answers go into "Decisions" and DECISIONS.md. Part of PLAN.md C4.

## 1. The shared parts

Every command's human output is built from the same four pieces, so learning one command teaches all of them.

**1. File lines** (format, and `--fix` runs): what happened to each file, rule IDs in a fixed column.
```
 ✔ app/Models/User.php        format/imports/ordered, lint/dead-code/no-unused-private-method
```

**2. Problem blocks** (lint and analyse): one block per problem, oxlint and rustc style, with the code it's about.
```
 ✖ lint/laravel/no-debug-calls  dd() left in the code                        fixable
   ╭─ app/Http/Controllers/PostController.php:42:9
 41│     $post = Post::find($id);
 42│     dd($post);
   │     ━━━━━━━━━
   ╰─ help: remove the dd() call, or use Log::debug()

 ⚠ lint/style/prefer-early-return  return early instead of nesting the if
   ╭─ app/Services/Billing.php:17:5
   …

 ✖ analyse/undefined/method  Call to undefined method App\Models\User::fullName()
   ╭─ app/Http/Controllers/ProfileController.php:23:16
 23│     return $user->fullName();
   │                   ━━━━━━━━
   ╰─ help: did you mean `full_name` (a column on users)?
```

**3. The summary line**: always last, always the same order, only non-zero parts.
```
✖ 3 errors · 1 warning · 2 fixable (run pun check --fix) · 2 files need formatting · 412 files in 0.31s
✔ No problems · 412 files in 0.08s
```

**4. Run errors** (bad config, missing PHP for a delegated rule): one red line, then a hint.
```
✖ pun.toml:7  unknown rule `lint/style/no-short-ternry`
  did you mean `lint/style/no-short-ternary`?
```

**The look:**
- Red ✖ means error. Yellow ⚠ means warning. Green ✔ means fixed or clean. Rule IDs are cyan, and the rule ID links to
  its docs page in terminals that support links.
- Paths are clickable (`path:line:col`).
- Unchanged files and timings are dim.
- ASCII fallbacks (`x`, `!`, `ok`) apply when the terminal can't show Unicode.
- Colour is on for a terminal and off when piped. `NO_COLOR` and `--color=always|never` override it.
- A spinner with a file counter shows only on a terminal, and only when a run passes one second.

## 2. Per command

| Command | Shows | Exit code |
| --- | --- | --- |
| `pun format` | file lines (✔ formatted), summary | 0 (Pint parity) |
| `pun format --test` | file lines (✖ needs formatting), summary | 1 if any file needs formatting (Pint parity) |
| `pun lint` | problem blocks, summary | 1 if any error |
| `pun lint --fix` | file lines for what was fixed, then problem blocks for what remains, summary | 1 if errors remain |
| `pun analyse` | problem blocks, summary | 1 if any error |
| `pun check` | three sections (Format, Lint, Analyse), one summary (decided) | 1 if any error or any file needs formatting |
| `pun migrate` | one line per config moved, then each unsupported item as a ⚠ block, summary | 1 if anything unsupported stopped it |
| `pun init` | init.md §2 | 0 |

Exit code 2 means Pun itself couldn't run: a bad config, an unreadable file, or a crash. That is the same for every
command.

**Not changed:** the machine formats (`--format=json|checkstyle|junit|gitlab|github|sarif`, Pint's and PHPStan's
shapes), and agent mode (compact `path:line:col rule message` lines, PLAN.md C4).

## 3. Questions

1. **Problem blocks with code, or one line each?** The default could be the rich block above (oxlint, rustc, Biome)
   or one line per problem (Ruff's default, ESLint's `compact`), with the other behind a flag.
   *Recommend: rich blocks by default, with `--concise` for one line each. People read the default; agents already get
   one-line output.*
2. **`pun check`: grouped by file, or by command?** By file shows everything about `User.php` together, mixing format,
   lint and analyse. By command shows three sections.
   *Recommend: by file, sorted by line. You fix a file, not a command.*
3. **GitHub Actions.** Detect `GITHUB_ACTIONS` and add `::error` annotations automatically (as oxlint and Biome do),
   so problems show on the PR diff?
   *Recommend: yes, on top of the normal output, with `--format=github` to force it.*
4. **Unchanged files.** Show nothing for clean files (the approved format design), or a dim line each with `-v`?
   *Recommend: nothing by default, a dim line with `-v`.*
5. **Pint's own screen.** `pun format` today prints Pint's exact screen. With the new design approved, is Pint's human
   screen kept anywhere (for example `--format=pint`)?
   *Recommend: no. Exit codes and machine formats stay Pint-compatible, which is what CI depends on.*
6. **Brand colours.** Use the terminal's standard red, yellow, green and cyan (they follow the user's theme), or
   Pun's brand colours from design/tokens.json?
   *Recommend: standard terminal colours. They are readable on every theme; brand colours go in the docs and site.*

## Decisions

From earlier rounds (Pushpak, 2026-10-17): format.md §10 approved; only lint `error` fails (oxlint); analyse findings
are always errors.

Answered 2026-10-17 (Pushpak):

- **Q1: problem blocks with code by default**; `--concise` prints one line per problem.
- **Q2: `pun check` groups by command**, the way Vite+'s `vp check` does. Each step prints one status line with its
  count and time, and its problems follow under it. The final summary line comes last:
  ```
  ✔ format   All 412 files formatted                         (0.08s)
  ✖ lint     2 errors, 1 warning in 2 files · 2 fixable      (0.12s)
     …problem blocks…
  ✖ analyse  1 error in 1 file                               (0.21s)
     …problem blocks…
  ✖ 3 errors · 1 warning · 2 fixable (run pun check --fix) · 412 files in 0.31s
  ```
  A passing step is one line. Pushpak reconsidered by file vs by command and kept by command after comparing with
  Vite+, whose output is one `pass:` line per step.
- **Q3: GitHub annotations are automatic** when `GITHUB_ACTIONS` is set, on top of the normal output; `--format=github`
  forces them.
- **Q4: clean files print nothing**; `-v` adds a dim line per clean file (assistant default, as approved for format).
- **Q5: Pint's human screen is dropped** once Pun's output lands. Exit codes and machine formats stay Pint- and
  PHPStan-compatible; scripts/cli-parity.sh then covers only those. AGENTS.md ("output, messages and exit codes must
  match upstream") needs updating when this is built.
