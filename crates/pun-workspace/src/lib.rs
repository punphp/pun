//! File discovery that reproduces Pint's finder (PHP-CS-Fixer `Finder` + Pint's `ConfigurationFactory::finder()`
//! + `ConfigurationResolver::resolveFinder()` in path-mode "override"), and Pint's git path selection.
//!
//! Notable upstream semantics kept on purpose:
//! - `.gitignore` is NOT honoured (Symfony `ignoreVCS` only skips VCS metadata dirs).
//! - Excluded names without `/` match a directory of that name at any depth; names with `/` match as path segments.
//! - Explicit file arguments bypass every filter (`Finder::append`).
//! - Symlinked dirs are not followed; symlinked files are listed (Pint counts them) and the runner skips them.
//!
//! On top of Pint's finder, pun.toml's `[files]` and per-command `exclude` (context/plan/config.md §3, §7): an
//! exclude entry with a glob metacharacter (`*`, `?`, `[`, `{`) is a Mago glob matched against the path relative to
//! the project root. For `format` a plain entry keeps Symfony's `exclude()` meaning above; for every other file set a
//! plain entry is Mago's root-relative path prefix. See [`files`].

use anyhow::Result;
use mago_database::GlobSettings;
use mago_database::matcher::{ExclusionMatcher, contains_glob_metacharacters};
use pcre2::bytes::Regex;
use pun_config::Config;
use pun_tokens::php_compat::{fnmatch, preg_compile, preg_quote};
use std::path::{Path, PathBuf};

const PINT_NOT_NAME: [&str; 4] = [
    "_ide_helper_actions.php",
    "_ide_helper_models.php",
    "_ide_helper.php",
    ".phpstorm.meta.php",
];
/// Pint's `notName` list, plus `*.blade.php` unless Blade formatting is on.
fn pint_not_name(cfg: &Config) -> Vec<&'static str> {
    let mut not_name = PINT_NOT_NAME.to_vec();
    if !cfg.blade {
        not_name.push("*.blade.php");
    }
    not_name
}

/// An explicit path, resolved against the root and canonicalized, or Pint's "not readable" error.
fn real_path(cfg: &Config, p: &Path) -> Result<PathBuf, FinderError> {
    cfg.root
        .join(p)
        .canonicalize()
        .map_err(|_| FinderError::NotReadable(p.to_path_buf()))
}

/// Pint's default excludes (compiled views, the config cache, build output, Node packages). `pun lint` and
/// `pun analyse` skip them too, besides pun.toml's excludes, matched as root-relative prefixes.
const PINT_EXCLUDE: [&str; 4] = ["bootstrap/cache", "build", "node_modules", "storage"];
const VCS: [&str; 9] = [
    ".svn",
    "_svn",
    "CVS",
    "_darcs",
    ".arch-params",
    ".monotone",
    ".bzr",
    ".git",
    ".hg",
];

/// A path the finder can't use, worded as the exception Pint throws for it. Callers branch on the variant (pun-cli
/// renders Pint's box from it), never on the text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FinderError {
    /// A path argument that doesn't resolve: ConfigurationResolver's InvalidConfigurationException.
    NotReadable(PathBuf),
    /// An `in` directory that doesn't exist: Symfony Finder's DirectoryNotFoundException.
    DirectoryNotFound(String),
}

impl std::fmt::Display for FinderError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FinderError::NotReadable(p) => {
                write!(f, "The path \"{}\" is not readable.", p.display())
            }
            FinderError::DirectoryNotFound(d) => write!(f, "The \"{d}\" directory does not exist."),
        }
    }
}

impl std::error::Error for FinderError {}

/// A Symfony Finder configured the way PHP-CS-Fixer and Pint configure it.
struct Finder {
    dirs: Vec<PathBuf>,
    append: Vec<PathBuf>,
    excluded_names: Vec<String>,
    excluded_pattern: Option<Regex>,
    not_names: Vec<Regex>,
    not_paths: Vec<Regex>,
    /// pun.toml excludes matched Mago's way against the path relative to `root`.
    matcher: Option<(PathBuf, ExclusionMatcher<String>)>,
}

impl Finder {
    /// `PhpCsFixer\Finder::create()`: files named `/\.php$/`, exclude vendor, ignore dot files and VCS dirs.
    fn php_cs_fixer() -> Self {
        Finder {
            dirs: vec![],
            append: vec![],
            excluded_names: vec![],
            excluded_pattern: None,
            not_names: vec![],
            not_paths: vec![],
            matcher: None,
        }
        .exclude(["vendor"])
    }

