//! The bits of Symfony Console that shape Pint's bytes: colour decision (StreamOutput::hasColorSupport,
//! `--ansi`/`--no-ansi`), terminal width (Terminal::getWidth) and the `<fg=red>…</>` OutputFormatter.

use pcre2::bytes::Regex;
use std::io::IsTerminal;
use std::sync::LazyLock;

/// The first of the 256 code points (plane 16's private use area) that carry the bytes of a non-UTF-8 diff through
/// Pun's `String` output until `raw` writes them out, as PHP's byte strings print them.
const CARRIED: u32 = 0x10_FF00;

/// `bytes` as text: UTF-8 stays as is, every other byte becomes its carrier code point.
pub(crate) fn carry(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len());
    for chunk in bytes.utf8_chunks() {
        s.push_str(chunk.valid());
        s.extend((chunk.invalid().iter()).filter_map(|&b| char::from_u32(CARRIED + u32::from(b))));
    }
    s
}

/// Whether `text` carries a byte that isn't UTF-8.
pub(crate) fn carries_bytes(text: &str) -> bool {
    text.chars()
        .any(|c| (CARRIED..=CARRIED + 0xFF).contains(&u32::from(c)))
}

/// The bytes `text` stands for: its carriers written back as the bytes they carry.
pub(crate) fn raw(text: &str) -> Vec<u8> {
    let mut out = Vec::with_capacity(text.len());
    let mut buf = [0; 4];
    for c in text.chars() {
        match u32::from(c).checked_sub(CARRIED) {
            Some(b) if b <= 0xFF => out.push(b as u8),
            _ => out.extend_from_slice(c.encode_utf8(&mut buf).as_bytes()),
        }
    }
    out
}

/// `print!` for text that may carry raw bytes.
pub(crate) fn print_raw(text: &str) -> std::io::Result<()> {
    std::io::Write::write_all(&mut std::io::stdout().lock(), &raw(text))
}

/// Whether stdout (fd 1) / stderr (fd 2) get ANSI styles, as Application::configureIO + ConsoleOutput decide.
pub(crate) fn decorated(fd: i32, ansi: bool, no_ansi: bool) -> bool {
    if ansi {
        return true;
    }
    if no_ansi {
        return false;
    }
    let tty = if fd == 1 {
        std::io::stdout().is_terminal()
    } else {
        std::io::stderr().is_terminal()
    };
    has_color_support(tty, &|k| std::env::var(k).ok())
}

/// StreamOutput::hasColorSupport (non-Windows).
fn has_color_support(tty: bool, env: &dyn Fn(&str) -> Option<String>) -> bool {
    let set = |k: &str| env(k).is_some_and(|v| !v.is_empty());
    if set("NO_COLOR") {
        return false;
    }
    if set("FORCE_COLOR") {
        return true;
    }
    let msys = env("MSYSTEM")
        .is_some_and(|v| matches!(v.to_ascii_uppercase().as_str(), "MINGW32" | "MINGW64"));
    if !tty && !msys {
        return false;
    }
    if env("TERM_PROGRAM").as_deref() == Some("Hyper")
        || env("COLORTERM").is_some()
        || env("ANSICON").is_some()
        || env("ConEmuANSI").as_deref() == Some("ON")
    {
        return true;
    }
    let term = env("TERM").unwrap_or_default();
    if term == "dumb" {
        return false;
    }
    static TERM_RE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"^((screen|xterm|vt100|vt220|putty|rxvt|ansi|cygwin|linux).*)|(.*-256(color)?(-bce)?)$").unwrap()
    });
    TERM_RE.is_match(term.as_bytes()).unwrap_or(false)
}

/// Terminal::getWidth: `COLUMNS` (as PHP's `(int)`), else `stty -a` on the inherited stdin, else 80.
pub(crate) fn width() -> usize {
    if let Ok(c) = std::env::var("COLUMNS") {
        return php_int(c.trim());
    }
    let stty = std::process::Command::new("stty")
        .arg("-a")
        .stdin(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::null())
        .output();
    let text = stty
        .map(|o| String::from_utf8_lossy(&o.stdout).into_owned())
        .unwrap_or_default();
    static RE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"(?is)rows.(\d+);.columns.(\d+);|;.(\d+).rows;.(\d+).columns").unwrap()
    });
    let cols = RE.captures(text.as_bytes()).ok().flatten().and_then(|c| {
        c.get(2)
            .or(c.get(4))
            .map(|m| php_int(&String::from_utf8_lossy(m.as_bytes())))
    });
    match cols {
        Some(w) if w > 0 => w,
        _ => 80,
    }
}

