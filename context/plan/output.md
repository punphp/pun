# Plan: terminal output for every command

Status: **walked through with Pushpak** (2026-10-17); answers in "Decisions". Nothing is built. Pushpak wants one
consistent output for format, lint, analyse, check, migrate and init. The approved starting point is format.md §9
(✔/✖ per file, a summary line with counts and time, colour, rustc-style code frames). This file extends it to every
command. Decisions with dates: ../PLAN.md §2. Build list: §4.

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

**4. Run errors** (bad config, an unknown or unported rule in pun.toml, an unreadable file): one red line, then a
hint. They exit 2.
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
| `pun check` | one status line per step (format, lint, analyse) with its problems under it, one summary (decided) | 1 if any error or any file needs formatting |
| `pun migrate` | one line per config moved, then each unsupported item as a ⚠ block, summary | 0 when the rest migrated (unsupported items are named and skipped) |
| `pun init` | init.md §2 | 0 |

Exit code 2 means Pun itself couldn't run: a bad config, an unreadable file, or a crash. That is the same for every
command.

**Not changed:** the machine formats (`--format=json|checkstyle|junit|gitlab|github|sarif`, Pint's and PHPStan's
shapes), and agent mode (compact `path:line:col rule message` lines, §3).

## 3. Agent mode (unchanged)

Under a coding agent (laravel/agent-detector v2 env list and `/opt/.devin`, then the agent-detector crate; Cursor's
`CURSOR_TRACE_ID` alone doesn't count; `PUN_AGENT=1|0`, `--agent` / `--no-agent`, `PAO_FORCE` / `PAO_DISABLE`) every
command prints compiler-style lines, not JSON:

```
pun analyse: fail, 3 errors in 2 files (412 files checked, 0.21s)
app/Models/User.php:42:9 analyse/undefined/method Call to undefined method App\Models\User::foo().
app/Http/Kernel.php:12:1 lint/style/prefer-early-return [fix:safe] Use an early return.
+2 more; run with -v for all
```

Summary line first (a passing run prints only that line); `path:line:col rule message` with an optional
`[fix:safe|unsafe]`; bare diff hunks; no ANSI, progress or boxes; lists capped with `+N more`; errors are one line
(`pun format: error: <message>`); `--format=json` stays available; exit codes don't change. Implemented once in
`crates/pun-cli/src/agent.rs`, pinned per command in `crates/pun-cli/tests/agent.rs`. Rule IDs switch to the new
names with config.md Build 1.

## 4. Build

1. **The renderer** in pun-cli: file lines, problem blocks (rustc-style frames from a span), the summary line, run
   errors with hints, `--concise`. One module that every command calls.
2. **The look:** brand colours from design/tokens.json at truecolor, 256 and 16 colours, light and dark variants,
   `NO_COLOR`, `--color`, ASCII fallbacks, clickable paths and rule links (OSC 8), the spinner after one second.
3. **`pun format` on it,** and Pint's human screen removed (format.md Build 2).
4. **GitHub annotations** when `GITHUB_ACTIONS` is set; `--format=github` forces them.
5. **`pun check`'s grouped view:** one status line per step with its count and time, problems under it, one summary.
6. **Exit code 2** for run errors in every command.
   *Done when* each sample in §1, §2 and the grouped `pun check` decision is pinned by a test, with and without colour.

## Decisions

From earlier rounds (Pushpak, 2026-10-17): format.md §9 approved; only lint `error` fails (oxlint); analyse findings
are always errors.

Answered 2026-10-17 (Pushpak):

- **Problem blocks with code by default**; `--concise` prints one line per problem.
- **`pun check` groups by command**, the way Vite+'s `vp check` does. Each step prints one status line with its
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
- **GitHub annotations are automatic** when `GITHUB_ACTIONS` is set, on top of the normal output; `--format=github`
  forces them.
- **Clean files print nothing**; `-v` adds a dim line per clean file (assistant default, as approved for format).
- **Pint's human screen is dropped** once Pun's output lands. Exit codes and machine formats stay Pint- and
  PHPStan-compatible; scripts/cli-parity.sh then covers only those. AGENTS.md ("output, messages and exit codes must
  match upstream") needs updating when this is built.
- **Pun's brand colours** from design/tokens.json (Sticker Pop) instead of the terminal's standard colours. Pun
  uses truecolor when `COLORTERM` is `truecolor` or `24bit`, the nearest 256-colour value otherwise, and the standard 16
  colours on basic terminals. It picks the light or dark token variant for readability on the terminal's background
  (dark by default when it can't tell). `NO_COLOR`, piping and `--color` work as in §1.

Assistant's calls while building Build 1-2 (`crates/pun-cli/src/render/`, wf/renderer, 2026-10-17), where the plan is
silent:

- **Columns from the §1 samples:** file lines pad the path to 27 characters (or the longest path in the batch plus two
  spaces), so the rules start in one column; a problem's `fixable` label starts at character 78 of its header (two
  spaces after a longer message). An unsafe fix reads `fixable (unsafe)`.
- **Code frames:** one line of context before the span unless that line is blank; no context after. A tab shows as 4
  spaces, and indentation deeper than 4 columns is cut to 4 (the §1 samples: 8 spaces at `:42:9` show as 4; one tab at
  `:23:16` shows as 4). A span over more than 3 lines shows its first two lines, `…`, and its last. An empty span, or
  one at the line end, gets a one-column marker. No help closes the frame with a bare `╰─`. Columns count bytes, as
  `pun_problem::LineIndex` does.
- **`--concise`:** ` ✖ path:line:col  rule  message  fixable`.
- **Summary glyph:** ✖ with any error or file needing formatting, ⚠ with only warnings, ✔ `No problems` otherwise.
- **Token roles:** error `bad`, success `ok`, rule IDs `lilac-deep`, dim `ink-3`, warning `zap` on dark. The tokens
  have no warning colour that reads on a light ground (zap is "never as text on paper"), so light terminals get their
  own yellow (SGR 33) for warnings. At 16 colours: red, yellow, green, cyan and SGR 2 (dim), as §1 says. 256 colours
  pick the nearest of xterm 16-255 (cube and grey ramp) by RGB distance.
- **Detection:** `--color=always|never` win, then non-empty `NO_COLOR` or `TERM=dumb` turn colour off, then a terminal
  gets colour and a pipe doesn't. 256 colours when `TERM` contains `256`. Light when `COLORFGBG`'s background is 7 or
  9-15. Unicode with a UTF-8 locale (`LC_ALL`, `LC_CTYPE`, `LANG`, first set) and not `TERM=linux`, or in Windows
  Terminal. ASCII fallbacks: `x` `!` `ok`, frame `,-` `|` `^` `` `- ``, `...`, and `-` for the summary's `·`.
- **Links (OSC 8):** only on a terminal (or `--color=always`), never under `--color=never`, and only in terminals
  known to support them (iTerm2, WezTerm, VS Code, Ghostty, Hyper, Windows Terminal, Konsole, VTE ≥ 0.50, kitty,
  Alacritty, foot); `FORCE_HYPERLINK` overrides. Paths link to `file://<root>/<path>` (percent-encoded). Rule IDs link
  to `<docs base><rule ID>` once a docs base is passed in; the site has no rule pages yet, so none is set.
- **Spinner:** on stderr, braille frames (`|/-\` in ASCII), `⠋ 120/412 files` every 80 ms after one second; dropping
  it clears the line.
- **`--color` is not a flag yet:** `ColorChoice` is a clap `ValueEnum` ready for output 3, which wires it in when
  `pun format` moves onto the renderer; Pint's `--ansi`/`--no-ansi` screen is untouched until then.
