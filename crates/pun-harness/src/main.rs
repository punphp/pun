//! Differential harness: run the pinned Pint oracle and `pun format` on copies of each
//! corpus project and byte-diff the results. See docs/oracle.md.

use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};
use serde::Deserialize;
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Run oracle (pint) and `pun format` on each corpus project and diff the output.
    Format {
        /// Only these corpus projects (repeatable). Default: all in corpus/manifest.toml.
        #[arg(long)]
        corpus: Vec<String>,
        /// Enable only this fixer (options taken from the project's effective config).
        #[arg(long)]
        fixer: Option<String>,
        /// Number of unified diffs to print per project.
        #[arg(long, default_value_t = 3)]
        diffs: usize,
    },
}

#[derive(Deserialize)]
struct Manifest {
    project: Vec<Project>,
}

#[derive(Deserialize)]
struct Project {
    name: String,
}

struct Outcome {
    name: String,
    files: usize,
    identical: usize,
    differing: Vec<String>,
    oracle_changed: usize,
    pun_changed: usize,
}

fn main() -> Result<()> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..").canonicalize()?;
    let Cmd::Format { corpus, fixer, diffs } = Cli::parse().cmd;
    let manifest: Manifest = toml::from_str(&fs::read_to_string(root.join("corpus/manifest.toml"))?)?;
    let pint = root.join(".oracle/pint/builds/pint");
    if !pint.exists() {
        bail!("{} missing: run scripts/oracle.sh", pint.display());
    }
    let pun = root.join("target/debug/pun");
    if !pun.exists() {
        eprintln!("note: {} missing, treating pun as identity", pun.display());
    }

    let tmp = std::env::temp_dir().join(format!("pun-harness-{}", std::process::id()));
    let mut outcomes = vec![];
    for p in manifest.project.iter().filter(|p| corpus.is_empty() || corpus.contains(&p.name)) {
        let src = root.join("corpus").join(&p.name);
        if !src.join(".git").exists() {
            bail!("{} missing: run scripts/corpus.sh", src.display());
        }
        let (oracle_dir, pun_dir) = (tmp.join(&p.name).join("oracle"), tmp.join(&p.name).join("pun"));
        copy_tree(&src, &oracle_dir)?;
        copy_tree(&src, &pun_dir)?;

        let mut args: Vec<String> = vec![];
        if let Some(name) = &fixer {
            let cfg = tmp.join(&p.name).join("single-fixer.json");
            fs::write(&cfg, single_fixer_config(&root, &src, name)?)?;
            args = vec!["--config".into(), cfg.display().to_string()];
        }
        run(Command::new("php").arg(&pint).args(&args).current_dir(&oracle_dir), "pint")?;
        if pun.exists() {
            run(Command::new(&pun).arg("format").args(&args).current_dir(&pun_dir), "pun")?;
        }

        let o = compare(&p.name, &src, &oracle_dir, &pun_dir)?;
        println!(
            "{}: {} files, {} identical, {} differing (pint changed {}, pun changed {})",
            o.name, o.files, o.identical, o.differing.len(), o.oracle_changed, o.pun_changed
        );
        for rel in o.differing.iter().take(diffs) {
            let a = String::from_utf8_lossy(&fs::read(oracle_dir.join(rel)).unwrap_or_default()).into_owned();
            let b = String::from_utf8_lossy(&fs::read(pun_dir.join(rel)).unwrap_or_default()).into_owned();
            print!(
                "{}",
                similar::TextDiff::from_lines(&a, &b)
                    .unified_diff()
                    .header(&format!("oracle/{rel}"), &format!("pun/{rel}"))
            );
        }
        outcomes.push(o);
        fs::remove_dir_all(tmp.join(&p.name)).ok();
    }
    fs::remove_dir_all(&tmp).ok();

    if fixer.is_none() {
        write_scoreboard(&root.join("docs/scoreboard.md"), &outcomes)?;
    }
    if outcomes.iter().any(|o| !o.differing.is_empty()) {
        std::process::exit(1);
    }
    Ok(())
}

fn run(cmd: &mut Command, what: &str) -> Result<()> {
    let out = cmd.output().with_context(|| format!("spawning {what}"))?;
    if !out.status.success() {
        eprintln!(
            "warning: {what} exited {}\n{}{}",
            out.status,
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );
    }
    Ok(())
}

