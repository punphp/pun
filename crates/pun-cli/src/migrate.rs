//! `pun migrate`: finds the Pint, PHP-CS-Fixer, PHPStan, Rector and Mago configs, asks which to migrate (a checklist
//! on a terminal; `--from`/`--yes`, agent mode or no terminal don't ask), and migrates the chosen ones it can. Today
//! that is the format part: pint.json or `.php-cs-fixer(.dist).php` becomes pun.toml, and `@php-cs-fixer-ignore`
//! comments become `@pun-ignore-file` with Pun rule IDs (context/plan/rules.md §4). PHPStan, Rector and Mago are listed and
//! left untouched. With no config at all it writes a starter pun.toml.
use crate::{agent, report, root};
use anyhow::{Context, Result, bail};
use pcre2::bytes::Regex;
use pun_config::migrate;
use std::fmt::Write as _;
use std::io::IsTerminal;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

#[derive(clap::Args, Default)]
pub struct MigrateArgs {
    /// Print what would change, as a diff, and write nothing.
    #[arg(long)]
    dry_run: bool,
    /// The pint.json to read (default: ./pint.json).
    #[arg(long)]
    config: Option<PathBuf>,
    /// The preset to use instead of pint.json's, as `pint --preset` does.
    #[arg(long)]
    preset: Option<String>,
    /// Migrate only these, without asking (comma-separated).
    #[arg(long, value_enum, value_delimiter = ',')]
    from: Vec<Source>,
    /// Don't ask: migrate everything detected.
    #[arg(long, short)]
    yes: bool,
}

/// FixerAnnotationAnalyzer's pattern, applied inside comment tokens only. Exactly one space after the name: it
/// splits there, so any other gap is an annotation upstream rejects, left for `pun format` to report.
static IGNORE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?m)^(\h*[*\/]+\h+@)php-cs-fixer-ignore ((?:@?[\w\/,])+)").unwrap()
});

#[derive(Clone, Copy, PartialEq, Debug, clap::ValueEnum)]
pub enum Source {
    Pint,
    PhpCsFixer,
    Phpstan,
    Rector,
    Mago,
}

impl Source {
    const ALL: [Source; 5] = [
        Source::Pint,
        Source::PhpCsFixer,
        Source::Phpstan,
        Source::Rector,
        Source::Mago,
    ];

