//! The classes a PHP file declares, and the class map of directories, as composer/class-map-generator 1.9 finds
//! them: strings, heredocs and comments are blanked out, then `class`, `interface`, `trait` and `enum` declarations are
//! matched together with the `namespace` they sit in. For PSR-0 and PSR-4 directories only the classes whose path
//! fits the rule count; the rest are reported as violations.

use anyhow::Result;
use pcre2::bytes::{Regex, RegexBuilder};
use pun_tokens::{Kind, tokenize};
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

fn re(pattern: &str) -> Regex {
    RegexBuilder::new()
        .jit_if_available(true)
        .build(pattern)
        .expect("valid pattern")
}

const TYPES: [&str; 4] = ["class", "interface", "trait", "enum"];

/// Per declaration keyword: the anchored pattern that finds one declaration, for files with a single candidate.
static TYPE_AT: LazyLock<Vec<(u8, &str, Regex)>> = LazyLock::new(|| {
    (TYPES.iter())
        .map(|t| {
            let pattern = format!(
                r"(?is)\G.\b(?<![\$:>]){t}\s++[a-zA-Z_\x7f-\xff:][a-zA-Z0-9_\x7f-\xff:\-]*+"
            );
            (t.as_bytes()[0], *t, re(&pattern))
        })
        .collect()
});
static CANDIDATES: LazyLock<Regex> =
    LazyLock::new(|| re(r"(?i)\b(?:class|interface|trait|enum)\s"));
