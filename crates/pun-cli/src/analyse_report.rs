//! `pun analyse --format=<f>`: PHPStan 2.3.1's error formatters, byte for byte
//! (.oracle/phpstan-src/src/Command/ErrorFormatter/*.php). `json` and `raw` print absolute paths, the others paths
//! relative to the project root (PHPStan's simple relative path helper, the cwd being the root). The errors about
//! ignores can't be ignored, and an unmatched `ignoreErrors` entry has no line (or, without a file path, no file:
//! a not-file-specific error). PHPStan's warnings have no Pun counterpart.

use crate::analyse::Finding;
use crate::console;
use anyhow::{Result, bail};
use sha2::{Digest, Sha256};
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

/// The formats, as `--format` takes them (PHPStan's `--error-format` names).
pub(crate) const FORMATS: [&str; 9] = [
    "table",
    "raw",
    "json",
    "prettyJson",
    "checkstyle",
    "junit",
    "gitlab",
    "github",
    "teamcity",
];

/// What a report needs besides the errors.
pub(crate) struct Context<'a> {
    /// `raw`'s identifiers (PHPStan's `-v`, or a detected coding agent).
    pub(crate) verbose: bool,
    /// The project root, PHPStan's current working directory.
    pub(crate) root: &'a Path,
    /// The analysed paths (absolute), for `table`'s relative paths.
    pub(crate) paths: &'a [PathBuf],
    /// The not-file-specific errors.
    pub(crate) general: &'a [String],
    /// pun.toml, relative to the root: teamcity's file for the not-file-specific errors (PHPStan's config file).
    pub(crate) config: Option<String>,
}

/// The report in `format`.
pub(crate) fn render(format: &str, findings: &[Finding], cx: &Context) -> Result<String> {
    let (verbose, general) = (cx.verbose, cx.general);
    Ok(match format {
        "raw" => raw(findings, general, verbose),
        "json" => json(findings, general, false),
        "prettyJson" => json(findings, general, true),
        "checkstyle" => checkstyle(findings, general),
        "junit" => junit(findings, general),
        "gitlab" => gitlab(findings, general),
        "github" => github(findings, general),
        "teamcity" => teamcity(findings, general, cx.config.as_deref()),
        "table" => table(findings, cx, console::width()),
        _ => bail!(
            "Error formatter \"{format}\" not found. Available error formatters are: {}",
            FORMATS.join(", ")
        ),
    })
}

/// `getLine() ?? ''` (`(string) null`).
fn line(f: &Finding) -> String {
    f.line().map(|l| l.to_string()).unwrap_or_default()
}

/// RawErrorFormatter: `?:?:message` per not-file-specific error, then `file:line:message`, with ` [identifier=x]`
/// under `-v`.
fn raw(findings: &[Finding], general: &[String], verbose: bool) -> String {
    let mut out = String::new();
    for g in general {
        let _ = writeln!(out, "?:?:{g}");
    }
    for f in findings {
        let at = f.line().map_or("?".to_string(), |l| l.to_string());
        let _ = write!(out, "{}:{at}:{}", f.file.display(), f.message);
        if verbose {
            let _ = write!(out, " [identifier={}]", f.identifier);
        }
        out.push('\n');
    }
    out
}

/// Nette's Json::encode: unescaped slashes and Unicode, but PHP still escapes U+2028 and U+2029.
fn encode(value: &serde_json::Value, pretty: bool) -> String {
    let out = if pretty {
        let mut buf = Vec::new();
        let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
        let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
        serde::Serialize::serialize(value, &mut ser).expect("json");
        String::from_utf8(buf).expect("utf-8")
    } else {
        value.to_string()
    };
    out.replace('\u{2028}', "\\u2028")
        .replace('\u{2029}', "\\u2029")
}

/// JsonErrorFormatter: totals, then the messages per file (absolute paths).
fn json(findings: &[Finding], general: &[String], pretty: bool) -> String {
    use serde_json::{Map, Value, json};
    let mut files = Map::new();
    for f in findings {
        let entry = files
            .entry(f.file.to_string_lossy().into_owned())
            .or_insert_with(|| json!({ "errors": 0, "messages": [] }));
        entry["errors"] = json!(entry["errors"].as_u64().unwrap_or(0) + 1);
        let mut message = Map::new();
        message.insert("message".into(), json!(f.message));
        message.insert("line".into(), json!(f.line()));
        message.insert("ignorable".into(), json!(f.ignorable()));
        if let Some(tip) = &f.tip {
            message.insert("tip".into(), json!(tip));
        }
        message.insert("identifier".into(), json!(f.identifier));
        entry["messages"]
            .as_array_mut()
            .expect("an array")
            .push(Value::Object(message));
    }
    let value = json!({
        "totals": { "errors": general.len(), "file_errors": findings.len() },
        "files": files,
        "errors": general,
    });
    encode(&value, pretty)
}

