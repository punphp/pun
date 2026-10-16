//! File discovery that reproduces Pint's finder (PHP-CS-Fixer `Finder` + Pint's `ConfigurationFactory::finder()`
//! + `ConfigurationResolver::resolveFinder()` in path-mode "override"), and Pint's git path selection.
//!
//! Notable upstream semantics kept on purpose:
//! - `.gitignore` is NOT honoured (Symfony `ignoreVCS` only skips VCS metadata dirs).
//! - Excluded names without `/` match a directory of that name at any depth; names with `/` match as path segments.
//! - Explicit file arguments bypass every filter (`Finder::append`).
//! - Symlinked dirs are not followed; symlinked files are listed (Pint counts them) and the runner skips them.

use anyhow::{Context, Result, bail};
use pcre2::bytes::{Regex, RegexBuilder};
use pun_config::Config;
use std::path::{Path, PathBuf};
use std::process::Command;

const PINT_NOT_NAME: [&str; 4] = ["_ide_helper_actions.php", "_ide_helper_models.php", "_ide_helper.php", ".phpstorm.meta.php"];
const PINT_EXCLUDE: [&str; 4] = ["bootstrap/cache", "build", "node_modules", "storage"];
const VCS: [&str; 9] = [".svn", "_svn", "CVS", "_darcs", ".arch-params", ".monotone", ".bzr", ".git", ".hg"];

/// A Symfony Finder configured the way PHP-CS-Fixer and Pint configure it.
struct Finder {
    dirs: Vec<PathBuf>,
    append: Vec<PathBuf>,
    excluded_names: Vec<String>,
    excluded_pattern: Option<Regex>,
    not_names: Vec<Regex>,
    not_paths: Vec<Regex>,
}

impl Finder {
    /// `PhpCsFixer\Finder::create()`: files named `/\.php$/`, exclude vendor, ignore dot files and VCS dirs.
    fn php_cs_fixer() -> Self {
        Finder { dirs: vec![], append: vec![], excluded_names: vec![], excluded_pattern: None, not_names: vec![], not_paths: vec![] }
            .exclude(["vendor"])
    }

    /// `ConfigurationFactory::finder()`.
    fn pint(cfg: &Config) -> Result<Self> {
        let mut not_name: Vec<&str> = PINT_NOT_NAME.to_vec();
        if !cfg.blade {
            not_name.push("*.blade.php");
        }
        let mut f = Self::php_cs_fixer().not_name(not_name)?.exclude(PINT_EXCLUDE);
        // pint.json finder options, applied in Pint's key order (exclude, in, notPath, notName are independent).
        f = f.exclude(cfg.finder.exclude.iter().map(String::as_str));
        for d in &cfg.finder.in_dirs {
            let p = cfg.root.join(d);
            // ponytail: Symfony also globs non-directory `in` values; add when a project uses `in: ["src/*"]`.
            if !p.is_dir() {
                bail!("The \"{d}\" directory does not exist.");
            }
            f.dirs.push(p);
        }
        for p in &cfg.finder.not_path {
            f.not_paths.push(path_regex(p)?);
        }
        f.not_name(cfg.finder.not_name.iter().map(String::as_str))
    }