/// PHP `(int) "12abc"` = 12, `(int) "abc"` = 0 (non-negative only; widths are never negative).
fn php_int(s: &str) -> usize {
    s.bytes()
        .take_while(u8::is_ascii_digit)
        .fold(0usize, |n, d| {
            n.saturating_mul(10).saturating_add(usize::from(d - b'0'))
        })
}

/// OutputFormatter::escape.
pub(crate) fn escape(text: &str) -> String {
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"([^\\]|^)([<>])").unwrap());
    let mut out = Vec::with_capacity(text.len() + 8);
    let mut last = 0;
    let b = text.as_bytes();
    for c in RE.captures_iter(b).flatten() {
        let (g1, g2) = (c.get(1).unwrap(), c.get(2).unwrap());
        out.extend_from_slice(&b[last..g1.end()]);
        out.push(b'\\');
        out.extend_from_slice(g2.as_bytes());
        last = g2.end();
    }
    out.extend_from_slice(&b[last..]);
    // escapeTrailingBackslash: trailing backslashes become NULs, which format() turns back into backslashes.
    let n = out.iter().rev().take_while(|&&c| c == b'\\').count();
    if n > 0 {
        let len = out.len();
        out.truncate(len - n);
        out.retain(|&c| c != 0);
        out.resize(len, 0);
    }
    String::from_utf8(out).expect("utf-8 in, utf-8 out")
}

#[derive(Clone, PartialEq, Eq)]
struct Style {
    set: String,
    unset: String,
}

impl Style {
    fn new(fg: Option<&str>, bg: Option<&str>, options: &[&str]) -> Option<Style> {
        let color = |c: &str, base: u8| -> Option<String> {
            const COLORS: [&str; 8] = [
                "black", "red", "green", "yellow", "blue", "magenta", "cyan", "white",
            ];
            const BRIGHT: [&str; 8] = [
                "gray",
                "bright-red",
                "bright-green",
                "bright-yellow",
                "bright-blue",
                "bright-magenta",
                "bright-cyan",
                "bright-white",
            ];
            if c == "default" {
                return Some(format!("{base}9"));
            }
            // ponytail: `#rgb` colours and unknown names throw in Symfony (Pint crashes); we print the tag literally.
            COLORS
                .iter()
                .position(|&n| n == c)
                .map(|i| format!("{base}{i}"))
                .or_else(|| {
                    BRIGHT
                        .iter()
                        .position(|&n| n == c)
                        .map(|i| format!("{}{i}", base + 6))
                })
        };
        let (mut set, mut unset) = (vec![], vec![]);
        if let Some(f) = fg {
            set.push(color(f, 3)?);
            unset.push("39".to_string());
        }
        if let Some(b) = bg {
            set.push(color(b, 4)?);
            unset.push("49".to_string());
        }
        let mut seen: Vec<&str> = vec![];
        for &o in options {
            let (s, u) = match o {
                "bold" => (1, 22),
                "underscore" => (4, 24),
                "blink" => (5, 25),
                "reverse" => (7, 27),
                "conceal" => (8, 28),
                _ => return None,
            };
            if !seen.contains(&o) {
                seen.push(o);
                set.push(s.to_string());
                unset.push(u.to_string());
            }
        }
        let esc = |codes: Vec<String>| {
            if codes.is_empty() {
                String::new()
            } else {
                format!("\x1b[{}m", codes.join(";"))
            }
        };
        Some(Style {
            set: esc(set),
            unset: esc(unset),
        })
    }

