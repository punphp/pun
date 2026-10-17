//! PHP-CS-Fixer's checkstyle, junit, xml and gitlab reporters (src/Console/Report/FixReport), byte for byte:
//! DOMDocument::saveXML with formatOutput, json_encode defaults, md5 fingerprints.

use crate::console::escape;
use crate::report::{ABOUT, php_json};
use std::fmt::Write as _;
use std::sync::LazyLock;

/// One changed file as the reporters see it: `diff` is empty unless `-v` (Pint uses a NullDiffer otherwise).
pub struct Change<'a> {
    pub file: &'a str,
    pub fixers: &'a [&'static str],
    pub diff: &'a str,
}

/// A libxml element: attributes, then either child elements or one CDATA section.
struct El {
    name: &'static str,
    attrs: Vec<(&'static str, String)>,
    children: Vec<El>,
    cdata: Option<String>,
}

fn el(name: &'static str, attrs: &[(&'static str, &str)]) -> El {
    El {
        name,
        attrs: attrs.iter().map(|(k, v)| (*k, v.to_string())).collect(),
        children: vec![],
        cdata: None,
    }
}

impl El {
    fn write(&self, o: &mut String, depth: usize) {
        let pad = "  ".repeat(depth);
        let _ = write!(o, "{pad}<{}", self.name);
        for (k, v) in &self.attrs {
            let v = v
                .replace('&', "&amp;")
                .replace('<', "&lt;")
                .replace('>', "&gt;")
                .replace('"', "&quot;");
            let _ = write!(
                o,
                " {k}=\"{}\"",
                v.replace('\t', "&#9;")
                    .replace('\n', "&#10;")
                    .replace('\r', "&#13;")
            );
        }
        if let Some(c) = &self.cdata {
            let _ = writeln!(
                o,
                "><![CDATA[{}]]></{}>",
                c.replace("]]>", "]]]]><![CDATA[>"),
                self.name
            );
        } else if self.children.is_empty() {
            o.push_str("/>\n");
        } else {
            o.push_str(">\n");
            for c in &self.children {
                c.write(o, depth + 1);
            }
            let _ = writeln!(o, "{pad}</{}>", self.name);
        }
    }
}

fn xml(root: El, decorated: bool) -> String {
    let mut o = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    root.write(&mut o, 0);
    if decorated { escape(&o) } else { o }
}

pub fn checkstyle(changes: &[Change], decorated: bool) -> String {
    let mut root = el("checkstyle", &[("version", ABOUT)]);
    for c in changes {
        let mut file = el("file", &[("name", c.file)]);
        for f in c.fixers {
            file.children.push(el(
                "error",
                &[
                    ("severity", "warning"),
                    ("source", &format!("PHP-CS-Fixer.{f}")),
                    ("message", &format!("Found violation(s) of type: {f}")),
                ],
            ));
        }
        root.children.push(file);
    }
    xml(root, decorated)
}

pub fn junit(changes: &[Change], verbose: bool, decorated: bool) -> String {
    let mut suite = el("testsuite", &[("name", "PHP CS Fixer")]);
    let mut props = el("properties", &[]);
    props
        .children
        .push(el("property", &[("name", "about"), ("value", ABOUT)]));
    suite.children.push(props);
    if changes.is_empty() {
        suite
            .children
            .push(el("testcase", &[("name", "All OK"), ("assertions", "1")]));
        suite.attrs.extend(
            [
                ("tests", "1"),
                ("assertions", "1"),
                ("failures", "0"),
                ("errors", "0"),
            ]
            .map(|(k, v)| (k, v.to_string())),
        );
    } else {
        let mut assertions = 0;
        for c in changes {
            // `str_replace('.', '_DOT_', preg_replace('@\.<extension>$@', '', $file))`
            let base = c.file.rsplit('/').next().unwrap_or_default();
            let ext = base.rfind('.').map_or("", |i| &base[i + 1..]);
            let name = c
                .file
                .strip_suffix(&format!(".{ext}"))
                .unwrap_or(c.file)
                .replace('.', "_DOT_");
            let mut case = el(
                "testcase",
                &[
                    ("name", &name),
                    ("file", c.file),
                    ("assertions", &c.fixers.len().to_string()),
                ],
            );
            let mut text = if verbose {
                "applied fixers:\n---------------\n".to_string()
                    + &c.fixers
                        .iter()
                        .map(|f| format!("* {f}\n"))
                        .collect::<String>()
            } else {
                "Wrong code style\n".to_string()
            };
            if !c.diff.is_empty() {
                let _ = write!(text, "\nDiff:\n---------------\n\n{}", c.diff);
            }
            let mut failure = el("failure", &[("type", "code_style")]);
            failure.cdata = Some(
                text.trim_matches([' ', '\t', '\n', '\r', '\0', '\x0b'])
                    .to_string(),
            );
            case.children.push(failure);
            suite.children.push(case);
            assertions += c.fixers.len();
        }
        let n = changes.len().to_string();
        let a = assertions.to_string();
        suite.attrs.extend(
            [
                ("tests", n.as_str()),
                ("assertions", &a),
                ("failures", &a),
                ("errors", "0"),
            ]
            .map(|(k, v)| (k, v.to_string())),
        );
    }
    let mut root = el("testsuites", &[]);
    root.children.push(suite);
    xml(root, decorated)
}

