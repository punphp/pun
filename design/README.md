Pun is one fast Rust binary for the whole PHP toolchain: `pun format`, `pun lint`, `pun analyse`, `pun check`, `pun lsp`. It is to PHP what Bun is to JavaScript. The brand is **Sticker Pop**: cream paper, pastel fills, thick ink outlines and hard offset shadows, like a laptop lid covered in stickers. Loud and cheeky on the surface, exact underneath: every claim is a number the repo can reproduce.

![The Pun mascot](assets/Mascot/pun-sprinter-512.png)

## Voice

Pun talks like a fast friend who also reads the benchmark methodology.

- **Lead with the number, then the joke.** "Format laravel/framework in 3.7 s. Pint takes 190." One pun or wink per page, never in docs headings.
- **Short.** Headlines under eight words. Ledes one sentence, max 30ch: "One Rust binary. Your pint.json. The exact same output."
- **Concrete nouns.** Name the tool replaced (Pint, PHP-CS-Fixer, Rector, PHPStan, Larastan), the file (`pint.json`, `pun.toml`), the flag (`--test`). No "blazing", "revolutionary", "seamless", "supercharge".
- **Prove it.** Only publish timings the repo measured (today: laravel/framework, Pint 190 s, pun 3.7 s cold, 0.07 s warm, so "51×"). Every chart links to "reproduce →".
- **Casing.** The product is "Pun" in sentences and "pun" in the wordmark, commands and the Ribbon. Sentence case for headings and buttons. Commands always in mono with `$`.
- **No emoji** in product UI or docs. The mascot and the bolt carry the personality.
- **You, not we.** "Your pint.json", "your CI". "We" only in the changelog.

Real copy to reuse: "PHP tooling, zero waiting." · "Four tools. One binary." · "Same bytes as Pint." · "Blink and it's done." · "Stop waiting on your tools." · "Made for PHP people who hate waiting."

## Color

Two themes: **Paper** (light, the default) and **Night** (dark). Most surfaces are neutral; colour comes in flat, outlined stickers.

- Ground is `paper`; sunken bands and the docs sidebar `paper-2`; every sticker surface (cards, buttons, pills) is `card`.
- Text is `ink`, secondary `ink-2`, captions and kickers `ink-3`. Links are `lilac-deep`, the only brand colour used as text.
- Four fills, each with a job:
  - `zap` is speed: the one primary Button, Highlight words, the bolt, Format, pun's cold-run bar.
  - `lilac` is the brand ground: hero StickerPanel, app icon, Analyse.
  - `gum` is play: Bursts, the closing CTA, the warm-cache bar, the `$` prompt.
  - `mint` is ready: status Tags, the version chip.
- Text on any fill is always `ink` (7.6:1 on `gum`, the lowest). Fills are 1.2:1 to 2.2:1 against `paper`, so **every fill gets the `outline`**; that outline is the brand, never drop it.
- `grape` and `grape-deep` are the mascot's own colours, for mascot-adjacent grounds only.
- Status text uses `ok` (blue) and `bad` (orange-red), always with a ✓ / ✗ glyph or the word, so it never relies on telling red from green.
- Code, terminals and the InstallBox are `code-bg` in both themes, with `code-add`, `code-del`, `code-dim` and `zap` for the hot result line.
- In Night, outlines turn light (`outline`) and hard shadows turn `#5B3FD9`, so stickers still lift off the dark ground.

## Type

Two families, both from Google Fonts: **Bricolage Grotesque** (`display`) for everything you read, **Martian Mono** (`mono`) for everything you type or measure.

- Headlines are heavy (800) and tight (-0.035 to -0.06em). `display` for the hero, `h1` for sections, `h2` for docs and the CTA, `h3` for cards.
- `mega` is reserved for the one speed multiple on a page (51×).
- Body copy is `body` at 16/1.6, max 68ch; ledes `lede` in `ink-2`, max 30ch.
- Mono covers commands, flags, file names, numbers in charts, Tags, Pills and the uppercase `kicker` above section headlines.
- Never set "pun" in a font as a logo; use the wordmark SVG.

## Shape, depth and texture

- **Outline:** `stroke` (3px) on cards, buttons, panels, code; `stroke-thin` (2px) on Pills, Tags, kbd.
- **Radius:** pills for Buttons, Tags, bars (`radius-pill`); `radius-lg` cards and code; `radius-xl` hero and CTA panels; `radius-md` the InstallBox and icon tiles; `radius-sm` copy buttons and small tiles.
- **Shadow:** hard offsets only, no blur. `pop` default, `pop-sm` for small things, `pop-lg` for panels, `pop-lilac` only on the InstallBox. Hover lifts (`translate(-2px,-2px)` to `pop-hover`), press sinks (`translate(3px,3px)` to `pop-press`).
- **Tilt:** ToolCards in a row alternate `tilt-card` -1 / +1deg; one sticker Highlight per headline at `tilt-word`; loose tags `tilt-sticker`; Bursts `tilt-burst`; one Ribbon per page at `tilt-ribbon`. Body text, tables and code are never tilted.
- **Texture:** `dot-grid` at 15% on lilac panels; `stripe` marks the slow lane (competitors) in benchmarks.
- **Spacing:** a 4px base. Card padding `space-6`, card gaps `space-5`, sections `space-24` apart, the closing CTA `space-28` after the last section.
- **Focus:** a 3px solid `focus` ring with a 3px offset on everything interactive.

## Motion

Quick and physical: 120 to 150ms ease for lift and press, a 30s linear loop for the Ribbon, an arrow that nudges 3px on hover. Nothing fades in on scroll. Everything stops under `prefers-reduced-motion`.

## Mascot and logos

- The **mascot** is the sprinter elephant (Assets > Mascot): grape skin, yellow high-tops, bolt cowlick, a wink. Hero panels, docs sidebar, 404, social cards. It always faces right and runs forward.
- The **wordmark** (Assets > Logos) is the bolt plus hand-built "pun" lettering; `pun-wordmark.svg` on light grounds, `pun-wordmark-reverse.svg` on dark.
- The **bolt** alone is the small mark: favicon, avatars, Ribbon separators.

## Iconography

No icon font. Status uses text glyphs from the CLI itself: ✓ pass, ✗ fail, ➜ the current line, `$` the prompt. Tool tiles use a single capital letter (F, A, L, E). The bolt is the only drawn icon. If a UI ever needs more icons, use a 2px-stroke outline set at 20px in `ink` and flag it here first.

## Layout

- Marketing pages: 1200px max, 56px side padding, hero two columns (copy left, mascot panel right), sections stacked with a kicker, an `h1` and one visual. See the LandingPage card.
- Docs: 250px sidebar on `paper-2`, 68ch article, 200px "On this page" rail. See the DocsPage card.
- Phones: one column, 24px gutter, ToolCards single file without tilt, `display` drops to 56px and `mega` to 120px.
