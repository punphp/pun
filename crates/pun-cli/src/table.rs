//! The two pieces of Symfony's console style that PHPStan's `table` error format prints, undecorated: PHPStan's
//! `ErrorsConsoleStyle::table` (a two-column `symfony-style-guide` table whose second column wraps at the terminal
//! width) and `SymfonyStyle::block` (`[OK] No errors`, `[ERROR] Found N errors`).

/// Symfony's SymfonyStyle::MAX_LINE_LENGTH.
const MAX_LINE_LENGTH: usize = 120;

pub(crate) struct Style {
    /// The terminal width (Symfony's Terminal::getWidth()).
    width: usize,
}

/// mb_strwidth: 2 for East Asian wide and fullwidth characters (and the emoji PHPStan prints), 1 otherwise.
pub(crate) fn width(text: &str) -> usize {
    text.chars().map(char_width).sum()
}

fn char_width(c: char) -> usize {
    let c = c as u32;
    let wide = matches!(c,
        0x1100..=0x115F
        | 0x2E80..=0x303E
        | 0x3041..=0x33FF
        | 0x3400..=0x4DBF
        | 0x4E00..=0x9FFF
        | 0xA000..=0xA4CF
        | 0xAC00..=0xD7A3
        | 0xF900..=0xFAFF
        | 0xFE30..=0xFE4F
        | 0xFF00..=0xFF60
        | 0xFFE0..=0xFFE6
        | 0x1F300..=0x1F64F
        | 0x1F900..=0x1F9FF
        | 0x1FA70..=0x1FAFF
        | 0x20000..=0x2FFFD
        | 0x30000..=0x3FFFD);
    if wide { 2 } else { 1 }
}

/// PHP's wordwrap($text, $width, "\n", true) on characters, one line at a time.
fn wrap(line: &str, max: usize) -> Vec<String> {
    if max == 0 || width(line) <= max {
        return vec![line.to_string()];
    }
    let mut out = vec![];
    let mut current = String::new();
    for word in line.split(' ') {
        let candidate = if current.is_empty() {
            word.to_string()
        } else {
            format!("{current} {word}")
        };
        if width(&candidate) <= max {
            current = candidate;
            continue;
        }
        if !current.is_empty() {
            out.push(std::mem::take(&mut current));
        }
        // A word longer than the line is cut.
        let mut piece = String::new();
        for c in word.chars() {
            if width(&piece) + char_width(c) > max {
                out.push(std::mem::take(&mut piece));
            }
            piece.push(c);
        }
        current = piece;
    }
    out.push(current);
    out
}

/// `text` padded with spaces to `columns` display columns.
fn pad(text: &str, columns: usize) -> String {
    format!("{text}{}", " ".repeat(columns.saturating_sub(width(text))))
}

impl Style {
    pub(crate) fn new(width: usize) -> Style {
        Style { width }
    }

    /// SymfonyStyle's line length: the terminal width, at most 120.
    fn line_length(&self) -> usize {
        self.width.min(MAX_LINE_LENGTH)
    }

    /// ErrorsConsoleStyle::table, undecorated: the header row, a separator, the rows (a cell's lines on lines of
    /// their own), a newline after the bottom border. The second column wraps at the terminal width less the first
    /// column and five spaces of padding.
    pub(crate) fn table(&self, header: [&str; 2], rows: &[[String; 2]]) -> String {
        let terminal = self.width.saturating_sub(2);
        let first = rows
            .iter()
            .map(|r| width(&r[0]))
            .fold(header[0].len(), usize::max);
        let max = terminal.saturating_sub(first + 5);
        let mut lines: Vec<[String; 2]> = vec![[header[0].into(), header[1].into()]];
        let separator = lines.len();
        for [a, b] in rows {
            let wrapped: Vec<String> = b.split('\n').flat_map(|l| wrap(l, max)).collect();
            for (i, line) in wrapped.into_iter().enumerate() {
                lines.push([if i == 0 { a.clone() } else { String::new() }, line]);
            }
        }
        let widths = [0, 1].map(|c| {
            let w = lines.iter().map(|l| width(&l[c])).max().unwrap_or(0);
            if c == 1 { w.min(max.max(1)) } else { w }
        });
        let border = format!(
            " {} {} \n",
            "-".repeat(widths[0] + 2),
            "-".repeat(widths[1] + 2)
        );
        let mut out = border.clone();
        for (i, [a, b]) in lines.iter().enumerate() {
            if i == separator {
                out.push_str(&border);
            }
            out.push_str(&format!(
                "  {}   {}  \n",
                pad(a, widths[0]),
                pad(b, widths[1])
            ));
        }
        out.push_str(&border);
        out.push('\n');
        out
    }

    /// SymfonyStyle::block with a type and padding, undecorated: a blank line before it (one more after a table,
    /// which ends with one), ` [TYPE] message` padded to the line length, then a blank line.
    pub(crate) fn block(&self, kind: &str, message: &str) -> String {
        let line = format!(" [{kind}] {message}");
        format!("\n{}\n\n", pad(&line, self.line_length()))
    }
}
