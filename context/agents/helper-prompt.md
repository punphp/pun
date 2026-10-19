# Prompt for a helper agent

Paste this, unchanged, into any number of new Claude Code or Codex sessions, on Pushpak's Mac or on a server.
Each one picks its own name and work from the helper queue in context/agents/README.md.

```text
You are joining a team of AI agents building Pun: one Rust binary that replaces Laravel Pint / PHP-CS-Fixer
(`pun format`), Rector (`pun lint`) and PHPStan + Larastan (`pun analyse`), with byte-identical output to the
pinned upstream tools. Other agents are pushing to origin/main right now; you coordinate with them only through
git and context/PLAN.md.

1. Get the latest code: in an existing checkout of git@github.com:punphp/pun.git run
   `git fetch origin && git merge --ff-only origin/main`; otherwise clone it. If you are on Pushpak's Mac,
   work in /Users/pushpakchhajed/Projects/pun.
2. Read context/agents/README.md and follow it exactly. It covers ownership, the helper queue and race-safe
   claiming, shared-file rules, the git protocol (push to main; on rejection fetch + merge origin/main; never
   rebase shared history, never force), builds (the heavy lock on the Mac; "Server setup" elsewhere), the
   quality bar and cleanup.
3. Read AGENTS.md, context/PLAN.md (§2 decisions, §3 build rows and claims), context/plan/rules.md and the plan
   for the area you claim (context/plan/lint.md, analyse.md or format.md).
4. Claim the first unclaimed slice of the helper queue as README.md describes, under a name you pick
   (`helper-<host>-<4 random letters>`).
5. Work batch after batch (5-10 rules, most-used first): port upstream function by function, test against
   upstream fixtures and the real tool's output on the corpus, set `ported = true` per rule in
   crates/pun-config/rules/mapping.toml with single-line edits, keep your files at 100% coverage, commit only
   your own paths, push after every batch.
6. Never stop to ask. Record a `needs Pushpak` row in PLAN.md §3 and continue. When your slice is done, mark its
   row done, push, and claim the next slice. Stop only when the queue is empty; then report the shas that
   landed, what is left, and any request / needs Pushpak rows.
```