pub fn xml_report(changes: &[Change], verbose: bool, decorated: bool) -> String {
    let mut root = el("report", &[]);
    root.children.push(el("about", &[("value", ABOUT)]));
    let mut files = el("files", &[]);
    for (i, c) in changes.iter().enumerate() {
        let mut file = el("file", &[("id", &(i + 1).to_string()), ("name", c.file)]);
        if verbose {
            let mut applied = el("applied_fixers", &[]);
            applied
                .children
                .extend(c.fixers.iter().map(|f| el("applied_fixer", &[("name", f)])));
            file.children.push(applied);
        }
        if !c.diff.is_empty() {
            let mut d = el("diff", &[]);
            d.cdata = Some(c.diff.to_string());
            file.children.push(d);
        }
        files.children.push(file);
    }
    root.children.push(files);
    xml(root, decorated)
}

/// `name => [summary, docs path]` for GitlabReporter's FixerFactory (generated: scripts/fixer-metadata.php --gitlab).
static FIXERS: LazyLock<serde_json::Map<String, serde_json::Value>> = LazyLock::new(|| {
    serde_json::from_str(include_str!("gitlab_fixers.json")).expect("gitlab_fixers.json")
});

pub fn gitlab(changes: &[Change], decorated: bool) -> String {
    let mut report = vec![];
    for c in changes {
        let lines = gitlab_lines(c.diff);
        for f in c.fixers {
            let (description, docs) = match FIXERS.get(*f) {
                Some(m) => (
                    m[0].as_str().unwrap_or_default().to_string(),
                    format!(
                        "Check [docs](https://cs.symfony.com/doc/rules/{}.html) for more information.",
                        m[1].as_str().unwrap_or_default()
                    ),
                ),
                None => (
                    format!("PHP-CS-Fixer.{f} (custom rule)"),
                    "Check performed with a custom rule.".to_string(),
                ),
            };
            report.push(serde_json::json!({
                "check_name": format!("PHP-CS-Fixer.{f}"),
                "description": description,
                "content": { "body": format!("{ABOUT}\n{docs}") },
                "categories": ["Style"],
                "fingerprint": md5(format!("{}{f}", c.file).as_bytes()),
                "severity": "minor",
                "location": { "path": c.file, "lines": { "begin": lines.0, "end": lines.1 } },
            }));
        }
    }
    let json = php_json(&serde_json::Value::Array(report));
    if decorated { escape(&json) } else { json }
}

/// GitlabReporter::getLines via sebastian/diff's Parser: first chunk of the first file diff; `begin` is its first
/// changed line, `end` is where the chunk (context included) ends.
fn gitlab_lines(diff: &str) -> (usize, usize) {
    let lines: Vec<&str> = diff
        .split("\r\n")
        .flat_map(|l| l.split(['\r', '\n']))
        .collect();
    let header = (0..lines.len().saturating_sub(1))
        .find(|&i| lines[i].starts_with("--- ") && lines[i + 1].starts_with("+++ "));
    let Some(h) = header.and_then(|i| {
        lines
            .iter()
            .skip(i + 2)
            .position(|l| l.starts_with("@@ -"))
            .map(|p| p + i + 2)
    }) else {
        return (0, 0);
    };
    // `@@ -start[,range] +…`
    let from = lines[h][4..].split(' ').next().unwrap_or_default();
    let (start, range) = from.split_once(',').unwrap_or((from, "1"));
    let (start, range): (usize, usize) = (start.parse().unwrap_or(0), range.parse().unwrap_or(1));
    let first = lines[h + 1..]
        .iter()
        .position(|l| l.starts_with(['+', '-']))
        .unwrap_or(0);
    (start + first, start + range)
}

/// RFC 1321 (PHP's md5()), hex.
fn md5(input: &[u8]) -> String {
    const S: [u32; 64] = [
        7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 5, 9, 14, 20, 5, 9, 14, 20, 5,
        9, 14, 20, 5, 9, 14, 20, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 6, 10,
        15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
    ];
    let k: Vec<u32> = (0..64)
        .map(|i| ((i as f64 + 1.0).sin().abs() * 4294967296.0) as u32)
        .collect();
    let mut msg = input.to_vec();
    msg.push(0x80);
    while msg.len() % 64 != 56 {
        msg.push(0);
    }
    msg.extend_from_slice(&((input.len() as u64).wrapping_mul(8)).to_le_bytes());
    let mut h: [u32; 4] = [0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476];
    for chunk in msg.chunks(64) {
        let m: Vec<u32> = chunk
            .chunks(4)
            .map(|w| u32::from_le_bytes([w[0], w[1], w[2], w[3]]))
            .collect();
        let [mut a, mut b, mut c, mut d] = h;
        for i in 0..64 {
            let (f, g) = match i / 16 {
                0 => ((b & c) | (!b & d), i),
                1 => ((d & b) | (!d & c), (5 * i + 1) % 16),
                2 => (b ^ c ^ d, (3 * i + 5) % 16),
                _ => (c ^ (b | !d), (7 * i) % 16),
            };
            let f = f.wrapping_add(a).wrapping_add(k[i]).wrapping_add(m[g]);
            a = d;
            d = c;
            c = b;
            b = b.wrapping_add(f.rotate_left(S[i]));
        }
        for (x, y) in h.iter_mut().zip([a, b, c, d]) {
            *x = x.wrapping_add(y);
        }
    }
    h.iter()
        .flat_map(|x| x.to_le_bytes())
        .map(|b| format!("{b:02x}"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn md5_like_php() {
        // php -r 'echo md5(""), md5("a.phparray_syntax"), md5(str_repeat("x", 64)), md5(str_repeat("é", 40));'
        assert_eq!(md5(b""), "d41d8cd98f00b204e9800998ecf8427e");
        assert_eq!(
            md5(b"a.phparray_syntax"),
            "0f09a23d834ce4b60a1f762a2597926b"
        );
        assert_eq!(md5(&[b'x'; 64]), "c1bb4f81d892b2d57947682aeb252456");
        assert_eq!(
            md5("é".repeat(40).as_bytes()),
            "639258863c18c505cc2183a5e4b63394"
        );
    }
}
