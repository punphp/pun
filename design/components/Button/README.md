# Button

Pill-shaped sticker button: 3px `outline`, `pop` hard shadow, lifts on hover and sinks on press. One `zap` button per view, for the main action ("Get started", "Install"). `plain` (card) for the second action, `gum` only inside or next to a `gum`-free area such as the closing CTA, `ink` for a third action on a light panel.

- Props: `variant` (`zap` | `plain` | `gum` | `ink`, default `plain`), `size` (`md` | `sm`), `arrow` (adds a → that nudges on hover), `href` (renders an `<a>`), plus any button attributes.
- Labels are verbs, sentence case, two or three words: "Get started", "Read the docs". No "Click here", no emoji.
- `sm` is for the nav and dense rows; it uses `pop-sm`.
- Focus: 3px solid `focus` ring, 3px offset. Never remove it.