    fn exclude<'a>(mut self, dirs: impl IntoIterator<Item = &'a str>) -> Self {
        let mut pats: Vec<String> = self.excluded_pattern.take().map(|r| vec![r.as_str().to_string()]).unwrap_or_default();
        for d in dirs {
            let d = d.trim_end_matches('/');
            if d.contains('/') {
                pats.push(format!("(?:^|/)(?:{})(?:/|$)", preg_quote(d)));
            } else {
                self.excluded_names.push(d.to_string());
            }
        }
        if !pats.is_empty() {
            self.excluded_pattern = Some(Regex::new(&pats.join("|")).expect("quoted pattern"));
        }
        self
    }

    fn not_name<'a>(mut self, globs: impl IntoIterator<Item = &'a str>) -> Result<Self> {
        for g in globs {
            self.not_names.push(if is_regex(g) { php_regex(g)? } else { php_regex(&glob_to_regex(g))? });
        }
        Ok(self)
    }

    fn files(&self) -> Result<Vec<PathBuf>> {
        let name = Regex::new(r"\.php$").unwrap();
        let dot = Regex::new(r"(^|/)\..+(/|$)").unwrap();
        let mut out = Vec::new();
        for dir in &self.dirs {
            let mut it = walkdir::WalkDir::new(dir).min_depth(1).into_iter();
            while let Some(entry) = it.next() {
                let entry = entry?;
                let rel = entry.path().strip_prefix(dir).unwrap().to_string_lossy().into_owned();
                // Follows symlinks, like SplFileInfo::isDir(). Symfony's ONLY_FILES only drops directories, so dangling
                // symlinks (and other non-regular files) are listed.
                let is_dir = entry.path().is_dir();
                if is_dir {
                    let fname = entry.file_name().to_string_lossy();
                    let pruned = self.excluded_names.iter().any(|n| *n == fname)
                        || VCS.contains(&fname.as_ref())
                        || self.excluded_pattern.as_ref().is_some_and(|r| r.is_match(rel.as_bytes()).unwrap_or(false));
                    if pruned && entry.file_type().is_dir() {
                        it.skip_current_dir();
                    }
                    continue;
                }
                let fname = entry.file_name().to_string_lossy();
                let m = |r: &Regex, s: &str| r.is_match(s.as_bytes()).unwrap_or(false);
                if !m(&name, &fname) || self.not_names.iter().any(|r| m(r, &fname)) {
                    continue;
                }
                if m(&dot, &rel) || self.not_paths.iter().any(|r| m(r, &rel)) {
                    continue;
                }
                out.push(entry.into_path());
            }
        }
        out.extend(self.append.iter().cloned());
        Ok(out)
    }
}

/// Files Pint would process for `pint [paths]` (empty `paths` = no argument, i.e. the cwd default).
/// Finder order (readdir, depth-first, then appended files), de-duplicated; paths are absolute under `cfg.root`.
pub fn discover(cfg: &Config, paths: &[PathBuf]) -> Result<Vec<PathBuf>> {
    let has_in = !cfg.finder.in_dirs.is_empty();
    let f = if paths.is_empty() && has_in {
        Finder::pint(cfg)?
    } else {
        let paths: Vec<PathBuf> = if paths.is_empty() { vec![cfg.root.clone()] } else { paths.to_vec() };
        // A fully defined finder in the config is replaced, not narrowed (ConfigurationResolver::resolveFinder).
        let mut f = if has_in { Finder::php_cs_fixer() } else { Finder::pint(cfg)? };
        for p in paths {
            let abs = cfg.root.join(&p);
            let real = abs.canonicalize().map_err(|_| anyhow::anyhow!("The path \"{}\" is not readable.", p.display()))?;
            if real.is_file() { f.append.push(real) } else { f.dirs.push(real) }
        }
        f
    };
    Ok(dedup(f.files()?))
}

#[derive(Debug, Clone)]
pub enum GitSelect {
    /// `--dirty`
    Dirty,
    /// `--diff=<branch>`: changed since the merge-base with `<branch>`, plus staged, unstaged and untracked files.
    Diff(String),
    /// `--staged` (Pun): files with staged changes.
    /// ponytail: selects staged files and formats the working tree; index-only formatting comes with M1.3 polish.
    Staged,
    /// `--since=<ref>` (Pun): like `--diff`, but against `<ref>` itself instead of the merge-base.
    Since(String),
}

