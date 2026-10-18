//! The look (output.md Build 2): which colours, glyphs and links a terminal gets.
//!
//! Colours are Pun's brand tokens from design/tokens.json: 24-bit when `COLORTERM` is `truecolor` or `24bit`, the
//! nearest xterm 256-colour value when `TERM` names a 256-colour terminal, and the standard 16 colours otherwise. The
//! light or dark token variant follows the terminal's background (`COLORFGBG`), dark when it can't tell.

use std::io::IsTerminal;
use std::sync::LazyLock;

/// `--color=auto|always|never`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, clap::ValueEnum)]
pub enum ColorChoice {
    #[default]
    Auto,
    Always,
    Never,
}

/// How many colours the terminal shows; `None` means no colour at all.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Depth {
    None,
    Ansi16,
    Ansi256,
    TrueColor,
}

/// The terminal's background, which picks the light or dark token variant.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Theme {
    Dark,
    Light,
}

/// What a piece of text means; [`Look::paint`] turns it into a colour.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Role {
    /// ✖, error counts, the marker under an error.
    Error,
    /// ⚠, warning counts, the marker under a warning.
    Warning,
    /// ✔ and "No problems".
    Ok,
    /// Rule IDs.
    Rule,
    /// Unchanged files, timings, the frame's gutter.
    Dim,
}

/// What one output stream can show.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Look {
    pub depth: Depth,
    pub theme: Theme,
    /// False: ASCII fallbacks (`x`, `!`, `ok`, `|`, `^`).
    pub unicode: bool,
    /// OSC 8 hyperlinks for paths and rule IDs.
    pub links: bool,
}

impl Look {
    /// No colour, no links, Unicode glyphs: what a pipe gets with a UTF-8 locale.
    #[cfg(test)]
    pub fn plain() -> Look {
        Look {
            depth: Depth::None,
            theme: Theme::Dark,
            unicode: true,
            links: false,
        }
    }

    /// The look for stdout under `--color=<choice>`.
    pub fn stdout(choice: ColorChoice) -> Look {
        Look::detect(choice, std::io::stdout().is_terminal(), &|k| {
            std::env::var(k).ok()
        })
    }

    /// The look for stderr (run errors) under `--color=<choice>`.
    pub fn stderr(choice: ColorChoice) -> Look {
        Look::detect(choice, std::io::stderr().is_terminal(), &|k| {
            std::env::var(k).ok()
        })
    }

    /// `--color=always|never` win; then `NO_COLOR` (non-empty) and `TERM=dumb` turn colour off; otherwise colour is
    /// on for a terminal and off for a pipe. Links need a terminal (or `always`) that is known to support them.
    pub fn detect(choice: ColorChoice, tty: bool, env: &dyn Fn(&str) -> Option<String>) -> Look {
        let var = |k: &str| env(k).filter(|v| !v.is_empty());
        let term = var("TERM").unwrap_or_default();
        let styled = match choice {
            ColorChoice::Always => true,
            ColorChoice::Never => false,
            ColorChoice::Auto => tty,
        };
        let colour = match choice {
            ColorChoice::Auto => tty && var("NO_COLOR").is_none() && term != "dumb",
            _ => styled,
        };
        let depth = if !colour {
            Depth::None
        } else if matches!(var("COLORTERM").as_deref(), Some("truecolor" | "24bit")) {
            Depth::TrueColor
        } else if term.contains("256") {
            Depth::Ansi256
        } else {
            Depth::Ansi16
        };
        Look {
            depth,
            theme: theme(var("COLORFGBG")),
            unicode: unicode(&var),
            links: styled && hyperlinks(&var, &term),
        }
    }

    /// `text` in the colour of `role`, reset after it. Unchanged without colour.
    pub fn paint(&self, role: Role, text: &str) -> String {
        let code = match self.depth {
            Depth::None => return text.to_string(),
            Depth::Ansi16 => ansi16(role).to_string(),
            depth => match token(role, self.theme) {
                None => ansi16(role).to_string(),
                Some((r, g, b)) if depth == Depth::TrueColor => format!("38;2;{r};{g};{b}"),
                Some(rgb) => format!("38;5;{}", nearest_256(rgb)),
            },
        };
        format!("\x1b[{code}m{text}\x1b[0m")
    }

    /// `text` as an OSC 8 link to `url` when links are on.
    pub fn link(&self, url: &str, text: &str) -> String {
        if self.links {
            format!("\x1b]8;;{url}\x1b\\{text}\x1b]8;;\x1b\\")
        } else {
            text.to_string()
        }
    }

    /// The Unicode glyph, or its ASCII fallback.
    pub fn glyph<'a>(&self, unicode: &'a str, ascii: &'a str) -> &'a str {
        if self.unicode { unicode } else { ascii }
    }
}

