//! `pun migrate`, format part: pint.json (or, without one, `.php-cs-fixer(.dist).php`) becomes pun.toml, and
//! `@php-cs-fixer-ignore` comments become `@pun-ignore-file` with Pun rule IDs (docs/RULES.md §7).
use crate::{report, root};
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
    let (from, m) = match php_cs_fixer {
        Some(f) => {
            let d = pun_delegate::Delegate::start(
                &root,
                pun_delegate::Source::Config(f.clone()),
                None,
            )?;
            eprintln!(
                "pun migrate: {} configures its own Finder, which is not migrated: check [format] paths and exclude.",
                rel(&root, &f)
            );
            (f, migrate::from_rules(d.rules()?)?)
        }
        None => (
            pint.clone(),
            migrate::from_pint(&pint, a.preset.as_deref())?,
        ),
    };
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
    let toml = format!(
        "# Written by `pun migrate` from {}.\n{}",
        rel(&root, &from),
        m.toml
    );
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
    for u in &unknown {
        eprintln!("pun migrate: {u}");
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
    for (f, old, new) in &edits {
        if a.dry_run {
            let (old, new) = (String::from_utf8_lossy(old), String::from_utf8_lossy(new));
            print!("{}", report::diff(&old, &new, Path::new(&rel(&root, f))));
        } else {
            std::fs::write(f, new)?;
        }
    }
    if !a.dry_run {
        println!(
            "pun migrate: pun.toml {} from {}, ignore comments rewritten in {rewritten} file(s).",
            if edits.len() > rewritten {
                "written"
            } else {
                "already up to date"
            },
            rel(&root, &from)
        );
    }
    Ok(0)
}

/// `code` with each `@php-cs-fixer-ignore a,b` in a comment turned into `@pun-ignore-file format.a,lint.b`, or
/// `None` when nothing changes. One naming a fixer Pun has no rule for stays as it is, reported in `unknown`.
fn rewrite(code: &[u8], path: &str, unknown: &mut Vec<String>) -> Option<Vec<u8>> {
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
                let rules = String::from_utf8_lossy(&c[2]);
                unknown.push(format!(
                    "{path}: `@php-cs-fixer-ignore {rules}` left as is (no Pun rule for it)"
                ));
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
