//! The bits of PHP and PHPStan's file helpers the config and ignore logic run on: `preg_*` patterns (delimiters,
//! modifiers and PHP's error text, as Nette's `Strings::match` reports it), `preg_quote`, `fnmatch`, and
//! `PHPStan\File\FileHelper` / `FileExcluder`.

use pcre2::bytes::{Regex, RegexBuilder};

/// Compile a PHP regex (`#...#flags`). `Err` is Nette's `RegexpException` message.
pub fn preg(pattern: &str) -> Result<Regex, String> {
    compile(pattern).map_err(|m| format!("{m} in pattern: {pattern}"))
}

fn compile(pattern: &str) -> Result<Regex, String> {
    let p = pattern.trim_start_matches([' ', '\t', '\n', '\r', '\x0B', '\x0C']);
    let Some(delimiter) = p.chars().next() else {
        return Err("Empty regular expression".into());
    };
    if delimiter.is_ascii_alphanumeric() || delimiter == '\\' || delimiter == '\0' {
        return Err("Delimiter must not be alphanumeric, backslash, or NUL byte".into());
    }
    let rest = &p[delimiter.len_utf8()..];
    let close = match delimiter {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        d => d,
    };
    let mut depth = 1;
    let mut end = None;
    let mut chars = rest.char_indices();
    while let Some((i, c)) = chars.next() {
        if c == '\\' {
            chars.next();
        } else if c == close && close != delimiter {
            depth -= 1;
            if depth == 0 {
                end = Some(i);
                break;
            }
        } else if c == close {
            end = Some(i);
            break;
        } else if c == delimiter {
            depth += 1;
        }
    }
    let Some(end) = end else {
        return Err(if close == delimiter {
            format!("No ending delimiter '{delimiter}' found")
        } else {
            format!("No ending matching delimiter '{close}' found")
        });
    };
    let (body, modifiers) = (&rest[..end], &rest[end + close.len_utf8()..]);
    let mut b = RegexBuilder::new();
    let mut flags = String::new();
    let mut anchored = false;
    let mut extended = false;
    for m in modifiers.chars() {
        match m {
            'i' => _ = b.caseless(true),
            'm' => _ = b.multi_line(true),
            's' => _ = b.dotall(true),
            'x' => {
                b.extended(true);
                extended = true;
            }
            'u' => _ = b.utf(true).ucp(true),
            'A' => anchored = true,
            'U' => flags.push_str("(?U)"),
            'J' => flags.push_str("(?J)"),
            'n' => flags.push_str("(?n)"),
            // ponytail: D (dollar end-only) and r (caseless-restrict) aren't exposed by the pcre2 crate; S/X are no-ops.
            'D' | 'S' | 'X' | 'r' | ' ' | '\n' | '\r' => {}
            '\0' => return Err("NUL byte is not a valid modifier".into()),
            m => return Err(format!("Unknown modifier '{m}'")),
        }
    }
    // Compiled as written first, so error offsets are PHP's; `A` (PCRE2_ANCHORED) then wraps it after `\G`.
    let re = b.build(&format!("{flags}{body}")).map_err(|e| {
        let at = e.offset().unwrap_or(0).saturating_sub(flags.len());
        let text = e.to_string();
        let msg = text
            .split_once("pattern at offset ")
            .and_then(|(_, r)| r.split_once(": "))
            .map_or(text.as_str(), |(_, m)| m);
        format!("Compilation failed: {msg} at offset {at}")
    })?;
    if !anchored {
        return Ok(re);
    }
    // (A newline ends an `x`-mode comment before the closing parenthesis.)
    let close = if extended { "\n)" } else { ")" };
    Ok(b.build(&format!("{flags}\\G(?:{body}{close}"))
        .expect("compiles unanchored"))
}

/// `preg_match($pattern, $subject) === 1` once compiled; matching errors (e.g. invalid UTF-8 under `/u`) count as no match.
pub fn is_match(re: &Regex, subject: &str) -> bool {
    re.is_match(subject.as_bytes()).unwrap_or(false)
}

/// `preg_quote($s, $delimiter)`.
pub fn preg_quote(s: &str, delimiter: Option<char>) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        if c == '\0' {
            out.push_str("\\000");
            continue;
        }
        if ".\\+*?[^]$(){}=!<>|:-#/".contains(c) && (c != '/' || delimiter == Some('/'))
            || Some(c) == delimiter
        {
            out.push('\\');
        }
        out.push(c);
    }
    out
}

/// PHP `fnmatch($pattern, $string, 0)`: the C library's fnmatch.
pub fn fnmatch(pattern: &str, string: &str) -> bool {
    use std::ffi::CString;
    // (PHP refuses NUL bytes; nothing matches them here)
    CString::new(pattern)
        .ok()
        .zip(CString::new(string).ok())
        // SAFETY: both are valid NUL-terminated C strings for the duration of the call.
        .is_some_and(|(p, s)| unsafe { libc::fnmatch(p.as_ptr(), s.as_ptr(), 0) == 0 })
}

/// `FileHelper::normalizePath()` with `/` as the directory separator.
pub fn normalize_path(original: &str) -> String {
    let local = original.starts_with('/') || (original.len() >= 3 && &original[1..3] == ":\\");
    let (mut scheme, path) = match original.split_once("://") {
        Some((s, p))
            if !local
                && !s.is_empty()
                && !p.is_empty()
                && s.bytes()
                    .all(|c| c.is_ascii_alphanumeric() || b"+-.".contains(&c)) =>
        {
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
    path.split_once("://").is_some_and(|(s, _)| {
        !s.is_empty()
            && s.bytes()
                .all(|c| c.is_ascii_alphanumeric() || b"+-.".contains(&c))
    })
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
