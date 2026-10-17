//! nunomaduro/collision's Highlighter as Pint uses it in `-v` output: the code snippet under a syntax error
//! (Issue::code) and the diff of a fixed file (Issue::diff). Colours only when Collision thinks stdout is a TTY.

use pun_tokens::Kind::*;

#[derive(Clone, Copy, PartialEq)]
enum Ty {
    Default,
    Comment,
    Str,
    Html,
    Keyword,
}

impl Ty {
    /// Highlighter::THEME (ConsoleColor codes).
    fn codes(self) -> &'static str {
        match self {
            Ty::Str => "37",
            Ty::Comment => "90;3",
            Ty::Keyword => "35;1",
            Ty::Default => "39;1",
            Ty::Html => "34;1",
        }
    }
}

const ACTUAL_LINE_MARK: &str = "31;1";
const LINE_NUMBER: &str = "90";
const MARKED_LINE_NUMBER: &str = "3;1";
const LINE_DIVIDER: &str = "90";

fn apply(colors: bool, codes: &str, text: &str) -> String {
    if colors {
        format!("\x1b[{codes}m{text}\x1b[0m")
    } else {
        text.to_string()
    }
}

/// Highlighter::tokenize: token_get_all, adjacent tokens of one type merged; whitespace joins the current run.
fn tokenize(source: &str) -> Vec<(Ty, String)> {
    let mut out = vec![];
    let (mut current, mut new): (Option<Ty>, Option<Ty>) = (None, None);
    let mut buffer = String::new();
    for t in pun_tokens::tokenize(source.as_bytes()) {
        match t.kind {
            Some(T_WHITESPACE) => {}
            Some(
                T_OPEN_TAG | T_OPEN_TAG_WITH_ECHO | T_CLOSE_TAG | T_STRING | T_VARIABLE | T_DIR
                | T_FILE | T_METHOD_C | T_DNUMBER | T_LNUMBER | T_NS_C | T_LINE | T_CLASS_C
                | T_FUNC_C | T_TRAIT_C,
            ) => new = Some(Ty::Default),
            Some(T_COMMENT | T_DOC_COMMENT) => new = Some(Ty::Comment),
            Some(T_ENCAPSED_AND_WHITESPACE | T_CONSTANT_ENCAPSED_STRING) => new = Some(Ty::Str),
            Some(T_INLINE_HTML) => new = Some(Ty::Html),
            Some(_) => new = Some(Ty::Keyword),
            None => {
                new = Some(if t.text == b"\"" {
                    Ty::Str
                } else {
                    Ty::Keyword
                })
            }
        }
        if current.is_none() {
            current = new;
        }
        if current != new {
            out.push((current.unwrap(), std::mem::take(&mut buffer)));
            current = new;
        }
        buffer.push_str(&String::from_utf8_lossy(t.text));
    }
    if let Some(ty) = new {
        out.push((ty, buffer));
    }
    out
}

/// getHighlightedLines + colorLines: one string per source line.
fn lines(source: &str, colors: bool) -> Vec<String> {
    let source = source.replace("\r\n", "\n").replace('\r', "\n");
    let mut lines = vec![];
    let mut line = String::new();
    for (ty, text) in tokenize(&source) {
        for (n, part) in text.split('\n').enumerate() {
            if n > 0 {
                lines.push(std::mem::take(&mut line));
            }
            if !part.is_empty() {
                line.push_str(&apply(colors, ty.codes(), part));
            }
        }
    }
    lines.push(line);
    lines
}

/// Highlighter::highlight($content, $line): four lines either side, line numbers, `➜` on the error line.
pub fn snippet(source: &str, mark: usize, colors: bool) -> String {
    let all = lines(source, colors);
    let offset = mark.saturating_sub(5);
    let shown: Vec<(usize, &String)> = all.iter().enumerate().skip(offset).take(9).collect();
    let width = shown
        .last()
        .map_or(1, |(i, _)| (i + 1).to_string().len())
        .max(3);
    let mut out = String::new();
    for (i, line) in shown {
        let marked = mark == i + 1;
        out.push_str(&if marked {
            apply(colors, ACTUAL_LINE_MARK, "  ➜ ")
        } else {
            "    ".to_string()
        });
        out.push_str(&apply(
            colors,
            if marked {
                MARKED_LINE_NUMBER
            } else {
                LINE_NUMBER
            },
            &format!("{:>width$}", i + 1),
        ));
        out.push_str(&apply(colors, LINE_DIVIDER, "▕ "));
        out.push_str(line);
        out.push('\n');
    }
    out.trim_end_matches([' ', '\t', '\n', '\r', '\0', '\x0b'])
        .to_string()
}