    /// OutputFormatter::createStyleFromString.
    fn parse(tag: &str) -> Option<Style> {
        match tag {
            "error" => return Style::new(Some("white"), Some("red"), &[]),
            "info" => return Style::new(Some("green"), None, &[]),
            "comment" => return Style::new(Some("yellow"), None, &[]),
            "question" => return Style::new(Some("black"), Some("cyan"), &[]),
            _ => {}
        }
        static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"([^=]+)=([^;]+)(;|$)").unwrap());
        let (mut fg, mut bg, mut options) = (None, None, String::new());
        let mut any = false;
        for c in RE.captures_iter(tag.as_bytes()).flatten() {
            any = true;
            let s = |i| String::from_utf8_lossy(c.get(i).unwrap().as_bytes()).to_ascii_lowercase();
            match s(1).as_str() {
                "fg" => fg = Some(s(2)),
                "bg" => bg = Some(s(2)),
                "options" => options = s(2),
                // ponytail: href= (hyperlinks) is never emitted by Pint; treated like an unknown key.
                _ => return None,
            }
        }
        let opts: Vec<&str> = options
            .split([',', ';'])
            .filter(|o| !o.is_empty())
            .collect();
        if any {
            Style::new(fg.as_deref(), bg.as_deref(), &opts)
        } else {
            None
        }
    }
}

/// OutputFormatter::format: `<fg=..;bg=..;options=..>`, `<info>` etc. become ANSI (or vanish when not decorated);
/// unknown tags stay as text; `\<` is an escaped `<`.
pub(crate) fn format(message: &str, decorated: bool) -> String {
    static TAG: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"(?i)<(([a-z](?:[^\\<>]*+|\\.)*)|/([a-z][^<>]*+)?)>").unwrap()
    });
    let b = message.as_bytes();
    // ponytail: Symfony keeps one style stack per output across writes; Pint never leaves a tag open, so one per call.
    let mut stack: Vec<Style> = vec![];
    let mut out = String::with_capacity(message.len());
    let apply = |out: &mut String, stack: &[Style], text: &str| match stack.last() {
        Some(s) if decorated && !text.is_empty() => {
            out.push_str(&s.set);
            out.push_str(text);
            out.push_str(&s.unset);
        }
        _ => out.push_str(text),
    };
    let mut offset = 0;
    for c in TAG.captures_iter(b).flatten() {
        let m = c.get(0).unwrap();
        if m.start() != 0 && b[m.start() - 1] == b'\\' {
            continue;
        }
        apply(&mut out, &stack, &message[offset..m.start()]);
        offset = m.end();
        let text = &message[m.start()..m.end()];
        let open = !text.starts_with("</");
        let tag = c
            .get(if open { 2 } else { 3 })
            .map_or("", |t| &message[t.start()..t.end()]);
        if !open && tag.is_empty() {
            stack.pop();
        } else if let Some(style) = Style::parse(tag) {
            if open {
                stack.push(style);
            } else if let Some(i) = stack.iter().rposition(|s| *s == style) {
                // ponytail: an unmatched close tag throws in Symfony; ignored here.
                stack.truncate(i);
            }
        } else {
            apply(&mut out, &stack, text);
        }
    }
    apply(&mut out, &stack, &message[offset..]);
    strtr(&out)
}