/// htmlspecialchars(ENT_XML1 | ENT_COMPAT): `&`, `<`, `>` and `"`.
fn xml(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

/// CheckstyleErrorFormatter: one `<file>` per file in error order, column 1; the not-file-specific errors in a
/// `<file>` without a name.
fn checkstyle(findings: &[Finding], general: &[String]) -> String {
    let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<checkstyle>\n");
    let mut files: Vec<(&str, Vec<&Finding>)> = vec![];
    for f in findings {
        match files.iter_mut().find(|(rel, _)| *rel == f.rel) {
            Some((_, list)) => list.push(f),
            None => files.push((&f.rel, vec![f])),
        }
    }
    for (rel, list) in files {
        let _ = writeln!(out, "<file name=\"{}\">", xml(rel));
        for f in list {
            let _ = writeln!(
                out,
                "  <error line=\"{}\" column=\"1\" severity=\"error\" message=\"{}\" source=\"{}\" />",
                line(f),
                xml(&f.message),
                xml(&f.identifier)
            );
        }
        out.push_str("</file>\n");
    }
    if !general.is_empty() {
        out.push_str("<file>\n");
        for g in general {
            let _ = writeln!(out, "  <error severity=\"error\" message=\"{}\" />", xml(g));
        }
        out.push_str("</file>\n");
    }
    out.push_str("</checkstyle>\n");
    out
}

/// JunitErrorFormatter: one failing test case per error (`General error` without a file), or one passing `phpstan`
/// case.
fn junit(findings: &[Finding], general: &[String]) -> String {
    let n = findings.len() + general.len();
    let mut out = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?><testsuite failures=\"{n}\" name=\"phpstan\" tests=\"{}\" xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xsi:noNamespaceSchemaLocation=\"https://raw.githubusercontent.com/junit-team/junit5/r5.5.1/platform-tests/src/test/resources/jenkins-junit.xsd\">",
        n.max(1)
    );
    for f in findings {
        let _ = write!(
            out,
            "<testcase name=\"{}\"><failure type=\"ERROR\" message=\"{}\" /></testcase>",
            xml(&format!("{}:{}", f.rel, line(f))),
            xml(&f.message)
        );
    }
    for g in general {
        let _ = write!(
            out,
            "<testcase name=\"General error\"><failure type=\"ERROR\" message=\"{}\" /></testcase>",
            xml(g)
        );
    }
    if n == 0 {
        out.push_str("<testcase name=\"phpstan\"></testcase>");
    }
    out.push_str("</testsuite>");
    out
}

/// GitlabErrorFormatter: a pretty JSON list; the fingerprint is the sha256 of the absolute path, line and message
/// (of the message alone without a file).
fn gitlab(findings: &[Finding], general: &[String]) -> String {
    let sha = |s: String| -> String {
        Sha256::digest(s)
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect()
    };
    let mut list: Vec<serde_json::Value> = findings
        .iter()
        .map(|f| {
            serde_json::json!({
                "description": f.message,
                "check_name": f.identifier,
                "fingerprint": sha(format!("{}{}{}", f.file.display(), line(f), f.message)),
                "severity": if f.ignorable() { "major" } else { "blocker" },
                "location": { "path": f.rel, "lines": { "begin": f.line } },
            })
        })
        .collect();
    list.extend(general.iter().map(|g| {
        serde_json::json!({
            "description": g,
            "fingerprint": sha(g.clone()),
            "severity": "major",
            "location": { "path": "", "lines": { "begin": 0 } },
        })
    }));
    encode(&serde_json::Value::Array(list), true)
}

/// GithubErrorFormatter: `::error file=…,line=…,col=0::message` (`::error ::message` without a file), newlines as
/// `%0A` and `@mentions` in backticks.
fn github(findings: &[Finding], general: &[String]) -> String {
    let message = |m: &str| crate::github::mentions(&m.replace('\n', "%0A"));
    let mut out = String::new();
    for f in findings {
        let _ = writeln!(
            out,
            "::error file={},line={},col=0::{}",
            f.rel,
            line(f),
            message(&f.message)
        );
    }
    for g in general {
        let _ = writeln!(out, "::error ::{}", message(g));
    }
    out
}