/// Issue::diff: `+`/`-` lines wrapped as `//+<fg=green>…</>` comments, highlighted after a `<?php` line, header
/// dropped, the comment marker stripped again. Returns Symfony formatter markup.
pub fn diff(diff: &str, colors: bool) -> String {
    let marked: Vec<String> = diff
        .split('\n')
        .map(|l| match l.as_bytes().first() {
            Some(b'+') => format!("//+<fg=green>{l}</>"),
            Some(b'-') => format!("//-<fg=red>{l}</>"),
            _ => l.to_string(),
        })
        .collect();
    let lines: Vec<String> = lines(&format!("<?php\n{}", marked.join("\n")), colors)
        .into_iter()
        .skip(3)
        .map(|l| {
            for p in ["\x1b[90;3m//-", "//-", "\x1b[90;3m//+", "//+"] {
                if l.starts_with(p) {
                    return l.replacen(p, "", 1);
                }
            }
            l
        })
        .collect();
    format!("  {}", lines.join("\n  "))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `[highlight(SRC, 3), highlight(SRC, 13), highlight("", 1), Issue::code() for DIFF]` from .oracle/pint's vendor
    /// (Collision's Highlighter and Pint's App\ValueObjects\Issue), without and with COLLISION_FORCE_COLORS.
    const SRC: &str = "<?php\r\n// c\n$a = \"x$b\" . 'y';\n?>\nhtml <b>\n<?= __DIR__ ?>\n/** d */\nfunction (\n\n\n\n\n\n\n";
    const DIFF: &str = "--- /p/a.php\n+++ /p/a.php\n@@ -1,3 +1,3 @@\n <?php\n-$a = array( 1 );\n+$a = [1];\n /*\n-x\n+y\n */\n";
    const PLAIN: &str = r#"["      1\u2595 <?php\n      2\u2595 \/\/ c\n  \u279c   3\u2595 $a = \"x$b\" . 'y';\n      4\u2595 ?>\n      5\u2595 html <b>\n      6\u2595 <?= __DIR__ ?>\n      7\u2595 \/** d *\/\n      8\u2595 function (\n      9\u2595","      9\u2595 \n     10\u2595 \n     11\u2595 \n     12\u2595 \n  \u279c  13\u2595 \n     14\u2595 \n     15\u2595","  \u279c   1\u2595","  @@ -1,3 +1,3 @@\n   <?php\n  <fg=red>-$a = array( 1 );<\/>\n  <fg=green>+$a = [1];<\/>\n   \/*\n  <fg=red>-x<\/>\n  <fg=green>+y<\/>\n   *\/\n  "]"#;
    const COLOR: &str = r#"["    \u001b[90m  1\u001b[0m\u001b[90m\u2595 \u001b[0m\u001b[39;1m<?php\u001b[0m\n    \u001b[90m  2\u001b[0m\u001b[90m\u2595 \u001b[0m\u001b[90;3m\/\/ c\u001b[0m\n\u001b[31;1m  \u279c \u001b[0m\u001b[3;1m  3\u001b[0m\u001b[90m\u2595 \u001b[0m\u001b[39;1m$a \u001b[0m\u001b[35;1m= \u001b[0m\u001b[37m\"x\u001b[0m\u001b[39;1m$b\u001b[0m\u001b[37m\" \u001b[0m\u001b[35;1m. \u001b[0m\u001b[37m'y'\u001b[0m\u001b[35;1m;\u001b[0m\n    \u001b[90m  4\u001b[0m\u001b[90m\u2595 \u001b[0m\u001b[39;1m?>\u001b[0m\n    \u001b[90m  5\u001b[0m\u001b[90m\u2595 \u001b[0m\u001b[34;1mhtml <b>\u001b[0m\n    \u001b[90m  6\u001b[0m\u001b[90m\u2595 \u001b[0m\u001b[39;1m<?= __DIR__ ?>\u001b[0m\n    \u001b[90m  7\u001b[0m\u001b[90m\u2595 \u001b[0m\u001b[34;1m\/** d *\/\u001b[0m\n    \u001b[90m  8\u001b[0m\u001b[90m\u2595 \u001b[0m\u001b[34;1mfunction (\u001b[0m\n    \u001b[90m  9\u001b[0m\u001b[90m\u2595 \u001b[0m","    \u001b[90m  9\u001b[0m\u001b[90m\u2595 \u001b[0m\n    \u001b[90m 10\u001b[0m\u001b[90m\u2595 \u001b[0m\n    \u001b[90m 11\u001b[0m\u001b[90m\u2595 \u001b[0m\n    \u001b[90m 12\u001b[0m\u001b[90m\u2595 \u001b[0m\n\u001b[31;1m  \u279c \u001b[0m\u001b[3;1m 13\u001b[0m\u001b[90m\u2595 \u001b[0m\n    \u001b[90m 14\u001b[0m\u001b[90m\u2595 \u001b[0m\n    \u001b[90m 15\u001b[0m\u001b[90m\u2595 \u001b[0m","\u001b[31;1m  \u279c \u001b[0m\u001b[3;1m  1\u001b[0m\u001b[90m\u2595 \u001b[0m","  \u001b[35;1m@@ -\u001b[0m\u001b[39;1m1\u001b[0m\u001b[35;1m,\u001b[0m\u001b[39;1m3 \u001b[0m\u001b[35;1m+\u001b[0m\u001b[39;1m1\u001b[0m\u001b[35;1m,\u001b[0m\u001b[39;1m3 \u001b[0m\u001b[35;1m@@\u001b[0m\n  \u001b[35;1m <?\u001b[0m\u001b[39;1mphp\u001b[0m\n  <fg=red>-$a = array( 1 );<\/>\u001b[0m\n  <fg=green>+$a = [1];<\/>\u001b[0m\n  \u001b[90;3m \/*\u001b[0m\n  <fg=red>-x<\/>\u001b[0m\n  <fg=green>+y<\/>\u001b[0m\n  \u001b[90;3m *\/\u001b[0m\n  "]"#;

    #[test]
    fn like_collision() {
        for (colors, oracle) in [(false, PLAIN), (true, COLOR)] {
            let want: Vec<String> = serde_json::from_str(oracle).unwrap();
            assert_eq!(
                [
                    snippet(SRC, 3, colors),
                    snippet(SRC, 13, colors),
                    snippet("", 1, colors),
                    diff(DIFF, colors)
                ],
                want[..],
                "colors={colors}"
            );
        }
    }
}
