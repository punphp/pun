//! The app's views and translations as UnusedViewsRule and NoMissingTranslationsRule read them: ViewFileHelper,
//! the view finder (`view()->exists()`, `getFinder()->find()`), UsedViewInAnotherViewCollector,
//! UsedTranslationViewCollector and the translation files.
//!
//! Larastan asks the booted app for the view finder's paths and hints; Pun takes `resources/views` (the
//! `view.paths` default), so paths and namespaces packages register at boot are not seen.

use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use mago_allocator::LocalArena;
use mago_database::file::FileType;
use mago_span::HasSpan;
use mago_syntax::cst::*;
use mago_syntax::parser::parse_file;
use pun_tokens::php_compat::preg_compile;

use crate::codebase::read;

/// Files below `dir` whose name matches `/\.blade\.php$/i` (or any file, for `None`), sorted.
fn files(dir: &Path, blade: bool) -> Vec<PathBuf> {
    let mut found = Vec::new();
    let mut entries: Vec<PathBuf> = std::fs::read_dir(dir)
        .into_iter()
        .flatten()
        .flatten()
        .map(|e| e.path())
        .collect();
    // ponytail: sorted, where PHP's directory iterators follow readdir order
    entries.sort();
    for p in entries {
        if p.is_dir() {
            found.extend(files(&p, blade));
        } else if !blade
            || p.to_string_lossy()
                .to_ascii_lowercase()
                .ends_with(".blade.php")
        {
            found.push(p);
        }
    }
    found
}

/// The view finder: its paths and `exists()` / `find()` for a name without a namespace.
pub(crate) struct Finder {
    pub(crate) paths: Vec<PathBuf>,
}

impl Finder {
    pub(crate) fn new(root: &Path) -> Finder {
        Finder {
            paths: vec![root.join("resources/views")],
        }
    }

    /// `FileViewFinder::find()`: the first path holding the name with one of the factory's extensions.
    pub(crate) fn find(&self, name: &str) -> Option<PathBuf> {
        let name = name.trim();
        if name.contains("::") {
            // ponytail: namespaced views need the hints packages register at boot
            return None;
        }
        let file = name.replace('.', "/");
        self.paths.iter().find_map(|path| {
            ["blade.php", "php", "css", "html"]
                .iter()
                .map(|e| path.join(format!("{file}.{e}")))
                .find(|p| p.exists())
        })
    }

    /// `getRootViewFilePaths()`: the blade files below the finder's paths.
    pub(crate) fn root_view_files(&self) -> Vec<PathBuf> {
        self.paths.iter().flat_map(|p| files(p, true)).collect()
    }
}

/// `ViewFileHelper::getAllViewNames()` over `directories`, without vendor and error views.
pub(crate) fn view_names(directories: &[PathBuf]) -> Vec<String> {
    let mut names = Vec::new();
    for dir in directories {
        let prefix = format!("{}/", dir.to_string_lossy().trim_end_matches(['/', '\\']));
        for file in files(dir, true) {
            let path = file.to_string_lossy();
            if path.contains("views/vendor") || path.contains("views/errors") {
                continue;
            }
            let Some(rest) = path.split(&prefix).nth(1) else {
                continue;
            };
            names.push(rest.replace('/', ".").replace(".blade.php", ""));
        }
    }
    names
}

/// The `InlineHTML` statements of a file parsed as PHP (none when it does not parse), with their first lines.
fn inline_html(path: &Path) -> Vec<(String, u32)> {
    let Some(file) = read(path, FileType::Host).ok().flatten() else {
        return Vec::new();
    };
    let arena = LocalArena::new();
    let program = parse_file(&arena, &file);
    if program.has_errors() {
        return Vec::new();
    }
    program
        .statements
        .iter()
        .filter_map(|s| match s {
            Statement::Inline(i) => Some((
                String::from_utf8_lossy(i.value).into_owned(),
                file.line_number(i.span().start.offset) + 1,
            )),
            _ => None,
        })
        .collect()
}

/// UsedViewInAnotherViewCollector: the views `@extends` and `@include*` name in the views below `directories`.
pub(crate) fn views_used_in_views(directories: &[PathBuf]) -> Vec<String> {
    static VIEW: OnceLock<pcre2::bytes::Regex> = OnceLock::new();
    let re = VIEW.get_or_init(|| {
        preg_compile(
            r#"/@(extends|include(If|Unless|When|First)?)(\(.*?(['"])(.*?)(['"])([),]))/m"#,
        )
        .expect("a valid pattern")
    });
    let mut used = Vec::new();
    for file in directories.iter().flat_map(|d| files(d, true)) {
        for (html, _) in inline_html(&file) {
            used.extend(
                re.captures_iter(html.as_bytes())
                    .flatten()
                    .map(|c| String::from_utf8_lossy(&c[5]).into_owned()),
            );
        }
    }
    used
}