/// Pint's GitPathsRepository: git's file list intersected with the full finder run in the project root.
/// Git failing (not a repository, unknown branch) selects nothing: Pint's ConfigurationResolverFactory swallows
/// that abort and reports an empty run.
pub fn git_paths(cfg: &Config, sel: &GitSelect) -> Result<Vec<PathBuf>> {
    let lists = |cmds: &[&[&str]]| -> Option<Vec<String>> {
        let mut all = Vec::new();
        for args in cmds {
            all.extend(git(&cfg.root, args)?.lines().filter(|l| !l.is_empty()).map(String::from));
        }
        Some(all)
    };
    let untracked: &[&str] = &["ls-files", "--others", "--exclude-standard", "--", "**.php"];
    let names = match sel {
        GitSelect::Dirty => git(&cfg.root, &["status", "--short", "--", "**.php"]).map(|out| {
            // mapWithKeys(substr($l, 3) => trim(substr($l, 0, 3))); git lists each path once, so no key collides.
            // Only exact "D" / "R" count: " D"/"D " are dropped, "RM" keeps "old -> new" (which then matches nothing).
            out.lines()
                .filter(|l| !l.is_empty())
                .map(|l| (l.get(3..).unwrap_or(""), l.get(..3.min(l.len())).unwrap_or("").trim()))
                .filter(|(_, s)| *s != "D")
                .map(|(f, s)| if s == "R" { f.split_once(" -> ").map_or(f, |x| x.1) } else { f }.to_string())
                .collect()
        }),
        GitSelect::Diff(branch) => lists(&[
            &["diff", "--name-only", "--diff-filter=AMR", &format!("{branch}...HEAD"), "--", "**.php"],
            &["diff", "--name-only", "--diff-filter=AMR", "--cached", "--", "**.php"],
            &["diff", "--name-only", "--diff-filter=AMR", "--", "**.php"],
            untracked,
        ]),
        GitSelect::Staged => lists(&[&["diff", "--name-only", "--diff-filter=AMR", "--cached", "--", "**.php"]]),
        GitSelect::Since(r) => lists(&[&["diff", "--name-only", "--diff-filter=AMR", r, "--", "**.php"], untracked]),
    };
    let wanted: std::collections::HashSet<PathBuf> = names.unwrap_or_default().iter().map(|n| cfg.root.join(n)).collect();
    if wanted.is_empty() {
        return Ok(vec![]);
    }
    let mut f = Finder::pint(cfg)?;
    f.dirs.push(cfg.root.clone());
    let picked: Vec<PathBuf> = f.files()?.into_iter().filter(|p| wanted.contains(p)).collect();
    // Project::paths() hands the list to the resolver like path arguments: symlinks resolve to their targets.
    if picked.is_empty() { Ok(picked) } else { discover(cfg, &picked) }
}

fn dedup(files: Vec<PathBuf>) -> Vec<PathBuf> {
    let mut seen = std::collections::HashSet::new();
    files.into_iter().filter(|p| seen.insert(p.clone())).collect()
}

/// ConfigurationFactory::isPathExcluded (used for `--stdin-filename`): fnmatch-based, not the finder.
pub fn is_path_excluded(cfg: &Config, file: &Path) -> bool {
    let base = cfg.root.to_string_lossy();
    let full = file.to_string_lossy().replace('\\', "/");
    let rel = full.strip_prefix(&format!("{base}/")).unwrap_or(&full).to_string();
    let name = file.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
    let mut not_name: Vec<&str> = PINT_NOT_NAME.to_vec();
    if !cfg.blade {
        not_name.push("*.blade.php");
    }
    let under = |d: &str| {
        let d = d.replace('\\', "/");
        rel == d || rel.starts_with(&format!("{d}/"))
    };
    not_name.iter().copied().chain(cfg.finder.not_name.iter().map(String::as_str)).any(|p| fnmatch(p, &name))
        || PINT_EXCLUDE.iter().copied().chain(cfg.finder.exclude.iter().map(String::as_str)).any(under)
        || cfg.finder.not_path.iter().any(|p| fnmatch(p, &rel))
}

/// PHP fnmatch() with no flags: `*` and `?` also match `/`, `[...]` classes, `\\` escapes.
fn fnmatch(pat: &str, s: &str) -> bool {
    let mut re = String::from("^");
    let mut chars = pat.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '*' => re.push_str("(?s:.*)"),
            '?' => re.push_str("(?s:.)"),
            '[' => {
                let mut class = String::from("[");
                if chars.peek() == Some(&'!') {
                    chars.next();
                    class.push('^');
                }
                for c in chars.by_ref() {
                    if c == '\\' { class.push_str("\\\\") } else { class.push(c) }
                    if c == ']' { break; }
                }
                re.push_str(&class);
            }
            '\\' => re.push_str(&preg_quote(&chars.next().map(String::from).unwrap_or_default())),
            c => re.push_str(&preg_quote(&c.to_string())),
        }
    }
    re.push('$');
    Regex::new(&re).and_then(|r| r.is_match(s.as_bytes())).unwrap_or(false)
}

fn git(root: &Path, args: &[&str]) -> Option<String> {
    let out = Command::new("git").args(args).current_dir(root).output().ok().filter(|o| o.status.success())?;
    // preg_split('/\R+/') — CR alone also separates.
    Some(String::from_utf8_lossy(&out.stdout).replace('\r', "\n"))
}