    /// `ConfigurationFactory::finder()`.
    fn pint(cfg: &Config) -> Result<Self> {
        let mut f = Self::php_cs_fixer()
            .not_name(pint_not_name(cfg))?
            .exclude(PINT_EXCLUDE);
        // pint.json finder options, applied in Pint's key order (exclude, in, notPath, notName are independent).
        // Globs are pun.toml's own (Pint has none): Mago's root-relative match.
        let (globs, plain): (Vec<&String>, Vec<&String>) =
            (cfg.finder.exclude.iter()).partition(|e| contains_glob_metacharacters(e.as_str()));
        f = f
            .exclude(plain.into_iter().map(String::as_str))
            .matching(&cfg.root, globs.into_iter().cloned())?;
        for d in &cfg.finder.in_dirs {
            let p = cfg.root.join(d);
            // ponytail: Symfony also globs non-directory `in` values; add when a project uses `in: ["src/*"]`.
            if !p.is_dir() {
                return Err(FinderError::DirectoryNotFound(d.clone()).into());
            }
            f.dirs.push(p);
        }
        for p in &cfg.finder.not_path {
            f.not_paths.push(path_regex(p)?);
        }
        f.not_name(cfg.finder.not_name.iter().map(String::as_str))
    }

    fn exclude<'a>(mut self, dirs: impl IntoIterator<Item = &'a str>) -> Self {
        let mut pats: Vec<String> = self
            .excluded_pattern
            .take()
            .map(|r| vec![r.as_str().to_string()])
            .unwrap_or_default();
        for d in dirs {
            let d = d.trim_end_matches('/');
            if d.contains('/') {
                pats.push(format!("(?:^|/)(?:{})(?:/|$)", quote(d)));
            } else {
                self.excluded_names.push(d.to_string());
            }
        }
        if !pats.is_empty() {
            self.excluded_pattern = Some(Regex::new(&pats.join("|")).expect("quoted pattern"));
        }
        self
    }

    /// Mago's `ExclusionMatcher` over `patterns`, relative to `root`; none when there are no patterns.
    fn matching(mut self, root: &Path, patterns: impl IntoIterator<Item = String>) -> Result<Self> {
        let m = ExclusionMatcher::compile(patterns, GlobSettings::default())
            .map_err(|e| anyhow::anyhow!("invalid exclude: {e}"))?;
        self.matcher = (!m.is_empty()).then(|| (root.to_path_buf(), m));
        Ok(self)
    }

    /// The path relative to the project root matches a pun.toml exclude.
    fn matched(&self, path: &Path) -> bool {
        self.matcher.as_ref().is_some_and(|(root, m)| {
            m.is_match(&path.strip_prefix(root).unwrap_or(path).to_string_lossy())
        })
    }

    fn not_name<'a>(mut self, globs: impl IntoIterator<Item = &'a str>) -> Result<Self> {
        for g in globs {
            self.not_names.push(if is_regex(g) {
                php_regex(g)?
            } else {
                php_regex(&glob_to_regex(g))?
            });
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
                let rel = entry
                    .path()
                    .strip_prefix(dir)
                    .unwrap()
                    .to_string_lossy()
                    .into_owned();
                // Follows symlinks, like SplFileInfo::isDir(). Symfony's ONLY_FILES only drops directories, so dangling
                // symlinks (and other non-regular files) are listed.
                let is_dir = entry.path().is_dir();
                if is_dir {
                    let fname = entry.file_name().to_string_lossy();
                    let pruned = self.excluded_names.iter().any(|n| *n == fname)
                        || VCS.contains(&fname.as_ref())
                        || self
                            .excluded_pattern
                            .as_ref()
                            .is_some_and(|r| r.is_match(rel.as_bytes()).unwrap_or(false))
                        || self.matched(entry.path());
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
                if m(&dot, &rel)
                    || self.not_paths.iter().any(|r| m(r, &rel))
                    || self.matched(entry.path())
                {
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
        let paths: Vec<PathBuf> = if paths.is_empty() {
            vec![cfg.root.clone()]
        } else {
            paths.to_vec()
        };
        // A fully defined finder in the config is replaced, not narrowed (ConfigurationResolver::resolveFinder).
        let mut f = if has_in {
            Finder::php_cs_fixer()
        } else {
            Finder::pint(cfg)?
        };
        for p in paths {
            push(&mut f, real_path(cfg, &p)?);
        }
        f
    };
    let mut seen = std::collections::HashSet::new();
    Ok((f.files()?)
        .into_iter()
        .filter(|p| seen.insert(p.clone()))
        .collect())
}

/// The command whose file set [`files`] lists (`pun list-files --command`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    Format,
    Lint,
    Analyse,
}

/// The files a command checks, sorted by path (`pun list-files`, context/plan/config.md §7). Without a command:
/// the `[files]` set, `[files] paths` (or `paths`) walked like PHP-CS-Fixer's `Finder::create()` (`*.php`, no
/// `vendor`, dot or VCS directories) minus `[files] exclude`. `Lint` and `Analyse` add their own `exclude` (and
/// analyse's `paths`) and skip Blade templates and the directories Pint skips, as root-relative prefixes
/// ([`PINT_EXCLUDE`]); `Format` is Pint's finder, [`discover`]. Outside `Format` an explicit file is excluded like
/// any other, as in Mago.
pub fn files(cfg: &Config, command: Option<Command>, paths: &[PathBuf]) -> Result<Vec<PathBuf>> {
    let dirs = match command {
        Some(Command::Format) => {
            let mut out = discover(cfg, paths)?;
            out.sort();
            return Ok(out);
        }
        None | Some(Command::Lint) => &cfg.files.paths,
        Some(Command::Analyse) => &cfg.analyse.paths,
    };
    let mut f = Finder::php_cs_fixer().matching(&cfg.root, exclude_patterns(cfg, command))?;
    if command.is_some() {
        f = f.not_name(["*.blade.php"])?;
    }
    if paths.is_empty() {
        for d in dirs {
            let p = cfg.root.join(d);
            if !p.exists() {
                return Err(FinderError::DirectoryNotFound(d.clone()).into());
            }
            push(&mut f, p);
        }
        if dirs.is_empty() {
            f.dirs.push(cfg.root.clone());
        }
    } else {
        for p in paths {
            push(&mut f, real_path(cfg, p)?);
        }
    }
    let mut out: Vec<PathBuf> = f.files()?;
    out.retain(|p| !f.matched(p));
    out.sort();
    out.dedup();
    Ok(out)
}

/// A command's exclude patterns: `[files] exclude` and its own, plus [`PINT_EXCLUDE`] for lint and analyse.
fn exclude_patterns(cfg: &Config, command: Option<Command>) -> Vec<String> {
    let own = match command {
        None | Some(Command::Format) => &cfg.files.exclude,
        Some(Command::Lint) => &cfg.lint.exclude,
        Some(Command::Analyse) => &cfg.analyse.exclude,
    };
    let defaults = command.map_or(&[][..], |_| &PINT_EXCLUDE[..]);
    (own.iter().cloned())
        .chain(defaults.iter().map(ToString::to_string))
        .collect()
}

/// A command's pun.toml excludes as a test on absolute paths: `pun analyse` drops excluded files from the symbol
/// scan as well as from the analysis (`excludePaths.analyseAndScan`).
pub struct Exclusions(Finder);

impl Exclusions {
    pub fn new(cfg: &Config, command: Command) -> Result<Exclusions> {
        Self::globs(&cfg.root, exclude_patterns(cfg, Some(command)))
    }

    /// Paths matching any of `patterns` (Mago's globs and root-relative prefixes): `[[lint.overrides]] files`.
    pub fn globs(root: &Path, patterns: Vec<String>) -> Result<Exclusions> {
        Ok(Exclusions(Finder::php_cs_fixer().matching(root, patterns)?))
    }

    pub fn is_excluded(&self, path: &Path) -> bool {
        self.0.matched(path)
    }
}

fn push(f: &mut Finder, p: PathBuf) {
    if p.is_file() {
        f.append.push(p);
    } else {
        f.dirs.push(p);
    }
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
            all.extend(
                git(&cfg.root, args)?
                    .lines()
                    .filter(|l| !l.is_empty())
                    .map(String::from),
            );
        }
        Some(all)
    };
    // `git diff --name-only --diff-filter=AMR [rev] -- **.php`.
    fn diff(rev: Option<&str>) -> Vec<&str> {
        ["diff", "--name-only", "--diff-filter=AMR"]
            .into_iter()
            .chain(rev)
            .chain(["--", "**.php"])
            .collect()
    }
    let untracked: &[&str] = &["ls-files", "--others", "--exclude-standard", "--", "**.php"];
    let names = match sel {
        GitSelect::Dirty => git(&cfg.root, &["status", "--short", "--", "**.php"]).map(|out| {
            // mapWithKeys(substr($l, 3) => trim(substr($l, 0, 3))); git lists each path once, so no key collides.
            // Only exact "D" / "R" count: " D"/"D " are dropped, "RM" keeps "old -> new" (which then matches nothing).
            out.lines()
                .filter(|l| !l.is_empty())
                .map(|l| {
                    (
                        l.get(3..).unwrap_or(""),
                        l.get(..3.min(l.len())).unwrap_or("").trim(),
                    )
                })
                .filter(|(_, s)| *s != "D")
                .map(|(f, s)| {
                    if s == "R" {
                        f.split_once(" -> ").map_or(f, |x| x.1)
                    } else {
                        f
                    }
                    .to_string()
                })
                .collect()
        }),
        GitSelect::Diff(branch) => lists(&[
            &diff(Some(&format!("{branch}...HEAD"))),
            &diff(Some("--cached")),
            &diff(None),
            untracked,
        ]),
        GitSelect::Staged => lists(&[&diff(Some("--cached"))]),
        GitSelect::Since(r) => lists(&[&diff(Some(r)), untracked]),
    };
    let wanted: std::collections::HashSet<PathBuf> = names
        .unwrap_or_default()
        .iter()
        .map(|n| cfg.root.join(n))
        .collect();
    if wanted.is_empty() {
        return Ok(vec![]);
    }
    let mut f = Finder::pint(cfg)?;
    f.dirs.push(cfg.root.clone());
    let picked: Vec<PathBuf> = f
        .files()?
        .into_iter()
        .filter(|p| wanted.contains(p))
        .collect();
    // Project::paths() hands the list to the resolver like path arguments: symlinks resolve to their targets.
    if picked.is_empty() {
        Ok(picked)
    } else {
        discover(cfg, &picked)
    }
}