/// UsedTranslationViewCollector: each view file's translation keys and lines.
pub(crate) fn translations_in_views(finder: &Finder) -> Vec<(PathBuf, Vec<(String, u32)>)> {
    static TRANSLATION: OnceLock<pcre2::bytes::Regex> = OnceLock::new();
    let re = TRANSLATION.get_or_init(|| {
        preg_compile(
            r#"/
    (
        (
            (?<!\w)
            (trans|trans_choice|Lang::get|Lang::choice|Lang::trans|Lang::transChoice|__|\$t)
        )
        |
        (@lang|@choice)
    )
    \(
    (?P<quote>['"])
    (?P<string>(\\.|(?!(?P=quote))[^\\\\])*?)
    (?P=quote)
    [),]
/mix"#,
        )
        .expect("a valid pattern")
    });
    finder
        .root_view_files()
        .into_iter()
        .map(|file| {
            let mut found = Vec::new();
            for (html, line) in inline_html(&file) {
                for c in re.captures_iter(html.as_bytes()).flatten() {
                    let quote = &c["quote"];
                    let string = String::from_utf8_lossy(&c["string"]).into_owned();
                    let start = c.get(0).map_or(0, |m| m.start());
                    let lines = html.as_bytes()[..start]
                        .iter()
                        .filter(|b| **b == b'\n')
                        .count() as u32;
                    found.push((unescape(quote == b"\"", &string), line + lines));
                }
            }
            (file, found)
        })
        .collect()
}

/// `unescapeMatch()`.
fn unescape(double: bool, s: &str) -> String {
    if double {
        stripcslashes(&s.replace("\\'", "\\\\'"))
    } else {
        s.replace("\\\\", "\\").replace("\\'", "'")
    }
}

/// PHP's `stripcslashes()`.
fn stripcslashes(s: &str) -> String {
    let b = s.as_bytes();
    let mut out = Vec::with_capacity(b.len());
    let mut i = 0;
    while i < b.len() {
        if b[i] != b'\\' || i + 1 == b.len() {
            out.push(b[i]);
            i += 1;
            continue;
        }
        i += 1;
        let c = b[i];
        i += 1;
        match c {
            b'n' => out.push(b'\n'),
            b't' => out.push(b'\t'),
            b'r' => out.push(b'\r'),
            b'a' => out.push(0x07),
            b'v' => out.push(0x0b),
            b'b' => out.push(0x08),
            b'f' => out.push(0x0c),
            b'x' if i < b.len() && b[i].is_ascii_hexdigit() => {
                let end = (i + 2).min(b.len());
                let digits = b[i..end]
                    .iter()
                    .take_while(|d| d.is_ascii_hexdigit())
                    .count();
                out.push(u8::from_str_radix(&s[i..i + digits], 16).unwrap_or_default());
                i += digits;
            }
            b'0'..=b'7' => {
                let end = (i + 2).min(b.len());
                let digits = 1 + b[i..end]
                    .iter()
                    .take_while(|d| (b'0'..=b'7').contains(*d))
                    .count();
                let value = u32::from_str_radix(&s[i - 1..i - 1 + digits], 8).unwrap_or_default();
                out.push(value as u8);
                i += digits - 1;
            }
            c => out.push(c),
        }
    }
    String::from_utf8_lossy(&out).into_owned()
}

/// NoMissingTranslationsRule's available translations: each PHP file's root and keys, each JSON file's keys.
pub(crate) fn available_translations(directories: &[PathBuf]) -> Vec<String> {
    let mut available = Vec::new();
    for dir in directories.iter().filter(|d| d.is_dir()) {
        for file in files(dir, false) {
            let name = file.file_name().unwrap_or_default().to_string_lossy();
            if name.starts_with('.') {
                continue;
            }
            match file.extension().and_then(|e| e.to_str()) {
                Some("php") => {
                    let relative = file.strip_prefix(dir).unwrap_or(&file);
                    let parts: Vec<String> = relative
                        .iter()
                        .map(|p| p.to_string_lossy().into_owned())
                        .collect();
                    // without the locale and the file name
                    let prefix = if parts.len() > 2 {
                        parts[1..parts.len() - 1].join("/")
                    } else {
                        String::new()
                    };
                    let stem = file.file_stem().unwrap_or_default().to_string_lossy();
                    let root = if prefix.is_empty() {
                        stem.into_owned()
                    } else {
                        format!("{prefix}/{stem}")
                    };
                    available.push(root.clone());
                    php_keys(&file, &root, &mut available);
                }
                Some("json") => {
                    if let Ok(serde_json::Value::Object(o)) =
                        serde_json::from_slice(&std::fs::read(&file).unwrap_or_default())
                    {
                        available.extend(o.keys().cloned());
                    }
                }
                _ => {}
            }
        }
    }
    available
}

/// `keys()` of the array a PHP translation file returns (`getRequire()`), read from its `return` literal.
fn php_keys(path: &Path, root: &str, out: &mut Vec<String>) {
    let Some(file) = read(path, FileType::Host).ok().flatten() else {
        return;
    };
    let arena = LocalArena::new();
    let program = parse_file(&arena, &file);
    let Some(value) = program.statements.iter().find_map(|s| match s {
        Statement::Return(r) => r.value,
        _ => None,
    }) else {
        return;
    };
    keys(value, root, out);
}

fn keys(e: &Expression, prefix: &str, out: &mut Vec<String>) {
    let elements = match e {
        Expression::Array(a) => &a.elements,
        Expression::LegacyArray(a) => &a.elements,
        _ => return,
    };
    let mut next = 0i64;
    for element in elements.iter() {
        let (key, value) = match element {
            ArrayElement::KeyValue(kv) => {
                let key = match kv.key {
                    Expression::Literal(Literal::String(s)) => {
                        String::from_utf8_lossy(s.value.unwrap_or_default()).into_owned()
                    }
                    Expression::Literal(Literal::Integer(i)) => {
                        let v = i.value.unwrap_or_default() as i64;
                        next = next.max(v + 1);
                        v.to_string()
                    }
                    _ => continue,
                };
                (key, kv.value)
            }
            ArrayElement::Value(v) => {
                next += 1;
                ((next - 1).to_string(), v.value)
            }
            _ => continue,
        };
        let key = format!("{prefix}.{key}");
        out.push(key.clone());
        keys(value, &key, out);
    }
}
