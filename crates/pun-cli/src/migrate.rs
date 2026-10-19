//! `pun migrate`: finds the Pint, PHP-CS-Fixer, PHPStan, Rector and Mago configs, asks which to migrate (a checklist
//! on a terminal; `--from`/`--yes`, agent mode or no terminal don't ask), and migrates the chosen ones it can. Today
//! that is the format part: pint.json or `.php-cs-fixer(.dist).php` becomes pun.toml, and `@php-cs-fixer-ignore`
//! comments become `@pun-ignore-file` with Pun rule IDs (context/plan/rules.md §4). PHPStan, Rector and Mago are listed and
//! left untouched. With no config at all it writes a starter pun.toml.
use crate::render::{ColorChoice, Look, Role};
use crate::{agent, report, root};
use anyhow::{Context, Result, bail};
use pcre2::bytes::Regex;
use pun_config::migrate;
use std::fmt::Write as _;
use std::io::IsTerminal;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

#[derive(clap::Args, Default)]
pub(crate) struct MigrateArgs {
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
pub(crate) enum Source {
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

    /// Pint and PHP-CS-Fixer give `[format]`, Rector `[lint]`; the rest isn't migrated yet.
    fn supported(self) -> bool {
        matches!(
            self,
            Source::Pint | Source::PhpCsFixer | Source::Rector | Source::Phpstan
        )
    }