/// Symfony `MultiplePcreFilterIterator::isRegex`.
fn is_regex(s: &str) -> bool {
    // preg_match('/^(.{3,}?)[imsxuADUn]*$/'): shortest body of >= 3 bytes followed only by modifiers.
    let b = s.as_bytes();
    if b.len() < 3 {
        return false;
    }
    let body = &b[..s.trim_end_matches(|c| "imsxuADUn".contains(c)).len().max(3)];
    let (start, end) = (body[0], body[body.len() - 1]);
    if start == end {
        return !(start.is_ascii_alphanumeric() || b"*? \\".contains(&start));
    }
    matches!((start, end), (b'{', b'}') | (b'(', b')') | (b'[', b']') | (b'<', b'>'))
}

/// Compile a PHP `preg_*` pattern (delimiters + modifiers) with PCRE2.
fn php_regex(p: &str) -> Result<Regex> {
    let open = p.chars().next().context("empty regex")?;
    let close = match open {
        '(' => ')',
        '{' => '}',
        '[' => ']',
        '<' => '>',
        c => c,
    };
    let end = p.rfind(close).filter(|&i| i > 0).with_context(|| format!("no ending delimiter in {p}"))?;
    let (body, mods) = (&p[1..end], &p[end + 1..]);
    let mut b = RegexBuilder::new();
    for m in mods.chars() {
        match m {
            'i' => b.caseless(true),
            'm' => b.multi_line(true),
            's' => b.dotall(true),
            'x' => b.extended(true),
            'u' => b.utf(true).ucp(true),
            _ => &mut b,
        };
    }
    Ok(b.build(body)?)
}

/// Symfony PathFilterIterator::toRegex.
fn path_regex(s: &str) -> Result<Regex> {
    if is_regex(s) { php_regex(s) } else { Ok(Regex::new(&preg_quote(s))?) }
}

fn preg_quote(s: &str) -> String {
    let mut o = String::with_capacity(s.len());
    for c in s.chars() {
        if ".\\+*?[^]$(){}=!<>|:-#/".contains(c) {
            o.push('\\');
        }
        o.push(c);
    }
    o
}

/// Symfony `Glob::toRegex($glob)` with defaults (strict leading dot, strict wildcard slash, `#` delimiter).
fn glob_to_regex(glob: &str) -> String {
    let unanchored = glob.starts_with("**/");
    let glob = if unanchored { format!("/{glob}") } else { glob.to_string() };
    let g = glob.as_bytes();
    let (mut first, mut escaping, mut curlies) = (true, false, 0);
    let mut re = String::new();
    let mut i = 0;
    while i < g.len() {
        let mut car = (g[i] as char).to_string();
        if first && g[i] != b'.' {
            re.push_str(r"(?=[^\.])");
        }
        first = g[i] == b'/';
        if first && g.len() > i + 2 && &g[i + 1..i + 3] == b"**" && (g.len() == i + 3 || g[i + 3] == b'/') {
            let mut c = "[^/]++/".to_string();
            if g.len() == i + 3 {
                c.push('?');
            }
            car = format!("/(?:(?=[^\\.]){c})*");
            i += 2 + usize::from(g.len() > i + 3);
            re.push_str(&car);
            i += 1;
            escaping = false;
            continue;
        }
        match car.as_str() {
            "#" | "." | "(" | ")" | "|" | "+" | "^" | "$" => re.push_str(&format!("\\{car}")),
            "*" => re.push_str(if escaping { "\\*" } else { "[^/]*" }),
            "?" => re.push_str(if escaping { "\\?" } else { "[^/]" }),
            "{" => {
                re.push_str(if escaping { "\\{" } else { "(" });
                if !escaping {
                    curlies += 1;
                }
            }
            "}" if curlies > 0 => {
                re.push_str(if escaping { "}" } else { ")" });
                if !escaping {
                    curlies -= 1;
                }
            }
            "," if curlies > 0 => re.push_str(if escaping { "," } else { "|" }),
            "\\" => {
                if escaping {
                    re.push_str("\\\\");
                    escaping = false;
                } else {
                    escaping = true;
                }
                i += 1;
                continue;
            }
            _ => re.push_str(&car),
        }
        escaping = false;
        i += 1;
    }
    if unanchored {
        re.insert(1 + "(?=[^\\.])".len(), '?');
    }
    format!("#^{re}$#")
}
