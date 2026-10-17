import { defineConfig } from "blume";

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
    accent: { light: "#5B3FD9", dark: "#B3A3FF" },
    action: "#FFE53D",
    background: { light: "#FBF6EC", dark: "#120E18" },
    radius: "lg",
    mode: "system",
    fonts: {
      display: { name: "Fredoka", weights: ["300..700"] },
      body: { name: "Fredoka", weights: ["300..700"] },
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
        "Pun is a single Rust binary that replaces Pint, PHP-CS-Fixer, Rector, PHPStan and Larastan for PHP 8.4+ projects, with output identical to the tools it replaces: `pun format` (layout, behaviour-safe), `pun lint` (risky fixers, Rector and Mago rules), `pun analyse` (PHPStan + Larastan), `pun check` (all three in one pass; `pun check --fix` writes exactly what Pint wrote) and `pun migrate` (writes pun.toml from pint.json, phpstan.neon and rector.php). Pun reads only pun.toml. Under a coding agent, every command switches to compact agent output automatically.",
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
