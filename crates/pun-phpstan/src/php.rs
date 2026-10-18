//! The bits of PHP and PHPStan's file helpers the config and ignore logic run on: `preg_*` patterns (delimiters,
//! modifiers and PHP's error text, as Nette's `Strings::match` reports it), `preg_quote`, `fnmatch`, and
//! `PHPStan\File\FileHelper` / `FileExcluder`.

use pcre2::bytes::Regex;
use pun_tokens::php_compat;

/// Compile a PHP regex (`#...#flags`). `Err` is Nette's `RegexpException` message.
pub fn preg(pattern: &str) -> Result<Regex, String> {
    php_compat::preg_compile(pattern).map_err(|m| format!("{m} in pattern: {pattern}"))
}

/// `preg_match($pattern, $subject) === 1` once compiled; matching errors (e.g. invalid UTF-8 under `/u`) count as no match.
pub fn is_match(re: &Regex, subject: &str) -> bool {
    re.is_match(subject.as_bytes()).unwrap_or(false)
}

/// `preg_quote($s, $delimiter)` (the delimiter's first byte, as PHP reads it).
pub fn preg_quote(s: &str, delimiter: Option<char>) -> String {
    let d = delimiter.map(|d| d.to_string().as_bytes()[0]);
    String::from_utf8_lossy(&php_compat::preg_quote(s.as_bytes(), d)).into_owned()
}

/// PHP `fnmatch($pattern, $string, 0)`.
pub fn fnmatch(pattern: &str, string: &str) -> bool {
    php_compat::fnmatch(pattern.as_bytes(), string.as_bytes())
}

/// `FileHelper::normalizePath()` with `/` as the directory separator.
pub fn normalize_path(original: &str) -> String {
    let local = original.starts_with('/') || original.as_bytes().get(1..3) == Some(b":\\");
    let (mut scheme, path) = match original.split_once("://") {
        Some((s, p)) if !local && !p.is_empty() && is_scheme(s) => {
            (Some(s.to_ascii_lowercase()), p)
        }
        _ => (None, original),
    };
    let path = path.replace('\\', "/").replace("//", "/");
    let root = if path.starts_with('/') { "/" } else { "" };
    let mut parts: Vec<&str> = Vec::new();
    for part in path.trim_matches('/').split('/') {
        match part {
            "." => {}
            ".." => {
                let removed = parts.pop();
                if scheme.as_deref() == Some("phar")
                    && removed.is_some_and(|r| r.ends_with(".phar"))
                {
                    scheme = None;
                }
            }
            p => parts.push(p),
        }
    }
    format!(
        "{}{root}{}",
        scheme.map(|s| s + "://").unwrap_or_default(),
        parts.join("/")
    )
}

/// `FileHelper::absolutizePath()` relative to `cwd`.
pub fn absolutize_path(cwd: &str, path: &str) -> String {
    if path.starts_with('/') || has_scheme(path) {
        return path.to_string();
    }
    format!(
        "{}/{}",
        cwd.trim_end_matches(['/', '\\']),
        path.trim_start_matches(['/', '\\'])
    )
}

/// `~^[a-z0-9+\-.]+://~i`
fn has_scheme(path: &str) -> bool {
    path.split_once("://").is_some_and(|(s, _)| is_scheme(s))
}

/// `[a-z0-9+\-.]+` (case-insensitive): a URL scheme before `://`.
fn is_scheme(s: &str) -> bool {
    !s.is_empty()
        && s.bytes()
            .all(|c| c.is_ascii_alphanumeric() || b"+-.".contains(&c))
}

/// `FileExcluder::isFnmatchPattern()`.
pub fn is_fnmatch_pattern(path: &str) -> bool {
    path.contains(['*', '?', '[', ']'])
}

/// `FileExcluder` over one or more `excludePaths`/`ignoreErrors.path` entries (literal files, directories, fnmatch).
pub struct FileExcluder {
    files: Vec<String>,
    dirs: Vec<String>,
    patterns: Vec<String>,
}

impl FileExcluder {
    pub fn new<'a>(excludes: impl IntoIterator<Item = &'a str>) -> FileExcluder {
        let mut ex = FileExcluder {
            files: Vec::new(),
            dirs: Vec::new(),
            patterns: Vec::new(),
        };
        for exclude in excludes {
            let trailing = exclude.ends_with(['/', '\\']);
            let mut normalized = normalize_path(exclude);
            if trailing {
                normalized.push('/');
            }
            if is_fnmatch_pattern(&normalized) {
                ex.patterns.push(normalized);
            } else if std::path::Path::new(&normalized).is_file() {
                ex.files.push(normalized);
            } else if std::path::Path::new(&normalized).is_dir() {
                if !trailing {
                    normalized.push('/');
                }
                ex.dirs.push(normalized);
            }
        }
        ex
    }

    pub fn is_excluded(&self, file: &str) -> bool {
        let file = normalize_path(file);
        self.files.contains(&file)
            || self.dirs.iter().any(|d| file.starts_with(d.as_str()))
            || self.patterns.iter().any(|p| fnmatch(p, &file))
    }
}

/// `ParentDirectoryRelativePathHelper($parent)->getRelativePath($filename)`.
pub fn parent_relative_path(parent: &str, filename: &str) -> String {
    let filename = filename
        .find("://")
        .map_or(filename, |p| &filename[p + 3..]);
    let parent = parent.replace('\\', "/");
    let file = filename.replace('\\', "/");
    let pp: Vec<&str> = parent.trim_matches('/').split('/').collect();
    let fp: Vec<&str> = file.trim_matches('/').split('/').collect();
    let mut i = 0;
    while i < fp.len() && i < pp.len() && pp[..=i] == fp[..=i] {
        i += 1;
    }
    if i == 0 {
        return filename.to_string();
    }
    let mut parts = vec![".."; pp.len() - i];
    parts.extend(&fp[i..]);
    parts.join("/")
}