/// TeamcityErrorFormatter: nothing without errors; else the inspection type and one inspection per error (on the
/// config file without one, `.` with none).
fn teamcity(findings: &[Finding], general: &[String], config: Option<&str>) -> String {
    if findings.is_empty() && general.is_empty() {
        return String::new();
    }
    let escape = |s: &str| {
        let mut o = String::new();
        for c in s.chars() {
            // Teamcity also escapes `\n` and `\r`, which no message and no path PHPStan analyses holds.
            match c {
                '\'' | '|' | '[' | ']' => {
                    o.push('|');
                    o.push(c);
                }
                c => o.push(c),
            }
        }
        o
    };
    let mut out = String::from(
        "##teamcity[inspectionType id='phpstan' name='phpstan' category='phpstan' description='phpstan Inspection']\n",
    );
    for f in findings {
        let message = match f.ignorable() {
            true => format!("{} (🪪 {})", f.message, f.identifier),
            false => f.message.clone(),
        };
        let _ = writeln!(
            out,
            "##teamcity[inspection typeId='phpstan' message='{}' file='{}' line='{}' SEVERITY='ERROR' ignorable='{}' tip='{}']",
            escape(&message),
            escape(&f.rel),
            line(f),
            if f.ignorable() { "1" } else { "" },
            escape(f.tip.as_deref().unwrap_or(""))
        );
    }
    for g in general {
        let _ = writeln!(
            out,
            "##teamcity[inspection typeId='phpstan' message='{}' file='{}' SEVERITY='ERROR']",
            escape(g),
            escape(config.unwrap_or("."))
        );
    }
    out
}

/// TableErrorFormatter: one Symfony table per file (`Line` and the path), the identifier (🪪) and tip (💡) under
/// each message, then the `[ERROR] Found N errors` block; `[OK] No errors` without any. The paths are relative to
/// the analysed paths' common directory (PHPStan's fuzzy relative path).
fn table(findings: &[Finding], cx: &Context, width: usize) -> String {
    let style = crate::table::Style::new(width);
    if findings.is_empty() && cx.general.is_empty() {
        return style.block("OK", "No errors", 0);
    }
    let mut out = String::new();
    let mut files: Vec<(&str, Vec<&Finding>)> = vec![];
    for f in findings {
        match files.iter_mut().find(|(rel, _)| *rel == f.rel) {
            Some((_, list)) => list.push(f),
            None => files.push((&f.rel, vec![f])),
        }
    }
    let mut tables = files.len();
    for (_, list) in files {
        let rows: Vec<[String; 2]> = list
            .iter()
            .map(|f| {
                let mut message = format!("{}\n🪪  {}", f.message, f.identifier);
                if !f.ignorable() {
                    message.push_str(" (non-ignorable)");
                }
                // ponytail: PHPStan puts 💡 before each line of a multi-line tip (less its leading ` •`); Pun's tips
                // are one line, so add that when a rule reports a bulleted tip.
                if let Some(tip) = &f.tip {
                    message.push_str(&format!("\n💡  {tip}"));
                }
                [line(f), message]
            })
            .collect();
        let shown = fuzzy_relative(cx.root, cx.paths, &list[0].file);
        out.push_str(&style.table(["Line", &shown], &rows));
    }
    if !cx.general.is_empty() {
        let rows: Vec<[String; 2]> = (cx.general.iter())
            .map(|g| [String::new(), g.clone()])
            .collect();
        out.push_str(&style.table(["", "Error"], &rows));
        tables += 1;
    }
    let n = findings.len() + cx.general.len();
    let found = if n == 1 {
        "Found 1 error".to_string()
    } else {
        format!("Found {n} errors")
    };
    out.push_str(&style.block("ERROR", &found, tables));
    out
}

/// FuzzyRelativePathHelper: `file` without the deepest directory that the working directory and every analysed
/// path share (the first analysed path may go deeper than the working directory), else relative to the root.
fn fuzzy_relative(root: &Path, paths: &[PathBuf], file: &Path) -> String {
    let parts = |p: &Path| -> Vec<String> {
        let s = p.to_string_lossy();
        s.trim_start_matches('/')
            .split('/')
            .map(str::to_string)
            .collect()
    };
    let mut trim: Option<Vec<String>> = (root.as_os_str() != "/").then(|| parts(root));
    for (n, path) in paths.iter().enumerate() {
        let path_parts = parts(path);
        let mut temp: Vec<String> = vec![];
        let size = path_parts.len();
        for (i, part) in path_parts.iter().enumerate() {
            if i == size - 1 && part.ends_with(".php") {
                continue;
            }
            match trim.as_ref().and_then(|t| t.get(i)) {
                None if n != 0 => break,
                Some(t) if t != part => break,
                _ => temp.push(part.clone()),
            }
        }
        trim = Some(temp);
    }
    let file = file.to_string_lossy();
    let prefix = trim
        .filter(|t| !t.is_empty())
        .map(|t| format!("/{}", t.join("/")));
    if let Some(rest) = prefix.as_deref().and_then(|p| file.strip_prefix(p)) {
        return rest.trim_start_matches('/').to_string();
    }
    file.strip_prefix(&format!("{}/", root.display()))
        .unwrap_or(&file)
        .to_string()
}
