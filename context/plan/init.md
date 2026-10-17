# Plan: `pun init`

Status: **walked through with Pushpak** (2026-10-17); answers in "Decisions". Nothing is built. The journey comes from
Pushpak (lint.md §9); the details below are proposals with questions. Answers go into "Decisions" and DECISIONS.md.
Build list: §4.

## 1. What it is

`pun init` writes pun.toml for a project that has no Pint, PHP-CS-Fixer, Rector or PHPStan config yet. A project that
has one runs `pun migrate` instead. Every other command stops without pun.toml and says "run `pun migrate`, or
`pun init` for a new project" (decided).

## 2. The flow (proposal)

```
$ pun init

  Pun · new project setup
  Found Laravel 13, Pest 4, PHP ^8.4 (composer.json)

? Code style                    › laravel        laravel | psr12
? Lint sets                     › [x] dead-code  [x] modernize  [ ] code-quality  [ ] types
                                  [ ] early-return  [ ] privatization  [ ] naming  [ ] coding-style
? Analysis preset               › recommended    basic | essential | recommended | strict | max

  Laravel, Pest and PHPUnit rules turn on by themselves (composer.lock).

  ✔ Wrote pun.toml
  Checking the project with these settings…
  ✖ 14 files need formatting · 9 lint problems (9 fixable) · 3 analyse errors · 212 files in 0.3s

? Fix what can be fixed now (pun check --fix)?          › yes
? Write a baseline for the 3 remaining analyse errors?  › no

  Done. Run `pun check` any time; `pun check --fix` before you commit.
  Add `pun check` to CI and your composer scripts.
```

1. **Detect** from composer.json and composer.lock: the PHP version, Laravel, Pest, PHPUnit and their versions.
2. **Ask three questions** with the detected defaults: `laravel` style for a Laravel app (otherwise `psr12`),
   dead-code + modernize, and `recommended`.
3. **Write pun.toml** (Q2 decides how much it contains).
4. **Show what the settings find** with one `pun check` run, so the choice isn't blind.
5. **Offer the next steps**: fix now, baseline the rest, wire up composer scripts and CI (Q3, Q4).

Non-interactive (agents, CI, no terminal, `--yes`): the defaults, or flags `--style laravel --lint-sets dead-code,types
--analyse strict`. Nothing else is asked; next steps are printed, not run.

## 3. Questions

1. **Old configs present.** If `pun init` finds pint.json, rector.php or phpstan.neon, should it stop and say
   "run `pun migrate`", or offer to run migrate itself?
   *Recommend: offer it ("Found phpstan.neon. Migrate it instead? › yes"), since the user's goal is the same either
   way.*
2. **What goes in pun.toml.** Only the three choices (a short file that relies on defaults), or every rule written out?
   *Recommend: only the choices. Five lines are readable, and new rules in later Pun versions reach the project.
   `pun config` prints the fully resolved list.*
3. **Run a check after writing.** Should init run `pun check` once and offer `--fix` and a baseline?
   *Recommend: yes; it turns "I set it up" into "it's working" in one step.*
4. **Wire up the workflow.** Offer to add a composer script, a GitHub Actions workflow and a pre-commit hook?
   *Recommend: offer composer script + CI (one question), and leave hooks for later.*
5. **Format preset `empty`.** It's one of the three presets, but init only offers laravel and psr12. Hide `empty` from
   init?
   *Recommend: hide it; `empty` is for people writing their own rules by hand.*

## 4. Build

After config.md Build 3 and 5 (the schema and composer.lock detection) and output.md Build 1:

1. **Detection and the three questions** with their defaults: `laravel` for a Laravel app, else `psr12`; dead-code +
   modernize; `recommended`. `empty` is not offered.
2. **Old configs found:** offer `pun migrate` ("Found phpstan.neon. Migrate it instead? › yes").
3. **Write pun.toml** with only the three choices. Refuse to overwrite an existing pun.toml.
4. **One `pun check`** with the new settings, then offer `--fix` and a baseline (interactive only).
5. **Non-interactive** (`--yes`, agents, CI, no terminal): defaults or `--style`, `--lint-sets`, `--analyse`; next
   steps printed, not run.
   *Done when* the flow in §2 is pinned by a test on a scratch Laravel project, interactive and not.

## Decisions

Answered 2026-10-17 (Pushpak):

- **Q1: old configs present → offer migrate** ("Found phpstan.neon. Migrate it instead? › yes"); yes runs `pun migrate`.
- **Q2: pun.toml holds only the three choices**; presets carry the rules, `pun config` prints the resolved list.
- **Q3: init runs one `pun check`** after writing pun.toml, then offers `--fix` and a baseline (interactive only).
- **Q4: no workflow wiring in init** (too much code to maintain for now). init prints the commands; the user adds them.
  Later: agent skills (a `pun-init` and a `pun-migrate` skill) do the composer, CI and hook wiring instead of Pun code.
- **Q5: `empty` is hidden from init** (assistant default); it stays a valid `[format] preset`.