    fn name(self) -> &'static str {
        ["Pint", "PHP-CS-Fixer", "PHPStan", "Rector", "Mago"][self as usize]
    }

    /// Its config files, in the order the tool looks for them. Pint's is `--config` when given.
    fn files(self) -> &'static [&'static str] {
        match self {
            Source::Pint => &["pint.json"],
            Source::PhpCsFixer => &[".php-cs-fixer.php", ".php-cs-fixer.dist.php"],
            Source::Phpstan => &["phpstan.neon", "phpstan.neon.dist", "phpstan.dist.neon"],
            Source::Rector => &["rector.php"],
            Source::Mago => &["mago.toml"],
        }
    }

    /// Pint and PHP-CS-Fixer give `[format]`; the rest isn't migrated yet.
    fn supported(self) -> bool {
        matches!(self, Source::Pint | Source::PhpCsFixer)
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Action {
    Migrate,
    /// Selected, but Pun can't migrate this tool yet: left untouched.
    Unsupported,
    NotSelected,
    /// PHP-CS-Fixer selected together with Pint: pint.json gives `[format]`, as Pint itself ignores the other.
    Covered,
}

/// The detected configs (tool, file) and the chosen tools (a subset of them) → what happens to each, in detection order.
fn plan(detected: &[(Source, String)], chosen: &[Source]) -> Vec<(Source, String, Action)> {
    let pint = chosen.contains(&Source::Pint);
    (detected.iter())
        .map(|(s, f)| {
            let action = match s {
                _ if !chosen.contains(s) => Action::NotSelected,
                Source::PhpCsFixer if pint => Action::Covered,
                _ if s.supported() => Action::Migrate,
                _ => Action::Unsupported,
            };
            (*s, f.clone(), action)
        })
        .collect()
}

/// The configs found in `root`, one per tool: (tool, file). `pun init` reads it too.
pub(crate) fn detect(root: &Path, pint: &Path) -> Vec<(Source, String)> {
    let pint = pint.display().to_string();
    (Source::ALL.iter())
        .filter_map(|&s| {
            let files = if s == Source::Pint {
                &[pint.as_str()][..]
            } else {
                s.files()
            };
            let f = files.iter().find(|f| root.join(f).exists())?;
            Some((s, f.to_string()))
        })
        .collect()
}

/// The checklist: every detected config, all checked. Esc or q selects nothing.
fn ask(detected: &[(Source, String)]) -> Result<Vec<Source>> {
    let width = detected.iter().map(|d| d.1.len()).max().unwrap_or(0);
    let items: Vec<String> = (detected.iter())
        .map(|(s, f)| {
            let note = if s.supported() {
                ""
            } else {
                "  (not supported yet)"
            };
            format!("{:<13} {f:<width$}{note}", s.name())
                .trim_end()
                .to_string()
        })
        .collect();
    let picked = dialoguer::MultiSelect::new()
        .with_prompt("Migrate which configs? (space toggles, enter confirms)")
        .items(&items)
        .defaults(&vec![true; items.len()])
        .report(false)
        .interact_on_opt(&dialoguer::console::Term::stdout())?;
    Ok(picked
        .unwrap_or_default()
        .into_iter()
        .map(|i| detected[i].0)
        .collect())
}

pub fn run(a: MigrateArgs) -> Result<u8> {
    let root = root()?;
    let pint = a.config.as_deref().unwrap_or(Path::new("pint.json"));
    let detected = detect(&root, pint);
    check_from(&a.from, &detected, pint)?;
    let interactive = !(a.yes || !a.from.is_empty() || agent::enabled())
        && std::io::stdin().is_terminal()
        && std::io::stdout().is_terminal();
    let human = !agent::enabled();
    let nothing = format!(
        "no {}, .php-cs-fixer.php, phpstan.neon, rector.php or mago.toml found",
        pint.display()
    );
    let chosen = if !a.from.is_empty() {
        a.from.clone()
    } else if interactive && !detected.is_empty() {
        ask(&detected)?
    } else {
        detected.iter().map(|d| d.0).collect()
    };
    if human && !announce(&a, &detected, &nothing, interactive)? {
        println!("pun migrate: nothing written.");
        return Ok(0);
    }
    let rows = plan(&detected, &chosen);
    let (summary, sources) = summarize(&rows, a.dry_run, pint);
    let format_source = (rows.iter())
        .find(|r| r.2 == Action::Migrate)
        .map(|r| (r.0, root.join(&r.1)));
    let has_format = detected.iter().any(|d| d.0.supported());
    if (chosen.is_empty() && !detected.is_empty()) || (format_source.is_none() && has_format) {
        // Nothing to write: the user left pint.json / .php-cs-fixer.php out.
        let why = if chosen.is_empty() {
            "nothing selected"
        } else {
            "no Pint or PHP-CS-Fixer config selected"
        };
        if human {
            println!("pun migrate: {why}, pun.toml not written.");
            print!("{summary}");
        } else {
            let detail = [format!("{why}, pun.toml not written")];
            print!("{}", agent::render("ok", &detail, "", &sources, false));
        }
        return Ok(0);
    }
    let mut notes = Notes {
        human,
        agent: sources,
    };
    let from = format_source.as_ref().map(|s| s.1.clone());
    let toml = convert(
        &root,
        &a,
        pint,
        format_source,
        &nothing,
        detected.is_empty(),
        &mut notes,
    )?;
    let path = root.join("pun.toml");
    let old = std::fs::read_to_string(&path).ok();
    if old.as_ref().is_some_and(|o| *o != toml) && !a.dry_run {
        bail!(
            "pun.toml exists and differs from what migrate writes; move it away first (`pun migrate --dry-run` shows the difference)"
        );
    }
    let cfg = pun_config::from_toml(&root, &toml)?;
    let mut edits = ignore_rewrites(&root, &cfg, &mut notes);
    let rewritten = edits.len();
    let written = old.as_deref() != Some(&toml);
    if written {
        let old = old.unwrap_or_default().into_bytes();
        edits.insert(0, (path, old, toml.into_bytes()));
    }
    let diffs = apply(&root, &edits, a.dry_run)?;
    let (toml, from) = match &from {
        Some(f) => ("pun.toml", format!("from {}", rel(&root, f))),
        None if detected.is_empty() && !human => ("starter pun.toml", nothing),
        None => ("starter pun.toml", String::new()),
    };
    let report = Report {
        toml,
        state: match (written, a.dry_run) {
            (false, _) => "already up to date",
            (true, true) => "to write",
            (true, false) => "written",
        },
        from,
        ignores: if a.dry_run { "to rewrite" } else { "rewritten" },
        rewritten,
        diffs,
        summary,
        dry_run: a.dry_run,
        cfg,
    };
    report.print(notes);
    Ok(0)
}

/// `--from` names only detected configs.
fn check_from(from: &[Source], detected: &[(Source, String)], pint: &Path) -> Result<()> {
    for s in from {
        if !detected.iter().any(|d| d.0 == *s) {
            let files = if *s == Source::Pint {
                vec![pint.display().to_string()]
            } else {
                s.files().iter().map(|f| f.to_string()).collect()
            };
            bail!(
                "--from {}: no {} found",
                s.name().to_lowercase(),
                files.join(" or ")
            );
        }
    }
    Ok(())
}

/// The human run's opening line: no config (a terminal is asked whether to write the starter; `false` when it says
/// no), or that everything detected is migrated when nothing was picked.
fn announce(
    a: &MigrateArgs,
    detected: &[(Source, String)],
    nothing: &str,
    interactive: bool,
) -> Result<bool> {
    if detected.is_empty() {
        println!("pun migrate: {nothing}.");
        let confirm = || {
            dialoguer::Confirm::new()
                .with_prompt("Write a starter pun.toml?")
                .default(true)
                .interact_on_opt(&dialoguer::console::Term::stdout())
        };
        if interactive && confirm()? != Some(true) {
            return Ok(false);
        }
    } else if a.from.is_empty() && !interactive {
        let files: Vec<&str> = detected.iter().map(|d| d.1.as_str()).collect();
        println!(
            "pun migrate: migrating everything detected (pick with --from): {}",
            files.join(", ")
        );
    }
    Ok(true)
}

/// What happens to each detected config: the human table (one line per config) and agent mode's findings.
fn summarize(
    rows: &[(Source, String, Action)],
    dry_run: bool,
    pint: &Path,
) -> (String, Vec<String>) {
    let width = rows.iter().map(|r| r.1.len()).max().unwrap_or(0);
    let mut summary = String::new();
    let mut sources = vec![];
    for (s, f, action) in rows {
        let (kind, text) = match action {
            Action::Migrate if dry_run => ("migrate", "to migrate".to_string()),
            Action::Migrate => ("migrated", "migrated".into()),
            Action::Unsupported => (
                "not-supported",
                format!(
                    "skipped: {} migration is not supported yet; {f} left untouched",
                    s.name()
                ),
            ),
            Action::NotSelected => ("not-selected", "not selected".into()),
            Action::Covered => (
                "covered",
                format!(
                    "skipped: {} gives [format]; {f} left untouched",
                    pint.display()
                ),
            ),
        };
        writeln!(summary, "  {f:<width$}  {text}").unwrap();
        sources.push(match action {
            Action::Migrate | Action::NotSelected => format!("{f} {kind}"),
            _ => format!("{f} {kind} {text}"),
        });
    }
    (summary, sources)
}

/// pun.toml's text from the config that gives `[format]` (`source`), or from Pint's defaults (its laravel preset, or
/// `--preset`) when there is none. Pun runs a rule only with a native fixer; the rules it skips become notes.
fn convert(
    root: &Path,
    a: &MigrateArgs,
    pint: &Path,
    source: Option<(Source, PathBuf)>,
    nothing: &str,
    none_detected: bool,
    notes: &mut Notes,
) -> Result<String> {
    let fixers: std::collections::HashSet<&str> =
        pun_fixers::all().iter().map(|f| f.name()).collect();
    let runs = |name: &str| fixers.contains(name);
    let m = match &source {
        Some((Source::PhpCsFixer, f)) => {
            let (rules, finder) = php_cs_fixer_config(root, f)?;
            migrate::from_rules(rules, &finder, &runs)?
        }
        Some((_, f)) => migrate::from_pint(f, a.preset.as_deref(), &runs)?,
        None => migrate::from_pint(&root.join(pint), a.preset.as_deref(), &runs)?,
    };
    let from = match &source {
        Some((_, f)) => rel(root, f),
        None => format!("preset {}", a.preset.as_deref().unwrap_or("laravel")),
    };
    for u in &m.unsupported {
        notes.add(
            format!("{from}: {u}; skipped"),
            format!("{from} unsupported {u}"),
        );
    }
    let header = match &source {
        Some(_) => format!(" from {from}."),
        None if none_detected => format!(": {nothing}; starter config."),
        None => format!(
            ": no {} or .php-cs-fixer.php found; starter config.",
            pint.display()
        ),
    };
    Ok(format!("# Written by `pun migrate`{header}\n{}", m.toml))
}

/// Each project file whose `@php-cs-fixer-ignore` comments [`rewrite`] changes: (path, old, new). An annotation
/// naming a fixer Pun has no rule for becomes a note.
fn ignore_rewrites(
    root: &Path,
    cfg: &pun_config::Config,
    notes: &mut Notes,
) -> Vec<(PathBuf, Vec<u8>, Vec<u8>)> {
    let mut edits = vec![];
    let mut unknown = vec![];
    // A broken finder (a missing `in` dir) is `pun format`'s to report, as Pint does.
    for f in pun_workspace::discover(cfg, &[]).unwrap_or_default() {
        // Unreadable files are `pun format`'s to report.
        let Ok(code) = std::fs::read(&f) else {
            continue;
        };
        if let Some(new) = rewrite(&code, &rel(root, &f), &mut unknown) {
            edits.push((f, code, new));
        }
    }
    for (path, rules) in &unknown {
        let note = format!("`@php-cs-fixer-ignore {rules}` left as is (no Pun rule for it)");
        notes.add(
            format!("{path}: {note}"),
            format!("{path} ignore-not-migrated {note}"),
        );
    }
    edits
}

/// Writes the edits, or with `--dry-run` returns their diffs instead.
fn apply(root: &Path, edits: &[(PathBuf, Vec<u8>, Vec<u8>)], dry_run: bool) -> Result<String> {
    let mut diffs = String::new();
    for (f, old, new) in edits {
        if dry_run {
            let (old, new) = (String::from_utf8_lossy(old), String::from_utf8_lossy(new));
            diffs += &report::diff(&old, &new, Path::new(&rel(root, f)));
        } else {
            std::fs::write(f, new).with_context(|| format!("can't write {}", f.display()))?;
        }
    }
    Ok(diffs)
}

/// Notes on what migrate leaves out: humans get each on stderr as it is found; agent mode lists them after the
/// configs, under its summary line.
struct Notes {
    human: bool,
    /// Agent mode's findings: one line per config, then the notes.
    agent: Vec<String>,
}

impl Notes {
    fn add(&mut self, human: String, agent: String) {
        if self.human {
            eprintln!("pun migrate: {human}");
        } else {
            self.agent.push(agent);
        }
    }
}

/// A migration that wrote (or, with `--dry-run`, would write) pun.toml.
struct Report {
    /// `pun.toml`, or `starter pun.toml` without a Pint or PHP-CS-Fixer config.
    toml: &'static str,
    /// `written`, `to write` or `already up to date`.
    state: &'static str,
    /// `from <config>`, agent mode's "nothing found" for a starter, or empty.
    from: String,
    ignores: &'static str,
    /// Files whose ignore comments were (or would be) rewritten.
    rewritten: usize,
    /// `--dry-run`'s diffs.
    diffs: String,
    /// The human table of detected configs.
    summary: String,
    dry_run: bool,
    cfg: pun_config::Config,
}

impl Report {
    fn print(self, notes: Notes) {
        let Report {
            toml,
            state,
            from,
            ignores,
            rewritten,
            diffs,
            ..
        } = &self;
        if notes.human {
            print!("{diffs}");
            let sep = if from.is_empty() { "" } else { " " };
            println!(
                "pun migrate: {toml} {state}{sep}{from}, ignore comments {ignores} in {rewritten} file(s)."
            );
            print!("{}", self.summary);
        } else {
            let cfg = &self.cfg;
            let detail = [
                format!(
                    "{toml} {} (preset {}, {}, {})",
                    state.trim_start_matches("already "),
                    cfg.preset,
                    agent::count(cfg.rules.len(), "format rule"),
                    agent::count(cfg.lint.rules.len(), "lint rule")
                ),
                format!(
                    "ignore comments {ignores} in {}",
                    agent::count(*rewritten, "file")
                ),
            ];
            let result = if self.dry_run { "dry-run" } else { "ok" };
            print!(
                "{}",
                agent::render(result, &detail, from, &notes.agent, false)
            );
            print!("{diffs}");
        }
    }
}

/// `code` with each `@php-cs-fixer-ignore a,b` in a comment turned into `@pun-ignore-file format/x/a,lint/y/b`, or
/// `None` when nothing changes. One naming a fixer Pun has no rule for stays as it is, reported in `unknown`.
fn rewrite(code: &[u8], path: &str, unknown: &mut Vec<(String, String)>) -> Option<Vec<u8>> {
    if !code.windows(19).any(|w| w == b"php-cs-fixer-ignore") {
        return None;
    }
    let mut out = Vec::with_capacity(code.len());
    for t in pun_tokens::Tokens::from_code(code).as_slice() {
        let mut last = 0;
        for c in IGNORE
            .captures_iter(&t.content)
            .flatten()
            .filter(|_| t.is_comment())
        {
            let ids: Option<Vec<&str>> = (c[2].split(|&b| b == b','))
                .map(|r| std::str::from_utf8(r).ok().and_then(pun_config::rule_of))
                .collect();
            let Some(ids) = ids else {
                let rules = String::from_utf8_lossy(&c[2]).into_owned();
                unknown.push((path.to_string(), rules));
                continue;
            };
            let m = c.get(0).unwrap();
            out.extend_from_slice(&t.content[last..m.start()]);
            out.extend_from_slice(&c[1]);
            out.extend_from_slice(b"pun-ignore-file ");
            out.extend_from_slice(ids.join(",").as_bytes());
            last = m.end();
        }
        out.extend_from_slice(&t.content[last..]);
    }
    (out != code).then_some(out)
}

fn rel(root: &Path, p: &Path) -> String {
    p.strip_prefix(root).unwrap_or(p).display().to_string()
}

type Object = serde_json::Map<String, serde_json::Value>;

/// A `.php-cs-fixer.php`'s rules and Finder, read through the project's own PHP-CS-Fixer (the config is PHP code and
/// may register custom fixers). Migrate is the only place Pun runs PHP, and only to read the old config.
fn php_cs_fixer_config(root: &Path, config: &Path) -> Result<(Object, Object)> {
    if !root.join("vendor/friendsofphp/php-cs-fixer").is_dir() {
        bail!(
            "no vendor/friendsofphp/php-cs-fixer: run `composer install` so Pun can read the config"
        );
    }
    let out = std::process::Command::new("php")
        // Startup warnings (a broken php.ini) go to stderr, off the JSON.
        .args([
            "-d",
            "display_errors=stderr",
            "-r",
            include_str!("php_cs_fixer_rules.php"),
            "--",
        ])
        .arg(root.join("vendor/autoload.php"))
        .arg(config)
        .current_dir(root)
        .output()
        .context("reading .php-cs-fixer.php needs `php` on PATH")?;
    if !out.status.success() {
        // A caught exception's message is on stdout; a fatal error (uncatchable) only on stderr.
        if out.stdout.is_empty() {
            bail!("{}", String::from_utf8_lossy(&out.stderr).trim());
        }
        bail!("{}", String::from_utf8_lossy(&out.stdout));
    }
    let mut v: Object = serde_json::from_slice(&out.stdout)?;
    let mut take = |k: &str| serde_json::from_value(v.remove(k).unwrap_or_default());
    Ok((take("rules")?, take("finder")?))
}

#[cfg(test)]
mod tests {
    use super::{Action::*, Source::*, *};

    /// The checklist's answer becomes the plan: hand-derived from the rules in `plan`'s match.
    #[test]
    fn plan_follows_the_selection() {
        let all: Vec<(Source, String)> = Source::ALL
            .iter()
            .map(|&s| (s, s.files()[0].into()))
            .collect();
        let actions = |chosen: &[Source]| -> Vec<Action> {
            plan(&all, chosen).into_iter().map(|r| r.2).collect()
        };
        // Everything checked: pint.json wins [format], the rest can't be migrated yet.
        assert_eq!(
            actions(&Source::ALL),
            [Migrate, Covered, Unsupported, Unsupported, Unsupported]
        );
        // Pint unchecked: PHP-CS-Fixer gives [format].
        assert_eq!(
            actions(&[PhpCsFixer, Rector]),
            [NotSelected, Migrate, NotSelected, Unsupported, NotSelected]
        );
        assert_eq!(actions(&[]), [NotSelected; 5]);
        let rows = plan(&all[2..3], &[Phpstan]);
        assert_eq!(rows, [(Phpstan, "phpstan.neon".to_string(), Unsupported)]);
    }
}
