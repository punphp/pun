# About Pun

Pun is one fast Rust binary for the PHP toolchain. It replaces Pint, PHP-CS-Fixer, Rector, PHPStan and Larastan (and later Composer and Pest's runner), with one install, one `pun.toml`, and the same results as the tools you use today.

**Why.** A Laravel project runs four or five PHP tools on every commit. Each one is slow (Pint takes 190 s on laravel/framework), has its own config format, rule names and ignore comments, overlaps with the others, and prints for humans while coding agents pay for every token. Rust tools like Mago are fast but not drop-in, so switching costs a reformat diff and a new baseline, and teams stay on the slow tools.

**How.**
- **Exact parity first, so migration day changes nothing.** Every command is checked against pinned versions of the real tools, their own test suites, real projects and a fuzzer.
- **Then a better model.** `pun migrate` moves a project onto one `pun.toml`, with every rule in exactly one place, short Pun rule names mapped to their upstream originals, and no levels.
- **Built on Mago's crates, Laravel first, PHP 8.4+.**

**Commands.**
- `pun format`: layout only, never changes behaviour.
- `pun lint [--fix]`: code rules, from Rector and Mago.
- `pun analyse`: type errors, from PHPStan and Larastan.
- `pun check [--fix]`: all three in one pass.
- `pun migrate`: move a project onto `pun.toml`.
- `pun lsp`: editor server, with PHPantom plus Pun's rules.

Under a coding agent, every command switches to compact `path:line:col rule message` output.

**Principles.** Prove every speed and parity claim against the real tool. Need no PHP or Node to check a project. Keep the code small, ported function by function, with 100% test coverage.

**Status.** `pun format` is done: Pint-identical on 7 corpus projects, and laravel/framework runs in 3.7 s cold and 0.07 s warm. `pun analyse` is in progress. `pun lint`, `pun migrate` and `pun check` are designed. The first release ships once format, analyse, lint and migrate exist. See [PLAN.md](PLAN.md), [RULES.md](RULES.md) and [scoreboard.md](scoreboard.md). MIT licensed.
