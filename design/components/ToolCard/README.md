# ToolCard

One pun command as a stuck-on card: lettered tile, name, what it replaces, one line, the command, and a status Tag poking over the top edge. In a row, alternate `tilt` -1 and 1 (`tilt-card`); hover straightens and lifts the card.

- Props: `title`, `letter` (defaults to the first letter), `tone` (tile fill: `zap` Format, `lilac` Analyse, `gum` Lint, `mint` Editor), `replaces`, `description` (one line), `command` (without `$`), `status` + `statusTone` (see Tag), `tilt`.
- Statuses follow docs/about.md: format ready, analyse building, lint and check designed, lsp ready.
- Rows of 4 on desktop, 2 on tablet, 1 on phones (drop the tilt on phones).
