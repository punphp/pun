//! The sections of rules/mapping.toml, each parsed on its own the first time it is asked for: a `pun format` run
//! needs the `[upstream.*]` fixer sections only, and the whole file is ten times bigger.

use crate::MAPPING;
use std::sync::{LazyLock, OnceLock};

/// One `[a.b]` table of mapping.toml.
pub(crate) struct Section {
    /// The header's keys, unquoted: `["options", "format/arrays/syntax"]`.
    pub path: Vec<String>,
    /// The 1-based line of the header.
    pub line: usize,
    /// The text between the header and the next one.
    body: &'static str,
    table: OnceLock<toml::Table>,
}

impl Section {
    /// The section's entries, in file order.
    pub fn table(&self) -> &toml::Table {
        self.table
            .get_or_init(|| toml::from_str(self.body).expect("rules/mapping.toml"))
    }

    /// The 1-based line of the entry `key` (`"arrays/syntax" = ...`).
    pub fn line_of(&self, key: &str) -> Option<usize> {
        let entry = format!("\"{key}\" = ");
        let i = self.body.lines().position(|l| l.starts_with(&entry))?;
        Some(self.line + 1 + i)
    }
}

/// The keys of a table header line (`[options."format/x"]`), or `None` for any other line. A line counts as a header
/// only when TOML reads it as one, so an array's `[...]` row at the start of a line is not.
fn header(line: &str) -> Option<Vec<String>> {
    if !line.starts_with('[') {
        return None;
    }
    let mut t: toml::Table = toml::from_str(line).ok()?;
    let mut path = vec![];
    while let Some((k, toml::Value::Table(next))) = t.into_iter().next() {
        path.push(k);
        t = next;
    }
    Some(path)
}

/// Every section, in file order.
pub(crate) fn sections() -> &'static [Section] {
    static T: LazyLock<Vec<Section>> = LazyLock::new(|| {
        let mut out: Vec<Section> = vec![];
        let mut at = 0;
        for (i, line) in MAPPING.split_inclusive('\n').enumerate() {
            if let Some(path) = header(line.trim_end()) {
                if let Some(last) = out.last_mut() {
                    last.body = &MAPPING[MAPPING.len() - last.body.len()..at];
                }
                out.push(Section {
                    path,
                    line: i + 1,
                    body: &MAPPING[at + line.len()..],
                    table: OnceLock::new(),
                });
            }
            at += line.len();
        }
        out
    });
    &T
}

/// The section whose header is `path` (`["upstream", "pint"]`).
pub(crate) fn section(path: &[&str]) -> &'static Section {
    sections()
        .iter()
        .find(|s| s.path == path)
        .expect("rules/mapping.toml section")
}

/// The sections directly under `parent` (`upstream`), in file order, with the key that names each one.
pub(crate) fn under(parent: &str) -> impl Iterator<Item = (&'static str, &'static Section)> {
    sections().iter().filter_map(move |s| match &s.path[..] {
        [p, k] if p == parent => Some((k.as_str(), s)),
        _ => None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn headers() {
        let path = |p: &[&str]| Some(p.iter().map(ToString::to_string).collect::<Vec<_>>());
        assert_eq!(header("[meta]"), path(&["meta"]));
        assert_eq!(
            header("[upstream.php-cs-fixer]"),
            path(&["upstream", "php-cs-fixer"])
        );
        assert_eq!(
            header(r#"[options."format/arrays/syntax"]"#),
            path(&["options", "format/arrays/syntax"])
        );
        // A row of a multi-line array that starts the line, and an indented one: values, not headers.
        assert_eq!(
            header(r#"["format/imports/group", "format/imports/single-per-statement"],"#),
            None
        );
        assert_eq!(header("  [meta]"), None);
        assert_eq!(header("x = 1"), None);
    }

    /// Each section holds its own entries only (lines and keys read off rules/mapping.toml).
    #[test]
    fn sections_split_at_headers() {
        let meta = section(&["meta"]);
        assert!(meta.table().contains_key("conflicts"));
        assert!(!meta.table().contains_key("arrays"));
        let groups = section(&["meta", "groups"]);
        assert_eq!(groups.line, meta.line + meta.body.lines().count() + 1);
        assert!(groups.table().contains_key("arrays"));
        let format = section(&["rules", "format"]);
        let line = format.line_of("arrays/syntax").unwrap();
        assert!(
            MAPPING
                .lines()
                .nth(line - 1)
                .unwrap()
                .starts_with("\"arrays/syntax\" = ")
        );
        assert_eq!(format.line_of("nope/nope"), None);
        let tools: Vec<&str> = under("upstream").map(|(t, _)| t).take(3).collect();
        assert_eq!(tools, ["php-cs-fixer", "pint", "pun"]);
    }
}