static HEREDOC: LazyLock<Regex> = LazyLock::new(|| {
    re(r#"\G<<<[ \t]*+(['"]?)([a-zA-Z_\x80-\xff][a-zA-Z0-9_\x80-\xff]*+)\1(?:\r\n|\n|\r)"#)
});
static DECLARATIONS: LazyLock<Regex> = LazyLock::new(|| {
    re(r"(?ix)
        (?:
             \b(?<![\\$:>])(?P<type>class|interface|trait|enum) \s++ (?P<name>[a-zA-Z_\x7f-\xff:][a-zA-Z0-9_\x7f-\xff:\-]*+)
           | \b(?<![\\$:>])(?P<ns>namespace) (?P<nsname>\s++[a-zA-Z_\x7f-\xff][a-zA-Z0-9_\x7f-\xff]*+(?:\s*+\\\s*+[a-zA-Z_\x7f-\xff][a-zA-Z0-9_\x7f-\xff]*+)*+)? \s*+ [\{;]
        )")
});

/// The code of `contents` with every string, heredoc and comment taken out, as `PhpFileCleaner` gives it; with
/// `max_matches` 1 it stops after the first declaration.
fn clean(c: &[u8], max_matches: usize, raw: bool) -> Vec<u8> {
    // The bytes that can start something the cleaner acts on, as a table: the scan between them is the hot loop.
    const REJECT: [bool; 256] = {
        let mut t = [false; 256];
        let set = b"?\"'`</#citre";
        let mut i = 0;
        while i < set.len() {
            t[set[i] as usize] = true;
            i += 1;
        }
        t
    };
    let len = c.len();
    let peek = |i: usize, ch: u8| i + 1 < len && c[i + 1] == ch;
    let mut out = Vec::with_capacity(len);
    let mut i = 0;
    // `skipToNewline`: to the next line break; in a `//` or `#` comment, a `?>` ends it too.
    let to_newline = |i: &mut usize, php_comment: bool| loop {
        let stop: &[u8] = if php_comment { b"\r\n?" } else { b"\r\n" };
        *i += c[*i..]
            .iter()
            .position(|b| stop.contains(b))
            .unwrap_or(len - *i);
        if !php_comment || *i >= len || c[*i] != b'?' || peek(*i, b'>') {
            return;
        }
        *i += 1;
    };
    while i < len {
        // skipToPhp
        while i < len {
            if c[i] == b'<' && peek(i, b'?') {
                i += 2;
                break;
            }
            i += 1;
        }
        out.extend_from_slice(b"<?");
        while i < len {
            let ch = c[i];
            if ch == b'?' && peek(i, b'>') {
                out.extend_from_slice(b"?>");
                i += 2;
                break;
            }
            if ch == b'"' || ch == b'\'' || ch == b'`' {
                // skipString
                i += 1;
                while i < len {
                    i += c[i..]
                        .iter()
                        .position(|b| *b == b'\\' || *b == ch)
                        .unwrap_or(len - i);
                    if i >= len {
                        break;
                    }
                    if c[i] == b'\\' && (peek(i, b'\\') || peek(i, ch)) {
                        i += 2;
                        continue;
                    }
                    if c[i] == ch {
                        i += 1;
                        break;
                    }
                    i += 1;
                }
                out.extend_from_slice(b"null");
                continue;
            }
            if ch == b'<'
                && peek(i, b'<')
                && let Some((end, label)) = heredoc_at(c, i)
            {
                i = end;
                skip_heredoc(c, &mut i, &label, &to_newline);
                out.extend_from_slice(b"null");
                continue;
            }
            if ch == b'#' && raw && !peek(i, b'[') {
                to_newline(&mut i, true);
                continue;
            }
            if ch == b'/' {
                if peek(i, b'/') {
                    to_newline(&mut i, raw);
                    continue;
                }
                if peek(i, b'*') {
                    i += 2;
                    while i < len {
                        i += c[i..].iter().position(|b| *b == b'*').unwrap_or(len - i);
                        if peek(i, b'/') {
                            i += 2;
                            break;
                        }
                        i += 1;
                    }
                    continue;
                }
            }
            if max_matches == 1
                && let Some((_, t, pattern)) = TYPE_AT.iter().find(|(first, ..)| *first == ch)
                && c[i..].starts_with(t.as_bytes())
                && let Ok(Some(m)) = pattern.find_at(c, i - 1)
            {
                out.extend_from_slice(&c[m.start()..m.end()]);
                return out;
            }
            i += 1;
            let skip = c[i..]
                .iter()
                .position(|b| REJECT[*b as usize])
                .unwrap_or(len - i);
            out.push(ch);
            out.extend_from_slice(&c[i..i + skip]);
            i += skip;
        }
    }
    out
}

/// A heredoc or nowdoc opener at `i`: where its body starts, and its label.
fn heredoc_at(c: &[u8], i: usize) -> Option<(usize, Vec<u8>)> {
    let mut locs = HEREDOC.capture_locations();
    let m = HEREDOC.captures_read_at(&mut locs, c, i).ok()??;
    let (s, e) = locs.get(2)?;
    Some((m.end(), c[s..e].to_vec()))
}

fn skip_heredoc(c: &[u8], i: &mut usize, label: &[u8], to_newline: &dyn Fn(&mut usize, bool)) {
    let len = c.len();
    let word = |b: u8| b.is_ascii_alphanumeric() || b == b'_' || b >= 0x80;
    while *i < len {
        match c[*i] {
            b'\t' | b' ' => {
                *i += 1;
                continue;
            }
            b if b == label[0]
                && c[*i..].starts_with(label)
                && !c.get(*i + label.len()).is_some_and(|b| word(*b)) =>
            {
                *i += label.len();
                return;
            }
            _ => {}
        }
        to_newline(i, false);
        *i += c[*i..]
            .iter()
            .take_while(|b| **b == b'\r' || **b == b'\n')
            .count();
    }
}

/// `php_strip_whitespace`: the code without comments, each run of whitespace one space.
fn strip_whitespace(code: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(code.len());
    let mut space = false;
    for t in tokenize(code) {
        match t.kind {
            Some(Kind::T_WHITESPACE) => {
                if !space {
                    out.push(b' ');
                    space = true;
                }
            }
            Some(Kind::T_COMMENT | Kind::T_DOC_COMMENT) => {}
            _ => {
                out.extend_from_slice(t.text);
                space = false;
            }
        }
    }
    out
}

/// The classes `code` declares, fully qualified, in order (`PhpFileParser::findClasses`).
pub fn find_classes(code: &[u8]) -> Vec<String> {
    // The tokenizer only runs for shell strings, which the cleaner would misread.
    let (contents, raw) = if code.contains(&b'`') {
        (strip_whitespace(code), false)
    } else {
        (code.to_vec(), true)
    };
    let candidates = CANDIDATES.find_iter(&contents).count();
    if candidates == 0 {
        return vec![];
    }
    let cleaned = clean(&contents, candidates, raw);
    let text = |b: &[u8]| String::from_utf8_lossy(b).into_owned();
    let mut classes = vec![];
    let mut namespace = String::new();
    let mut locs = DECLARATIONS.capture_locations();
    let (ty, name, ns, nsname) = (
        group(&DECLARATIONS, "type"),
        group(&DECLARATIONS, "name"),
        group(&DECLARATIONS, "ns"),
        group(&DECLARATIONS, "nsname"),
    );
    let mut at = 0;
    while let Ok(Some(m)) = DECLARATIONS.captures_read_at(&mut locs, &cleaned, at) {
        // Every alternative consumes at least the keyword.
        at = m.end();
        let get = |g: usize| locs.get(g).map(|(s, e)| &cleaned[s..e]);
        if get(ns).is_some() {
            let declared: Vec<u8> = get(nsname)
                .unwrap_or(b"")
                .iter()
                .copied()
                .filter(|b| !b" \t\r\n".contains(b))
                .collect();
            namespace = format!("{}\\", text(&declared));
            continue;
        }
        let mut class = text(get(name).unwrap_or(b""));
        if class == "extends" || class == "implements" {
            continue;
        }
        if class.starts_with(':') {
            class = format!("xhp{}", &class.replace('-', "_").replace(':', "__")[1..]);
        } else if get(ty).is_some_and(|t| t.eq_ignore_ascii_case(b"enum"))
            && let Some(colon) = class.rfind(':')
        {
            class.truncate(colon);
        }
        classes.push(
            format!("{namespace}{class}")
                .trim_start_matches('\\')
                .to_string(),
        );
    }
    classes
}

fn group(re: &Regex, name: &str) -> usize {
    (re.capture_names().iter())
        .position(|n| n.as_deref() == Some(name))
        .expect("named group")
}

/// Which mapping a directory is scanned for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rule {
    Classmap,
    Psr0,
    Psr4,
}

/// A class map being built: class → file, the classes found in more than one file, and the PSR violations.
#[derive(Debug, Default)]
pub struct ClassMap {
    pub map: Vec<(String, String)>,
    index: HashMap<String, usize>,
    /// class → the other files that declare it.
    pub ambiguous: Vec<(String, Vec<String>)>,
    /// (file, warning).
    pub violations: Vec<(String, String)>,
    scanned: HashSet<PathBuf>,
}

impl ClassMap {
    pub fn add(&mut self, class: &str, path: &str) {
        match self.index.get(class) {
            None => {
                self.index.insert(class.to_string(), self.map.len());
                self.map.push((class.to_string(), path.to_string()));
            }
            Some(&i) if self.map[i].1 != path => {
                match self.ambiguous.iter_mut().find(|a| a.0 == class) {
                    Some(a) => a.1.push(path.to_string()),
                    None => self
                        .ambiguous
                        .push((class.to_string(), vec![path.to_string()])),
                }
            }
            Some(_) => {}
        }
    }

    /// `ClassMap::addClass`: sets the class's file, replacing the one there without reporting it as ambiguous.
    pub fn set(&mut self, class: &str, path: &str) {
        match self.index.get(class) {
            Some(&i) => self.map[i].1 = path.to_string(),
            None => self.add(class, path),
        }
    }

    pub fn path_of(&self, class: &str) -> Option<&str> {
        self.index.get(class).map(|&i| self.map[i].1.as_str())
    }

    /// The map sorted by class name (`ksort`).
    pub fn sorted(&self) -> Vec<(String, String)> {
        let mut map = self.map.clone();
        map.sort_by(|a, b| a.0.as_bytes().cmp(b.0.as_bytes()));
        map
    }

    /// The ambiguous classes with the files that look like tests, fixtures, examples or stubs left out, and only those
    /// still in two files.
    pub fn ambiguous_reported(&self) -> Vec<(&str, Vec<&str>)> {
        static DUPLICATES: LazyLock<Regex> =
            LazyLock::new(|| re(r"(?i)(?:^|/)(test|fixture|example|stub)s?/"));
        (self.ambiguous.iter())
            .filter_map(|(class, paths)| {
                let kept: Vec<&str> = (paths.iter())
                    .filter(|p| !DUPLICATES.is_match(p.as_bytes()).unwrap_or(false))
                    .map(String::as_str)
                    .collect();
                (!kept.is_empty()).then_some((class.as_str(), kept))
            })
            .collect()
    }

    /// `ClassMapGenerator::scanPaths` for each job in order, as one parallel pass: every directory is listed and
    /// every file parsed at once, then the results are added job by job, so the first job to reach a file still
    /// claims it. Fails on the first file that can't be read, as Composer does.
    pub fn scan_all(&mut self, jobs: &[Job], cwd: &str) -> Result<()> {
        let listed: Vec<Result<Vec<(String, PathBuf)>>> = jobs.par_iter().map(list).collect();
        let mut unique: HashMap<&Path, usize> = HashMap::new();
        let mut todo: Vec<&str> = vec![];
        for files in listed.iter().flatten() {
            for (path, real) in files {
                if !self.scanned.contains(real) && !unique.contains_key(real.as_path()) {
                    unique.insert(real, todo.len());
                    todo.push(path);
                }
            }
        }
        let parsed: Vec<Result<Vec<String>, String>> = (todo.par_iter())
            .map(|path| match std::fs::read(path) {
                Ok(code) => Ok(find_classes(&code)),
                Err(_) => Err(format!(
                    "File at \"{path}\" is not readable, check its permissions"
                )),
            })
            .collect();
        for (job, files) in jobs.iter().zip(&listed) {
            let files = files.as_ref().map_err(|e| anyhow::anyhow!("{e}"))?;
            for (path, real) in files {
                if self.scanned.contains(real) {
                    continue;
                }
                let mut classes = parsed[unique[real.as_path()]]
                    .clone()
                    .map_err(anyhow::Error::msg)?;
                if job.rule == Rule::Classmap {
                    self.scanned.insert(real.clone());
                } else {
                    classes = self.filter(classes, path, &job.namespace, job.rule, &job.dir, cwd);
                    if !classes.is_empty() {
                        self.scanned.insert(real.clone());
                    }
                }
                for class in classes {
                    self.add(&class, path);
                }
            }
        }
        Ok(())
    }

    /// The classes of `path` its PSR rule can load; when none can, each is a violation.
    fn filter(
        &mut self,
        classes: Vec<String>,
        path: &str,
        namespace: &str,
        rule: Rule,
        base: &str,
        cwd: &str,
    ) -> Vec<String> {
        let sub = &path[(base.len() + 1).min(path.len())..];
        let sub = sub.rfind('.').map_or(sub, |dot| &sub[..dot]);
        let (valid, rejected): (Vec<String>, Vec<String>) =
            classes.into_iter().partition(|class| {
                let expected = if rule == Rule::Psr0 {
                    if !namespace.is_empty() && !class.starts_with(namespace) {
                        return false;
                    }
                    match class.rfind('\\') {
                        Some(i) => format!(
                            "{}{}",
                            class[..=i].replace('\\', "/"),
                            class[i + 1..].replace('_', "/")
                        ),
                        None => class.replace('_', "/"),
                    }
                } else {
                    class
                        .get(namespace.len()..)
                        .unwrap_or("")
                        .replace('\\', "/")
                };
                expected == sub
            });
        if valid.is_empty() {
            let short = |p: &str| match p.strip_prefix(cwd) {
                Some(rest) => format!(".{rest}"),
                None => p.to_string(),
            };
            let kind = if rule == Rule::Psr0 { "psr-0" } else { "psr-4" };
            for class in rejected {
                let warning = format!(
                    "Class {class} located in {} does not comply with {kind} autoloading standard (rule: {namespace} => {}). Skipping.",
                    short(path),
                    short(base)
                );
                self.violations.push((path.to_string(), warning));
            }
        }
        valid
    }
}

/// One directory (or file) to scan.
#[derive(Debug, Clone)]
pub struct Job {
    /// Absolute and normalized.
    pub dir: String,
    /// As the autoload rule gives it (relative for the root), for the error when it is missing.
    pub given: String,
    pub exclude: Option<Regex>,
    pub rule: Rule,
    pub namespace: String,
}

/// The PHP files of a job, (path, real path), without the excluded ones.
fn list(job: &Job) -> Result<Vec<(String, PathBuf)>> {
    let dir = Path::new(&job.dir);
    let mut files = vec![];
    match std::fs::canonicalize(dir) {
        Ok(real) if dir.is_file() => files.push((job.dir.clone(), real)),
        Ok(real) if dir.is_dir() => walk(&job.dir, &real, &mut files),
        _ => anyhow::bail!(
            "Could not scan for classes inside \"{}\" which does not appear to be a file nor a folder",
            job.given
        ),
    }
    files.retain(|(path, real)| {
        let php = [".php", ".inc", ".hh"].iter().any(|e| path.ends_with(e));
        php && !job.exclude.as_ref().is_some_and(|x| {
            x.is_match(real.to_string_lossy().as_bytes())
                .unwrap_or(false)
                || x.is_match(path.as_bytes()).unwrap_or(false)
        })
    });
    Ok(files)
}

/// Finder's walk: depth first in directory order, without dot files or version control directories, symlinks
/// followed. Each file comes with its real path: `real` (its directory's) joined with its name, unless a symlink
/// leads elsewhere, so only symlinks cost a lookup.
fn walk(dir: &str, real: &Path, out: &mut Vec<(String, PathBuf)>) {
    const VCS: [&str; 4] = ["_svn", "CVS", "_darcs", ".arch-params"];
    for e in std::fs::read_dir(dir).into_iter().flatten().flatten() {
        let name = e.file_name();
        let name = name.to_string_lossy();
        if name.starts_with('.') || VCS.contains(&name.as_ref()) {
            continue;
        }
        let path = format!("{dir}/{name}");
        let Ok(kind) = e.file_type() else { continue };
        let (is_dir, real) = if kind.is_symlink() {
            match (std::fs::metadata(&path), std::fs::canonicalize(&path)) {
                (Ok(m), Ok(target)) => (m.is_dir(), target),
                _ => continue,
            }
        } else if kind.is_dir() || kind.is_file() {
            (kind.is_dir(), real.join(name.as_ref()))
        } else {
            continue;
        };
        if is_dir {
            walk(&path, &real, out);
        } else {
            out.push((path, real));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Expected values from composer/class-map-generator 1.9.0's `PhpFileParser::findClasses` in PHP 8.5, on the same
    /// code (written to a file).
    #[test]
    fn finds_classes_like_composer() {
        let code = br"<?php
namespace App\Models;

// class Commented {}
/* interface Hidden {} */
$s = 'class InString {}';
$h = <<<EOT
class InHeredoc {}
EOT;
#[Attribute]
final class User extends Model {}
enum Status: string { case A = 'a'; }
$anon = new class extends Base {};
interface Contract {}
trait Helpers {}
namespace Other { abstract class Base {} }
";
        assert_eq!(
            find_classes(code),
            [
                "App\\Models\\User",
                "App\\Models\\Status",
                "App\\Models\\Contract",
                "App\\Models\\Helpers",
                "Other\\Base"
            ]
        );
        // A single candidate takes the cleaner's early return; a shell string goes through the tokenizer.
        assert_eq!(find_classes(b"<?php\nclass Only {}\n"), ["Only"]);
        assert_eq!(
            find_classes(b"<?php\n$x = `ls`; # class NotThis\nclass Shell {}\n"),
            ["Shell"]
        );
        assert_eq!(find_classes(b"<?php\n$x = 1;\n"), Vec::<String>::new());
        assert_eq!(
            find_classes(b"<?php\nclass :x:my-tag {}\n"),
            ["xhp_x__my_tag"]
        );
        assert_eq!(
            find_classes(b"<html><?php class A {} ?> class B <?php class C {}"),
            ["A", "C"]
        );
        // A `?` inside a `#` comment, escaped and unescaped backslashes in strings, a `*` inside a block comment, a
        // heredoc whose closing label is indented, and a string or comment still open at the end of the file.
        let tricky = br#"<?php
# note ? here, class FakeHash {}
class A {}
$s = 'it\'s class Fake1 {}'; $t = "a\\"; $u = "x\y class Fake2 {}";
/* a * b class Fake3 {} */
$h = <<<EOT
  class Fake4 {}
  EOT;
class B {}
$d = 4 / 2;
$x = 'unterminated class Fake5 {}
"#;
        assert_eq!(find_classes(tricky), ["A", "B"]);
        assert_eq!(
            find_classes(b"<?php\nclass C {}\n/* never closed class Fake6 {}\n"),
            ["C"]
        );
    }

    /// Expected values from class-map-generator 1.9.0's `ClassMapGenerator::scanPaths` on the same tree, run from
    /// `/usr`: a classmap scan then a psr-0 scan of one directory (the psr-0 scan skips the files already mapped), and
    /// the psr-0 scan alone with its violations. Dot files, VCS directories, a dangling symlink and a fifo are passed
    /// over; a directory symlink is followed; a class declared twice in one file is not ambiguous.
    #[test]
    fn scans_like_composer() {
        let t = std::env::temp_dir().join(format!("pun-classmap-scan-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&t);
        let d = t.join("d");
        for dir in ["Foo/Bar/Baz", ".hidden", "CVS"] {
            std::fs::create_dir_all(d.join(dir)).unwrap();
        }
        std::fs::create_dir_all(t.join("real")).unwrap();
        for (file, code) in [
            (
                "d/Foo/Bar/Baz/Qux.php",
                "<?php\nnamespace Foo\\Bar;\nclass Baz_Qux {}\n",
            ),
            (
                "d/Foo/Bar/Wrong.php",
                "<?php\nnamespace Other;\nclass X {}\n",
            ),
            (
                "d/Twice.php",
                "<?php\nif (PHP_VERSION_ID > 1) { class Twice {} } else { class Twice {} }\n",
            ),
            ("d/.hidden.php", "<?php class Hidden {}\n"),
            ("d/.hidden/In.php", "<?php class InHidden {}\n"),
            ("d/CVS/C.php", "<?php class InCvs {}\n"),
            ("real/Linked.php", "<?php class Linked {}\n"),
        ] {
            std::fs::write(t.join(file), code).unwrap();
        }
        std::os::unix::fs::symlink(t.join("real"), d.join("linked")).unwrap();
        std::os::unix::fs::symlink(t.join("nowhere"), d.join("dangling")).unwrap();
        assert!(
            std::process::Command::new("mkfifo")
                .arg(d.join("pipe"))
                .status()
                .unwrap()
                .success()
        );
        let dir = d.to_string_lossy().into_owned();
        let job = |rule, namespace: &str| Job {
            dir: dir.clone(),
            given: dir.clone(),
            exclude: None,
            rule,
            namespace: namespace.into(),
        };
        let at = |p: &str| format!("{dir}/{p}");
        let mut map = ClassMap::default();
        map.scan_all(&[job(Rule::Classmap, ""), job(Rule::Psr0, "Foo\\")], "/usr")
            .unwrap();
        assert_eq!(
            map.sorted(),
            [
                ("Foo\\Bar\\Baz_Qux".to_string(), at("Foo/Bar/Baz/Qux.php")),
                ("Linked".to_string(), at("linked/Linked.php")),
                ("Other\\X".to_string(), at("Foo/Bar/Wrong.php")),
                ("Twice".to_string(), at("Twice.php")),
            ]
        );
        assert!(map.ambiguous.is_empty() && map.violations.is_empty());
        map.set("Twice", "/elsewhere.php");
        assert_eq!(map.path_of("Twice"), Some("/elsewhere.php"));

        let mut psr0 = ClassMap::default();
        psr0.scan_all(&[job(Rule::Psr0, "Foo\\")], "/usr").unwrap();
        let rule = format!("(rule: Foo\\ => {dir}). Skipping.");
        let violation = |class: &str, file: &str| {
            format!(
                "Class {class} located in {} does not comply with psr-0 autoloading standard {rule}",
                at(file)
            )
        };
        let mut violations: Vec<String> = psr0.violations.iter().map(|v| v.1.clone()).collect();
        violations.sort();
        let mut want = [
            violation("Other\\X", "Foo/Bar/Wrong.php"),
            violation("Linked", "linked/Linked.php"),
            violation("Twice", "Twice.php"),
            violation("Twice", "Twice.php"),
        ];
        want.sort();
        assert_eq!(violations, want);
        assert_eq!(
            psr0.sorted(),
            [("Foo\\Bar\\Baz_Qux".to_string(), at("Foo/Bar/Baz/Qux.php"))]
        );
        std::fs::remove_dir_all(&t).unwrap();
    }
}
