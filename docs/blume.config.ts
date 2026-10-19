import { defineConfig } from "blume";

export default defineConfig({
  title: "Pun",
  description:
    "Format, lint, and analyse PHP 8.4+ projects with one binary and one pun.toml.",
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
        "Pun is a Rust toolchain for PHP 8.4+ projects. It reads pun.toml. `pun format` changes layout; `pun lint --fix` applies code fixes; `pun analyse` reports type errors; `pun check` runs all three, with --fix for format and lint edits. `pun migrate` converts existing tool configs and reports skipped items. PHPStan, Larastan, and Rector support is incomplete; Planned rules are not implemented. Blade formatting requires Node.js and Prettier. Coding agents receive compact output automatically.",
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
