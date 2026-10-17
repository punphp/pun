//! `pun list-files` (context/plan/config.md §7, after Mago's `list-files`): the files a command would check with the
//! current pun.toml, one per line, relative to the project root and sorted.
use crate::agent;
use anyhow::{Result, bail};
use clap::Args;
use pun_config::Options;
use pun_workspace::Command;
use std::io::Write;
use std::path::PathBuf;

#[derive(Args)]
pub struct ListFilesArgs {
    /// Paths to list instead of `[files] paths`.
    paths: Vec<PathBuf>,
    /// List one command's files: its own `exclude` added (`format` also applies Pint's finder rules).
    #[arg(long, value_enum)]
    command: Option<CommandArg>,
    /// End each name with a NUL byte, for `xargs -0`.
    #[arg(short = '0', long)]
    zero_terminate: bool,
    /// Output format: `json` prints an array.
    #[arg(long)]
    format: Option<String>,
    /// The configuration that should be used.
    #[arg(long)]
    config: Option<PathBuf>,
}

#[derive(Clone, Copy, clap::ValueEnum)]
enum CommandArg {
    Format,
    Lint,
    Analyse,
}

pub fn run(a: ListFilesArgs) -> Result<u8> {
    let json = match a.format.as_deref() {
        None => false,
        Some("json") => true,
        Some(f) => bail!("--format={f}: expected `json`"),
    };
    let root = super::root()?;
    let cfg = pun_config::load(
        &root,
        &Options {
            config: a.config,
            ..Default::default()
        },
    )?;
    let command = a.command.map(|c| match c {
        CommandArg::Format => (Command::Format, "format"),
        CommandArg::Lint => (Command::Lint, "lint"),
        CommandArg::Analyse => (Command::Analyse, "analyse"),
    });
    let files: Vec<String> = pun_workspace::files(&cfg, command.map(|c| c.0), &a.paths)?
        .iter()
        .map(|f| f.strip_prefix(&root).unwrap_or(f).display().to_string())
        .collect();
    let out = if json {
        serde_json::to_string(&files)? + "\n"
    } else if a.zero_terminate {
        files.iter().map(|f| format!("{f}\0")).collect()
    } else {
        let list: String = files.iter().map(|f| format!("{f}\n")).collect();
        if agent::enabled() {
            let detail: Vec<String> = command
                .map(|c| format!("{} set", c.1))
                .into_iter()
                .collect();
            agent::render(&agent::count(files.len(), "file"), &detail, "", &[], false) + &list
        } else {
            list
        }
    };
    std::io::stdout().write_all(out.as_bytes())?;
    Ok(0)
}