/// The standard colour (or dim) for a role.
fn ansi16(role: Role) -> &'static str {
    match role {
        Role::Error => "31",
        Role::Warning => "33",
        Role::Ok => "32",
        Role::Rule => "36",
        Role::Dim => "2",
    }
}

/// The brand token for a role. The tokens have no warning colour that reads on a light ground (zap is "never as text
/// on paper"), so a light terminal's warnings use its own yellow.
fn token(role: Role, theme: Theme) -> Option<(u8, u8, u8)> {
    let name = match (role, theme) {
        (Role::Error, _) => "bad",
        (Role::Warning, Theme::Dark) => "zap",
        (Role::Warning, Theme::Light) => return None,
        (Role::Ok, _) => "ok",
        (Role::Rule, _) => "lilac-deep",
        (Role::Dim, _) => "ink-3",
    };
    let t = TOKENS.iter().find(|t| t.0 == name).expect("a brand token");
    Some(match theme {
        Theme::Dark => t.2,
        Theme::Light => t.1,
    })
}

type Rgb = (u8, u8, u8);

/// design/tokens.json's colour tokens: (name, light, dark).
static TOKENS: LazyLock<Vec<(String, Rgb, Rgb)>> = LazyLock::new(|| {
    let doc: serde_json::Value =
        serde_json::from_str(include_str!("../../../../design/tokens.json")).expect("tokens.json");
    let hex = |v: &serde_json::Value| {
        let s = v.as_str().expect("a colour");
        let n = u32::from_str_radix(&s[1..], 16).expect("#RRGGBB");
        ((n >> 16) as u8, (n >> 8) as u8, n as u8)
    };
    doc["color"]["tokens"]
        .as_array()
        .expect("color.tokens")
        .iter()
        .map(|t| {
            let v = &t["value"];
            let (light, dark) = if v.is_string() {
                (hex(v), hex(v))
            } else {
                (hex(&v["light"]), hex(&v["dark"]))
            };
            (t["name"].as_str().expect("a name").to_string(), light, dark)
        })
        .collect()
});

/// The nearest xterm colour among 16-255 (the 6×6×6 cube and the grey ramp; 0-15 vary by terminal), by squared
/// RGB distance.
pub fn nearest_256((r, g, b): (u8, u8, u8)) -> u8 {
    const LEVELS: [i32; 6] = [0, 95, 135, 175, 215, 255];
    let dist = |c: (i32, i32, i32)| {
        (c.0 - r as i32).pow(2) + (c.1 - g as i32).pow(2) + (c.2 - b as i32).pow(2)
    };
    (16..=255u8)
        .min_by_key(|&i| {
            let c = if i < 232 {
                let n = (i - 16) as usize;
                (LEVELS[n / 36], LEVELS[n / 6 % 6], LEVELS[n % 6])
            } else {
                let v = 8 + 10 * (i - 232) as i32;
                (v, v, v)
            };
            dist(c)
        })
        .expect("a non-empty range")
}

/// `COLORFGBG` is `fg;bg` (or `fg;default;bg`); background 7 or 9-15 is light. Anything else, or no variable, is dark.
fn theme(colorfgbg: Option<String>) -> Theme {
    let bg = colorfgbg.and_then(|v| v.rsplit(';').next()?.parse::<u8>().ok());
    match bg {
        Some(7 | 9..=15) => Theme::Light,
        _ => Theme::Dark,
    }
}

/// A UTF-8 locale (the first set of `LC_ALL`, `LC_CTYPE`, `LANG`) on anything but the Linux console, or Windows
/// Terminal.
fn unicode(var: &dyn Fn(&str) -> Option<String>) -> bool {
    if var("WT_SESSION").is_some() {
        return true;
    }
    if var("TERM").as_deref() == Some("linux") {
        return false;
    }
    let locale = ["LC_ALL", "LC_CTYPE", "LANG"]
        .iter()
        .find_map(|k| var(k))
        .unwrap_or_default()
        .to_ascii_lowercase();
    locale.contains("utf-8") || locale.contains("utf8")
}

/// Terminals known to support OSC 8; `FORCE_HYPERLINK` (non-zero, or `0`) overrides the guess.
fn hyperlinks(var: &dyn Fn(&str) -> Option<String>, term: &str) -> bool {
    if let Some(force) = var("FORCE_HYPERLINK") {
        return force != "0";
    }
    let program = var("TERM_PROGRAM").unwrap_or_default();
    matches!(
        program.as_str(),
        "iTerm.app" | "WezTerm" | "vscode" | "ghostty" | "Hyper"
    ) || var("WT_SESSION").is_some()
        || var("KONSOLE_VERSION").is_some()
        || var("VTE_VERSION").is_some_and(|v| v.parse::<u32>().is_ok_and(|n| n >= 5000))
        || matches!(term, "xterm-kitty" | "xterm-ghostty" | "alacritty" | "foot")
}
