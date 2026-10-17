# CodeBlock

Code, diffs and terminal sessions on `code-bg`, in a `radius-lg` sticker with the `pop` shadow. Lines colour themselves by prefix: `$ ` becomes a command with a `gum` prompt, `+` `code-add`, `-` `code-del`, `@@` and `#` comments `code-dim`, `✓` `code-add`, and a leading `!` marks a hot result line in `zap` (the `!` is not shown).

- Props: `lines` (array of strings) or `code` (one string), `title`, `meta` (right side of the bar: language or shell), `dots` (three brand-colour window dots for terminals).
- Show real output. Diffs and timings come from the repo's tests and benchmarks, never invented.