    /// Gives `[format]`.
    fn format(self) -> bool {
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
    /// rector.php selected without `vendor/rector/rector`: Rector's config is PHP that only Rector can read.
    NoVendor,
    /// Selected, but pun.toml is not written: the Pint or PHP-CS-Fixer config was left out.
    Held,
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
    let picked = dialoguer::MultiSelect::with_theme(&crate::init::theme())
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

pub(crate) fn run(a: MigrateArgs) -> Result<u8> {
    let root = root()?;
    let pint = a.config.as_deref().unwrap_or(Path::new("pint.json"));
    let detected = detect(&root, pint);
    check_from(&a.from, &detected, pint)?;
    let interactive = !(a.yes || !a.from.is_empty() || agent::enabled())
        && std::io::stdin().is_terminal()
        && std::io::stdout().is_terminal();
    let mut report = Report::new(!agent::enabled(), a.dry_run);
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
    if detected.is_empty() && report.human {
        println!("N{}.", &nothing[1..]);
        if interactive && !starter()? {
            report.finish(Mark::Skip, "Nothing written".into(), vec![], "");
            return Ok(0);
        }
    }
    let rector_installed = root.join("vendor/rector/rector").is_dir();
    let rows: Vec<_> = (plan(&detected, &chosen).into_iter())
        .map(|(s, f, a)| match (s, a) {
            (Source::Rector, Action::Migrate) if !rector_installed => (s, f, Action::NoVendor),
            _ => (s, f, a),
        })
        .collect();
    let format_source = (rows.iter())
        .find(|r| r.2 == Action::Migrate && r.0.format())
        .map(|r| (r.0, root.join(&r.1)));
    let rector = (rows.iter())
        .find(|r| r.2 == Action::Migrate && r.0 == Source::Rector)
        .map(|r| root.join(&r.1));
    let phpstan = (rows.iter())
        .find(|r| r.2 == Action::Migrate && r.0 == Source::Phpstan)
        .map(|r| root.join(&r.1));
    let has_format = detected.iter().any(|d| d.0.format());
    if (chosen.is_empty() && !detected.is_empty()) || (format_source.is_none() && has_format) {
        // Nothing to write: the user left pint.json / .php-cs-fixer.php out.
        let why = if chosen.is_empty() {
            "nothing selected"
        } else {
            "no Pint or PHP-CS-Fixer config selected"
        };
        let rows: Vec<_> = (rows.into_iter())
            .map(|(s, f, a)| match a {
                Action::Migrate => (s, f, Action::Held),
                a => (s, f, a),
            })
            .collect();
        report.configs(&rows, pint, &["", "", ""]);
        let detail = vec![format!("{why}, pun.toml not written")];
        report.finish(
            Mark::Skip,
            format!("N{}; pun.toml not written", &why[1..]),
            detail,
            "",
        );
        return Ok(0);
    }
    let from = format_source.as_ref().map(|s| rel(&root, &s.1));
    let arrow = report.look.glyph("→", "->");
    let mut m = convert(&root, &a, pint, format_source, &mut report)?;
    let mut lint = String::new();
    if let Some(r) = &rector {
        lint = rector_lint(&root, r, &mut m.parts, &mut report)?;
        m.toml = m.parts.render();
    }
    let mut analyse = String::new();
    if let Some(p) = &phpstan {
        analyse = phpstan_analyse(&root, p, &mut m.parts, &mut report)?;
        m.toml = m.parts.render();
    }
    let header = match &from {
        Some(f) => format!(" from {f}."),
        None if detected.is_empty() => format!(": {nothing}; starter config."),
        None => format!(
            ": no {} or .php-cs-fixer.php found; starter config.",
            pint.display()
        ),
    };
    let toml = format!("# Written by `pun migrate`{header}\n{}", m.toml);
    let path = root.join("pun.toml");
    let old = std::fs::read_to_string(&path).ok();
    if old.as_ref().is_some_and(|o| *o != toml) && !a.dry_run {
        bail!(
            "pun.toml exists and differs from what migrate writes; move it away first (`pun migrate --dry-run` shows the difference)"
        );
    }
    let cfg = pun_config::from_toml(&root, &toml)?;
    let mut edits = ignore_rewrites(&root, &cfg, &mut report);
    let rewritten = edits.len();
    let written = old.as_deref() != Some(&toml);
    if written {
        let old = old.unwrap_or_default().into_bytes();
        edits.insert(0, (path, old, toml.into_bytes()));
    }
    report.diffs = apply(&root, &edits, a.dry_run)?;
    let mut moved = format!(
        "{arrow} [format] preset {}, {} changed",
        m.preset,
        agent::count(m.format_rules, "rule")
    );
    if m.lint_rules > 0 {
        let lint = agent::count(m.lint_rules, "rule");
        moved += &format!(", {lint} {arrow} [lint]");
    }
    let moved = [
        moved,
        format!("{arrow} {lint}"),
        format!("{arrow} {analyse}"),
    ];
    report.configs(&rows, pint, &moved.each_ref().map(String::as_str));
    if rewritten > 0 {
        let files = agent::count(rewritten, "file");
        report.line(
            Mark::Done,
            "ignore comments",
            format!("{arrow} @pun-ignore-file in {files}"),
        );
    }
    let toml = if from.is_some() {
        "pun.toml"
    } else {
        "starter pun.toml"
    };
    let state = match (written, a.dry_run) {
        (false, _) => "already up to date",
        (true, true) => "to write",
        (true, false) => "written",
    };
    let ignores = if a.dry_run { "to rewrite" } else { "rewritten" };
    let detail = vec![
        format!(
            "{toml} {} (preset {}, {}, {})",
            state.trim_start_matches("already "),
            cfg.preset,
            agent::count(cfg.rules.len(), "format rule"),
            agent::count(cfg.lint.rules.len(), "lint rule")
        ),
        format!(
            "ignore comments {ignores} in {}",
            agent::count(rewritten, "file")
        ),
    ];
    let starter = if from.is_some() {
        String::new()
    } else {
        format!(" (preset {})", cfg.preset)
    };
    let (mark, last) = match (written, a.dry_run) {
        (false, true) => (Mark::Skip, format!("Dry run: {toml} is already up to date")),
        (false, false) => (Mark::Done, format!("{toml} is already up to date")),
        (true, true) => (Mark::Skip, format!("Dry run: would write {toml}{starter}")),
        (true, false) => (Mark::Done, format!("Wrote {toml}{starter}")),
    };
    let checked = match &from {
        Some(f) => format!("from {f}"),
        None if detected.is_empty() => nothing,
        None => String::new(),
    };
    report.finish(mark, last, detail, &checked);
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

/// No config at all, on a terminal: whether to write the starter pun.toml.
fn starter() -> Result<bool> {
    let yes = dialoguer::Confirm::with_theme(&crate::init::theme())
        .with_prompt("Write a starter pun.toml?")
        .default(true)
        .interact_on_opt(&dialoguer::console::Term::stdout())?;
    Ok(yes == Some(true))
}

/// pun.toml's body from the config that gives `[format]` (`source`), or from Pint's defaults (its laravel preset, or
/// `--preset`) when there is none. Pun runs a rule only with a native fixer; the rules it skips are reported.
fn convert(
    root: &Path,
    a: &MigrateArgs,
    pint: &Path,
    source: Option<(Source, PathBuf)>,
    report: &mut Report,
) -> Result<migrate::Migration> {
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
        let (name, why) = u.split_once(": ").unwrap();
        report.unsupported(name, why, format!("{from} unsupported {u}"));
    }
    Ok(m)
}

/// Each project file whose `@php-cs-fixer-ignore` comments [`rewrite`] changes: (path, old, new). An annotation
/// naming a fixer Pun has no rule for is reported.
fn ignore_rewrites(
    root: &Path,
    cfg: &pun_config::Config,
    report: &mut Report,
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
        report.unsupported(path, &note, format!("{path} ignore-not-migrated {note}"));
    }
    edits
}

/// rector.php's `[lint]` added to `parts` (context/plan/migrate.md Build 4); the ✔ line's text. Rector's paths become
/// `[lint] exclude` entries for the files `pun lint` would otherwise check outside them.
fn rector_lint(
    root: &Path,
    config: &Path,
    parts: &mut migrate::Parts,
    report: &mut Report,
) -> Result<String> {
    let mut r = migrate::from_rector(root, &rector_config(root, config)?);
    let cfg = pun_config::from_toml(root, &parts.render())?;
    let files = pun_workspace::files(&cfg, Some(pun_workspace::Command::Lint), &[])?;
    r.exclude.extend(outside(root, &files, &r.paths));
    let done = migrate::compose(root, parts, &r)?;
    let mut skipped = r.unsupported;
    skipped.extend(done.unsupported);
    // One line per reason that several rules share (a whole upgrade set below Pun's floor), else one per rule.
    let mut reasons: Vec<(&str, usize)> = vec![];
    for (_, why) in &skipped {
        match reasons.iter_mut().find(|(w, _)| *w == why) {
            Some((_, n)) => *n += 1,
            None => reasons.push((why, 1)),
        }
    }
    let from = rel(root, config);
    for (why, n) in &reasons {
        let rules = skipped.iter().filter(|s| s.1 == *why);
        if *n > 3 {
            let finding = format!("{from} unsupported {n} Rector rules: {why}");
            report.unsupported(&format!("{n} Rector rules"), why, finding);
        } else {
            for (class, _) in rules {
                let finding = format!("{from} unsupported {class}: {why}");
                report.unsupported(class, why, finding);
            }
        }
    }
    let sets = match r.sets.is_empty() {
        true => "no sets".to_string(),
        false => format!("sets {}", r.sets.join(", ")),
    };
    let mut text = format!("[lint] {sets}; {} on, {} off", done.on, done.off);
    if done.waiting > 0 {
        text += &format!(" ({} more once Pun ports them)", done.waiting);
    }
    if !r.exclude.is_empty() {
        text += &format!(", {} excluded", agent::count(r.exclude.len(), "path"));
    }
    if !r.overrides.is_empty() {
        text += &format!(", {}", agent::count(r.overrides.len(), "per-path skip"));
    }
    Ok(text)
}

/// phpstan.neon's `[analyse]` added to `parts` (context/plan/migrate.md Build 5); the ✔ line's text. Baselines are
/// not copied: each gets the note to generate a Pun one.
fn phpstan_analyse(
    root: &Path,
    config: &Path,
    parts: &mut migrate::Parts,
    report: &mut Report,
) -> Result<String> {
    let p = migrate::phpstan::from_phpstan(root, config)?;
    let arrow = report.look.glyph("→", "->");
    let done = migrate::phpstan::compose(root, parts, &p)?;
    let from = rel(root, config);
    for (name, why) in p.unsupported.iter().chain(&done.unsupported) {
        report.unsupported(name, why, format!("{from} unsupported {name}: {why}"));
    }
    for b in &p.baselines {
        let text = "was not copied. Run `pun analyse --generate-baseline` for a Pun baseline.";
        report.notes.push((b.clone(), text.into()));
        (report.findings.1).push(format!(
            "{b} not-copied run `pun analyse --generate-baseline` for a Pun baseline"
        ));
    }
    let mut text = format!("[analyse] preset {} (level {})", done.preset, p.level);
    if done.on + done.off > 0 {
        text += &format!(", {} on, {} off", done.on, done.off);
    }
    if done.lint > 0 {
        text += &format!(", {} {arrow} [lint]", agent::count(done.lint, "rule"));
    }
    let ignores = p.ignores.len();
    if ignores > 0 {
        text += &format!(", {} ignored", agent::count(ignores, "error pattern"));
    }
    Ok(text)
}

/// The shallowest path (relative to `root`) holding each of `files` that no Rector path covers, once each; nothing
/// when Rector checks the whole project (`.`).
fn outside(root: &Path, files: &[PathBuf], paths: &[String]) -> Vec<String> {
    if paths.is_empty() || paths.iter().any(|p| p == ".") {
        return vec![];
    }
    let under = |a: &str, b: &str| a == b || a.starts_with(&format!("{b}/"));
    let mut out: Vec<String> = vec![];
    for f in files {
        let f = rel(root, f);
        if paths.iter().any(|p| under(&f, p)) {
            continue;
        }
        let parts: Vec<&str> = f.split('/').collect();
        // The first ancestor that holds no Rector path.
        let a = (1..=parts.len())
            .map(|k| parts[..k].join("/"))
            .find(|a| !paths.iter().any(|p| under(p, a)))
            .unwrap();
        if !out.contains(&a) {
            out.push(a);
        }
    }
    out
}

/// A rector.php as Rector resolves it, read through the project's own Rector (the config is PHP code). Like
/// [`php_cs_fixer_config`], migrate is the only place Pun runs PHP.
fn rector_config(root: &Path, config: &Path) -> Result<migrate::RectorConfig> {
    let out = std::process::Command::new("php")
        .args([
            "-d",
            "display_errors=stderr",
            "-r",
            include_str!("rector_config.php"),
            "--",
        ])
        .arg(root.join("vendor/autoload.php"))
        .arg(config)
        .current_dir(root)
        .output()
        .context("reading rector.php needs `php` on PATH")?;
    if !out.status.success() {
        if out.stdout.is_empty() {
            bail!("{}", String::from_utf8_lossy(&out.stderr).trim());
        }
        bail!("{}", String::from_utf8_lossy(&out.stdout));
    }
    Ok(serde_json::from_slice(&out.stdout)?)
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

/// Where unsupported items are asked for (context/plan/migrate.md §3).
const PR: &str = "https://github.com/punphp/pun";

/// A line's mark: ✔ moved or written, ⚠ named and skipped, · left alone.
#[derive(Clone, Copy, PartialEq, Debug)]
enum Mark {
    Done,
    Warn,
    Skip,
}

/// What migrate prints (context/plan/output.md §2): one line per detected config, a ⚠ line per item it skips with
/// the PR request under them, then the last line. Agent mode gets one summary line and a finding per config and item.
struct Report {
    human: bool,
    dry_run: bool,
    look: Look,
    /// One per detected config.
    configs: Vec<(Mark, String, String)>,
    /// What migrate couldn't move: (name, why).
    items: Vec<(String, String)>,
    /// Lines after the PR request: the baselines not copied, (file, text).
    notes: Vec<(String, String)>,
    /// Agent mode's findings: one per config, then one per item.
    findings: (Vec<String>, Vec<String>),
    /// `--dry-run`'s diffs, printed first.
    diffs: String,
}

impl Report {
    fn new(human: bool, dry_run: bool) -> Report {
        Report {
            human,
            dry_run,
            look: Look::stdout(ColorChoice::Auto),
            configs: vec![],
            items: vec![],
            notes: vec![],
            findings: (vec![], vec![]),
            diffs: String::new(),
        }
    }

    /// The line of each detected config; `moved` says what the format config and rector.php became.
    fn configs(&mut self, rows: &[(Source, String, Action)], pint: &Path, moved: &[&str; 3]) {
        for (s, f, action) in rows {
            let moved = match s {
                Source::Rector => moved[1],
                Source::Phpstan => moved[2],
                _ => moved[0],
            }
            .to_string();
            let (mark, kind, text) = match action {
                Action::Migrate if self.dry_run => (Mark::Done, "migrate", moved),
                Action::Migrate => (Mark::Done, "migrated", moved),
                Action::Unsupported => (
                    Mark::Warn,
                    "not-supported",
                    format!(
                        "{} config: not supported by Pun yet; left untouched",
                        s.name()
                    ),
                ),
                Action::NotSelected => (Mark::Skip, "not-selected", "not selected".into()),
                Action::Covered => (
                    Mark::Skip,
                    "covered",
                    format!(
                        "not migrated: {} gives [format], as in Pint",
                        pint.display()
                    ),
                ),
                Action::NoVendor => (
                    Mark::Warn,
                    "not-read",
                    "not read: run `composer install` so Pun can read it with Rector; left untouched".into(),
                ),
                Action::Held => (
                    Mark::Skip,
                    "not-migrated",
                    "not migrated: pun.toml needs the format config too".into(),
                ),
            };
            self.findings.0.push(match action {
                Action::Migrate | Action::NotSelected => format!("{f} {kind}"),
                _ => format!("{f} {kind} {text}"),
            });
            if *action == Action::Unsupported {
                self.items.push((f.clone(), text));
            } else {
                self.configs.push((mark, f.clone(), text));
            }
        }
    }

    /// A line under the configs (the rewritten ignore comments).
    fn line(&mut self, mark: Mark, name: &str, text: String) {
        self.configs.push((mark, name.into(), text));
    }

    /// Something migrate names and skips.
    fn unsupported(&mut self, name: &str, why: &str, finding: String) {
        self.items.push((name.into(), format!("{why}; skipped")));
        self.findings.1.push(finding);
    }

    /// Prints everything, ending with `last` (humans) or the summary line built from `detail` and `checked` (agents).
    fn finish(mut self, mark: Mark, last: String, detail: Vec<String>, checked: &str) {
        if !self.items.is_empty() {
            self.findings
                .1
                .push(format!("send a PR for what is skipped: {PR}"));
        }
        if !self.human {
            let result = if self.dry_run { "dry-run" } else { "ok" };
            let findings = [&self.findings.0[..], &self.findings.1[..]].concat();
            print!(
                "{}",
                agent::render(result, &detail, checked, &findings, false)
            );
            print!("{}", self.diffs);
            return;
        }
        let mut out = std::mem::take(&mut self.diffs);
        out += &self.block(&self.configs);
        let items: Vec<_> = (self.items.iter())
            .map(|(n, t)| (Mark::Warn, n.clone(), t.clone()))
            .collect();
        out += &self.block(&items);
        if !items.is_empty() {
            out += &format!("   Want them? Send a PR to {PR}\n");
        }
        for (name, text) in &self.notes {
            let glyph = self.look.paint(Role::Warning, self.look.glyph("⚠", "!"));
            writeln!(out, " {glyph} {name} {text}").unwrap();
        }
        out += &self.block(&[(mark, last, String::new())]);
        print!("{out}");
    }

    /// Lines with their texts in one column, two spaces after the longest name (up to 36 characters).
    fn block(&self, lines: &[(Mark, String, String)]) -> String {
        let width = (lines.iter())
            .filter(|l| !l.2.is_empty())
            .map(|l| l.1.chars().count())
            .filter(|w| *w <= 36)
            .max()
            .unwrap_or(0);
        let mut out = String::new();
        for (mark, name, text) in lines {
            let (glyph, role) = match mark {
                Mark::Done => (self.look.glyph("✔", "ok"), Role::Ok),
                Mark::Warn => (self.look.glyph("⚠", "!"), Role::Warning),
                Mark::Skip => (self.look.glyph("·", "-"), Role::Dim),
            };
            let glyph = self.look.paint(role, glyph);
            let pad = width.saturating_sub(name.chars().count());
            let text = match mark {
                Mark::Skip => self.look.paint(Role::Dim, text),
                _ => text.clone(),
            };
            let line = format!(" {glyph} {name}{:pad$}  {text}", "");
            writeln!(out, "{}", line.trim_end()).unwrap();
        }
        out
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
        // Everything checked: pint.json wins [format], Mago can't be migrated yet.
        assert_eq!(
            actions(&Source::ALL),
            [Migrate, Covered, Migrate, Migrate, Unsupported]
        );
        // Pint unchecked: PHP-CS-Fixer gives [format].
        assert_eq!(
            actions(&[PhpCsFixer, Mago]),
            [NotSelected, Migrate, NotSelected, NotSelected, Unsupported]
        );
        assert_eq!(actions(&[]), [NotSelected; 5]);
    }
}
