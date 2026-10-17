import { defineConfig } from "blume";

// The Pun docs site and homepage. Run from this folder: `npm install`, then `npm run dev` / `npm run build`.
// Content lives in content/ (routes under /docs); the homepage is pages/index.astro.
// The other Markdown files in this folder (PLAN.md, TASKS.md, RULES.md, ...) are internal notes and are not published.
export default defineConfig({
  title: "Pun",
  description:
    "One fast Rust binary for the whole PHP toolchain: format, lint, analyse and migrate, with the same results as Pint, Rector and PHPStan.",
  logo: { image: "/logo.svg", text: "" },

  content: {
    root: "content",
  },

  github: {
    owner: "punphp",
    repo: "pun",
    dir: "docs",
  },

  lastModified: "git",

  navigation: {
    tabs: [{ label: "Docs", path: "/docs" }],
  },

  theme: {
    // lilac-deep, the only brand colour used as text (5.6:1 on paper; lighter in dark mode)
    accent: { light: "#5B3FD9", dark: "#B3A3FF" },
    // zap, the primary call to action (labels render dark on it)
    action: "#FFE53D",
    background: { light: "#FBF6EC", dark: "#120E18" },
    radius: "lg",
    mode: "system",
    fonts: {
      display: { name: "Bricolage Grotesque", weights: "400..800" },
      body: { name: "Bricolage Grotesque", weights: "400..800" },
      mono: { name: "Martian Mono", weights: [400, 500, 600, 700] },
    },
  },

  footer: {
    copyright: `© ${new Date().getFullYear()} Pun. MIT licensed.`,
    links: [
      { label: "Docs", href: "/docs" },
      { label: "Parity", href: "/docs/parity" },
      { label: "Report an issue", href: "https://github.com/punphp/pun/issues" },
    ],
  },

  agents: {
    llmsTxt: {
      details:
        "Pun is a single Rust binary that replaces Pint, PHP-CS-Fixer, Rector, PHPStan and Larastan for PHP 8.4+ projects, with output identical to the tools it replaces: `pun format` (layout, behaviour-safe), `pun lint` (risky fixers, Rector and Mago rules), `pun analyse` (PHPStan + Larastan), `pun check` (all three in one pass; `pun check --fix` writes exactly what Pint wrote), `pun migrate` (writes pun.toml from pint.json, phpstan.neon and rector.php) and `pun lsp` (editor server). Pun reads only pun.toml. Under a coding agent, every command switches to compact agent output automatically.",
    },
  },

  seo: {
    organization: {
      name: "Pun",
      logo: "/app-icon.svg",
      sameAs: ["https://github.com/punphp/pun"],
    },
  },
});