/// `strtr($output, ["\0" => '\\', '\\<' => '<', '\\>' => '>'])`.
fn strtr(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut it = s.chars().peekable();
    while let Some(c) = it.next() {
        match c {
            '\0' => out.push('\\'),
            '\\' if matches!(it.peek(), Some('<' | '>')) => out.push(it.next().unwrap()),
            c => out.push(c),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    // Expectations: Symfony Console 7 (`.oracle/pint/vendor/symfony/console`) run on the same input, e.g.
    // php -r 'require ".oracle/pint/vendor/autoload.php"; echo (new Symfony\Component\Console\Formatter\OutputFormatter(true))->format("…");'
    #[test]
    fn formats_like_symfony() {
        for (input, want) in [
            ("<options=bold;fg=red>!</>", "\x1b[31;1m!\x1b[39;22m"),
            (
                "<bg=green;fg=gray;options=bold>  PASS  </>",
                "\x1b[90;42;1m  PASS  \x1b[39;49;22m",
            ),
            (
                "<fg=red;options=bold><options=bold;fg=green>✓</></>",
                "\x1b[32;1m✓\x1b[39;22m",
            ),
            (
                "  <error>ERROR</error> a<b>: \\<info>",
                "  \x1b[37;41mERROR\x1b[39;49m a<b>: <info>",
            ),
            (
                "<comment>x</comment><question>q</question><info>i</info>",
                "\x1b[33mx\x1b[39m\x1b[30;46mq\x1b[39;49m\x1b[32mi\x1b[39m",
            ),
            (
                "<fg=cyan>a<fg=default>b</fg=default>c</fg=cyan>d</>",
                "\x1b[36ma\x1b[39m\x1b[39mb\x1b[39m\x1b[36mc\x1b[39md",
            ),
            (
                "<options=underscore,blink,reverse,conceal,bold,bold>u</>",
                "\x1b[4;5;7;8;1mu\x1b[24;25;27;28;22m",
            ),
            (
                "<INFO>y</INFO> <fg=red;bg>w</>",
                "<INFO>y</INFO> \x1b[31mw\x1b[39m",
            ),
            ("trail\0\0", "trail\\\\"),
            ("a<options=,>b</>c", "abc"),
        ] {
            assert_eq!(format(input, true), want, "{input:?}");
        }
        assert_eq!(
            format("<options=bold;fg=red>!</> <fg=gray>x</> \\<b> <x=y>", false),
            "! x <b> <x=y>"
        );
    }

    /// Hand-derived: Symfony throws on these (Pint would crash); Pun prints the tag as text instead.
    #[test]
    fn unsupported_styles_stay_text() {
        assert_eq!(
            format(
                "<fg=nope>x</> <bg=nope>y</> <options=wavy>z</> <href=http://x>h</>",
                true
            ),
            "<fg=nope>x <bg=nope>y <options=wavy>z <href=http://x>h"
        );
        assert_eq!(
            format("<fg=red>a</fg=blue>b</>", true),
            "\x1b[31ma\x1b[39m\x1b[31mb\x1b[39m"
        );
    }

    #[test]
    fn escapes_like_symfony() {
        // OutputFormatter::escape("…") under PHP.
        assert_eq!(escape("<?xml a=\"<b>\"?>"), "\\<?xml a=\"\\<b\\>\"?\\>");
        assert_eq!(escape("a<<b \\<c>"), "a\\<<b \\<c\\>");
        assert_eq!(escape("x\\\\"), "x\0\0");
        assert_eq!(format(&escape("<a> x\\"), true), "<a> x\\");
    }

    #[test]
    fn color_support_like_symfony() {
        let env = |pairs: &'static [(&'static str, &'static str)]| {
            move |k: &str| pairs.iter().find(|p| p.0 == k).map(|p| p.1.to_string())
        };
        assert!(!has_color_support(
            true,
            &env(&[("TERM", "xterm"), ("NO_COLOR", "1")])
        ));
        assert!(has_color_support(
            false,
            &env(&[("NO_COLOR", ""), ("FORCE_COLOR", "1")])
        ));
        assert!(!has_color_support(false, &env(&[("TERM", "xterm")])));
        assert!(has_color_support(
            false,
            &env(&[("MSYSTEM", "mingw64"), ("COLORTERM", "")])
        ));
        assert!(has_color_support(
            true,
            &env(&[("TERM_PROGRAM", "Hyper"), ("TERM", "dumb")])
        ));
        assert!(has_color_support(true, &env(&[("ANSICON", "1")])));
        assert!(has_color_support(true, &env(&[("ConEmuANSI", "ON")])));
        assert!(!has_color_support(true, &env(&[("TERM", "dumb")])));
        assert!(has_color_support(true, &env(&[("TERM", "xterm-kitty")])));
        assert!(has_color_support(true, &env(&[("TERM", "foot-256color")])));
        assert!(!has_color_support(true, &env(&[("TERM", "foot")])));
        assert!(!has_color_support(true, &env(&[])));
        assert_eq!(
            (php_int("120"), php_int("12abc"), php_int("abc")),
            (120, 12, 0)
        );
    }
}