/// ConfigurationFactory::isPathExcluded (used for `--stdin-filename`): fnmatch-based, not the finder.
pub fn is_path_excluded(cfg: &Config, file: &Path) -> bool {
    let base = cfg.root.to_string_lossy();
    let full = file.to_string_lossy().replace('\\', "/");
    let rel = full
        .strip_prefix(&format!("{base}/"))
        .unwrap_or(&full)
        .to_string();
    let name = file
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();
    let not_name = pint_not_name(cfg);
    let under = |d: &str| {
        let d = d.replace('\\', "/");
        rel == d || rel.starts_with(&format!("{d}/"))
    };
    not_name
        .iter()
        .copied()
        .chain(cfg.finder.not_name.iter().map(String::as_str))
        .any(|p| fnmatch(p.as_bytes(), name.as_bytes()))
        || PINT_EXCLUDE
            .iter()
            .copied()
            .chain(cfg.finder.exclude.iter().map(String::as_str))
            .any(under)
        || cfg
            .finder
            .not_path
            .iter()
            .any(|p| fnmatch(p.as_bytes(), rel.as_bytes()))
        || (cfg.finder.exclude.iter())
            .any(|e| contains_glob_metacharacters(e) && glob_matches(e, &rel))
}

/// One pun.toml glob exclude against a root-relative path, as Mago matches it.
fn glob_matches(glob: &str, rel: &str) -> bool {
    ExclusionMatcher::compile([glob], GlobSettings::default()).is_ok_and(|m| m.is_match(rel))
}