/// pint.json with the project's finder options, preset "empty", and one rule configured as
/// in the project's effective rule list (or `true` if the project doesn't enable it).
fn single_fixer_config(root: &Path, src: &Path, name: &str) -> Result<String> {
    let pint_json = src.join("pint.json");
    let mut cmd = Command::new("php");
    cmd.arg(root.join("scripts/effective-rules.php"));
    let mut cfg = serde_json::Map::new();
    if pint_json.exists() {
        cmd.arg(&pint_json);
        let orig: serde_json::Map<String, serde_json::Value> = serde_json::from_slice(&fs::read(&pint_json)?)?;
        for key in ["exclude", "notPath", "notName", "in"] {
            if let Some(v) = orig.get(key) {
                cfg.insert(key.into(), v.clone());
            }
        }
    }
    let out = cmd.output()?;
    let eff: serde_json::Value = serde_json::from_slice(&out.stdout).context("effective-rules.php output")?;
    let options = eff["fixers"]
        .as_array()
        .and_then(|fs| fs.iter().find(|f| f["name"] == name))
        .map(|f| f["options"].clone())
        .filter(|o| !o.is_null())
        .unwrap_or(serde_json::Value::Bool(true));
    cfg.insert("preset".into(), "empty".into());
    cfg.insert("rules".into(), serde_json::json!({ name: options }));
    Ok(serde_json::to_string_pretty(&cfg)?)
}

fn copy_tree(src: &Path, dst: &Path) -> Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let (from, to) = (entry.path(), dst.join(entry.file_name()));
        let name = entry.file_name();
        if name == ".git" {
            continue;
        }
        let ty = entry.file_type()?;
        if name == "node_modules" {
            // Needed by Pint/laravel_blade (prettier); excluded from formatting and diffing.
            std::os::unix::fs::symlink(&from, &to)?;
        } else if ty.is_dir() {
            copy_tree(&from, &to)?;
        } else if ty.is_symlink() {
            std::os::unix::fs::symlink(fs::read_link(&from)?, &to)?;
        } else {
            fs::copy(&from, &to)?;
        }
    }
    Ok(())
}

fn list_files(dir: &Path, rel: &str, out: &mut BTreeSet<String>) -> Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().into_owned();
        let rel = if rel.is_empty() { name.clone() } else { format!("{rel}/{name}") };
        let ty = entry.file_type()?;
        if ty.is_dir() {
            list_files(&entry.path(), &rel, out)?;
        } else if ty.is_file() {
            out.insert(rel);
        }
    }
    Ok(())
}

fn compare(name: &str, src: &Path, oracle: &Path, pun: &Path) -> Result<Outcome> {
    let (mut a, mut b) = (BTreeSet::new(), BTreeSet::new());
    list_files(oracle, "", &mut a)?;
    list_files(pun, "", &mut b)?;
    let mut o = Outcome {
        name: name.into(),
        files: a.union(&b).count(),
        identical: 0,
        differing: vec![],
        oracle_changed: 0,
        pun_changed: 0,
    };
    for rel in a.union(&b) {
        let (x, y, s) = (fs::read(oracle.join(rel)).ok(), fs::read(pun.join(rel)).ok(), fs::read(src.join(rel)).ok());
        o.oracle_changed += (x != s) as usize;
        o.pun_changed += (y != s) as usize;
        if x == y {
            o.identical += 1;
        } else {
            o.differing.push(rel.clone());
        }
    }
    Ok(o)
}

/// Rewrites the rows for the projects that ran, keeps the rest.
fn write_scoreboard(path: &Path, outcomes: &[Outcome]) -> Result<()> {
    let old = fs::read_to_string(path).unwrap_or_default();
    let mut rows: Vec<String> = old
        .lines()
        .filter(|l| l.starts_with("| ") && !l.starts_with("| project") && !outcomes.iter().any(|o| l.starts_with(&format!("| {} |", o.name))))
        .map(String::from)
        .collect();
    for o in outcomes {
        rows.push(format!(
            "| {} | {} | {} | {} | {} | {} |",
            o.name, o.files, o.identical, o.differing.len(), o.oracle_changed, o.pun_changed
        ));
    }
    rows.sort();
    let md = format!(
        "# Scoreboard\n\nGenerated by `cargo run -p punphp-harness -- format`. Oracle: see docs/oracle.md.\n\
         `differing` must reach 0 on every project.\n\n\
         | project | files | identical | differing | pint changed | pun changed |\n\
         |---|---|---|---|---|---|\n{}\n",
        rows.join("\n")
    );
    fs::write(path, md)?;
    Ok(())
}
