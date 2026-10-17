//! `pun migrate`, format part: pint.json (or, without one, `.php-cs-fixer(.dist).php`) becomes pun.toml, and
//! `@php-cs-fixer-ignore` comments become `@pun-ignore-file` with Pun rule IDs (context/RULES.md §7).
use crate::{agent, report, root};
use anyhow::{Result, bail};
use pcre2::bytes::Regex;
use pun_config::migrate;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

#[derive(clap::Args)]
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
}

/// FixerAnnotationAnalyzer's pattern, applied inside comment tokens only. Exactly one space after the name: it
/// splits there, so any other gap is an annotation upstream rejects, left for `pun format` to report.
static IGNORE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?m)^(\h*[*\/]+\h+@)php-cs-fixer-ignore ((?:@?[\w\/,])+)").unwrap()
});

pub fn run(a: MigrateArgs) -> Result<u8> {
    let root = root()?;
    let pint = root.join(a.config.as_deref().unwrap_or(Path::new("pint.json")));
    let php_cs_fixer = [".php-cs-fixer.php", ".php-cs-fixer.dist.php"]
        .map(|f| root.join(f))
        .into_iter()
        .find(|f| f.exists() && !pint.exists() && a.config.is_none());
    // Agent mode: notes become findings under one summary line, printed at the end.
    let mut notes = vec![];
    let (from, m) = match php_cs_fixer {
        Some(f) => {
            let d = pun_delegate::Delegate::start(
                &root,
                pun_delegate::Source::Config(f.clone()),
                None,
            )?;
            let note = "configures its own Finder, which is not migrated: check [format] paths and exclude.";
            if agent::enabled() {
                notes.push(format!("{} finder-not-migrated {note}", rel(&root, &f)));
            } else {
                eprintln!("pun migrate: {} {note}", rel(&root, &f));
            }
            (f, migrate::from_rules(d.rules()?)?)
        }
        None => (
            pint.clone(),
            migrate::from_pint(&pint, a.preset.as_deref())?,
        ),
    };
    if !m.unsupported.is_empty() && agent::enabled() {
        let from = rel(&root, &from);
        notes.extend(
            m.unsupported
                .iter()
                .map(|u| format!("{from} unsupported-rule {u}")),
        );
        let detail = [
            agent::count(m.unsupported.len(), "unsupported rule"),
            "nothing written".into(),
        ];
        print!(
            "{}",
            agent::render("fail", &detail, &format!("from {from}"), &notes, false)
        );
        return Ok(1);
    }
    if !m.unsupported.is_empty() {
        eprintln!(
            "pun migrate: {} uses rules Pun can't run, so nothing was written:",
            rel(&root, &from)
        );
        for u in &m.unsupported {
            eprintln!("  {u}");
        }
        return Ok(1);
    }
    // No config to migrate: the defaults, said as such (not "from pint.json") in pun.toml and the output.
    let missing = (!from.exists()).then(|| {
        let or = if a.config.is_none() {
            " or .php-cs-fixer.php"
        } else {
            ""
        };
        let preset = a.preset.as_deref().unwrap_or("laravel");
        format!(
            "no {}{or} found; wrote pun.toml with the {preset} preset",
            rel(&root, &from)
        )
    });
    let toml = match &missing {
        Some(why) => format!("# Written by `pun migrate`: {why}.\n{}", m.toml),
        None => format!(
            "# Written by `pun migrate` from {}.\n{}",
            rel(&root, &from),
            m.toml
        ),
    };
    let path = root.join("pun.toml");
    let old = std::fs::read_to_string(&path).ok();
    if old.as_ref().is_some_and(|o| *o != toml) && !a.dry_run {
        bail!(
            "pun.toml exists and differs from what migrate writes; move it away first (`pun migrate --dry-run` shows the difference)"
        );
    }

    let cfg = pun_config::from_toml(&root, &toml)?;
    let mut edits = vec![];
    let mut unknown = vec![];
    // A broken finder (a missing `in` dir) is `pun format`'s to report, as Pint does.
    for f in pun_workspace::discover(&cfg, &[]).unwrap_or_default() {
        // Unreadable files are `pun format`'s to report.
        let Ok(code) = std::fs::read(&f) else {
            continue;
        };
        if let Some(new) = rewrite(&code, &rel(&root, &f), &mut unknown) {
            edits.push((f, code, new));
        }
    }
    for (path, rules) in &unknown {
        let note = format!("`@php-cs-fixer-ignore {rules}` left as is (no Pun rule for it)");
        if agent::enabled() {
            notes.push(format!("{path} ignore-not-migrated {note}"));
        } else {
            eprintln!("pun migrate: {path}: {note}");
        }
    }
    let rewritten = edits.len();
    if old.as_deref() != Some(&toml) {
        edits.insert(
            0,
            (
                path,
                old.unwrap_or_default().into_bytes(),
                toml.into_bytes(),
            ),
        );
    }
    let mut diffs = String::new();
    for (f, old, new) in &edits {
        if a.dry_run {
            let (old, new) = (String::from_utf8_lossy(old), String::from_utf8_lossy(new));
            diffs += &report::diff(&old, &new, Path::new(&rel(&root, f)));
        } else {
            std::fs::write(f, new)?;
        }
    }
    if agent::enabled() {
        let toml = match (edits.len() > rewritten, a.dry_run, &missing) {
            (false, _, _) => "pun.toml up to date",
            (true, true, _) => "pun.toml to write",
            (true, false, Some(why)) => why,
            (true, false, None) => "pun.toml written",
        };
        let detail = [
            format!(
                "{toml} (preset {}, {}, {})",
                cfg.preset,
                agent::count(cfg.rules.len(), "format rule"),
                agent::count(cfg.lint.len(), "lint rule")
            ),
            format!(
                "ignore comments {} in {}",
                if a.dry_run { "to rewrite" } else { "rewritten" },
                agent::count(rewritten, "file")
            ),
        ];
        let result = if a.dry_run { "dry-run" } else { "ok" };
        let from = match missing {
            Some(_) => String::new(),
            None => format!("from {}", rel(&root, &from)),
        };
        print!("{}", agent::render(result, &detail, &from, &notes, false));
        print!("{diffs}");
    } else if a.dry_run {
        print!("{diffs}");
    } else {
        let toml = match (edits.len() > rewritten, missing) {
            (true, Some(why)) => why,
            (false, Some(why)) => format!(
                "{}; pun.toml already up to date",
                why.split(';').next().unwrap()
            ),
            (true, None) => format!("pun.toml written from {}", rel(&root, &from)),
            (false, None) => format!("pun.toml already up to date from {}", rel(&root, &from)),
        };
        println!("pun migrate: {toml}, ignore comments rewritten in {rewritten} file(s).");
    }
    Ok(0)
}

/// `code` with each `@php-cs-fixer-ignore a,b` in a comment turned into `@pun-ignore-file format.a,lint.b`, or
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