fn git(root: &Path, args: &[&str]) -> Option<String> {
    let out = std::process::Command::new("git")
        .args(args)
        .current_dir(root)
        .output()
        .ok()
        .filter(|o| o.status.success())?;
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
    matches!(
        (start, end),
        (b'{', b'}') | (b'(', b')') | (b'[', b']') | (b'<', b'>')
    )
}

/// Compile a PHP `preg_*` pattern; PHP's warning stops Pint (Laravel turns it into an exception).
fn php_regex(p: &str) -> Result<Regex> {
    preg_compile(p).map_err(|m| anyhow::anyhow!("preg_match(): {m}"))
}

/// Symfony PathFilterIterator::toRegex.
fn path_regex(s: &str) -> Result<Regex> {
    if is_regex(s) {
        php_regex(s)
    } else {
        Ok(Regex::new(&quote(s))?)
    }
}

/// `preg_quote($s, '/')` as a pattern string (`$s` is UTF-8, so the result is too).
fn quote(s: &str) -> String {
    String::from_utf8_lossy(&preg_quote(s.as_bytes(), Some(b'/'))).into_owned()
}

/// Symfony `Glob::toRegex($glob)` with defaults (strict leading dot, strict wildcard slash, `#` delimiter).
fn glob_to_regex(glob: &str) -> String {
    let unanchored = glob.starts_with("**/");
    let glob = if unanchored {
        format!("/{glob}")
    } else {
        glob.to_string()
    };
    let g = glob.as_bytes();
    let (mut first, mut escaping, mut curlies) = (true, false, 0);
    let mut re = String::new();
    let mut i = 0;
    while i < g.len() {
        // Whole UTF-8 characters: they are literal bytes in the (non-`u`) regex, as in PHP.
        let ch = glob[i..].chars().next().unwrap();
        let mut car = ch.to_string();
        if first && g[i] != b'.' {
            re.push_str(r"(?=[^\.])");
        }
        first = g[i] == b'/';
        if first
            && g.len() > i + 2
            && &g[i + 1..i + 3] == b"**"
            && (g.len() == i + 3 || g[i + 3] == b'/')
        {
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
            "#" | "." | "(" | ")" | "|" | "+" | "^" | "$" => {
                re.push('\\');
                re.push_str(&car);
            }
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
        i += ch.len_utf8();
    }
    if unanchored {
        re.insert(1 + "(?=[^\\.])".len(), '?');
    }
    format!("#^{re}$#")
}
