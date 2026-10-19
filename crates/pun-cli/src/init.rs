//! `pun init` (context/plan/init.md): writes pun.toml for a project without a Pint, PHP-CS-Fixer, PHPStan, Rector or
//! Mago config. It reads composer.json and composer.lock, asks three questions (code style, lint sets, analysis
//! preset) with defaults that fit the project, and writes only those three choices. A project with an old config is
//! `pun migrate`'s: a terminal is offered it, anything else is told so and nothing is written. `--yes`, agent mode,
//! `CI` or no terminal ask nothing and take the defaults or the flags. Workflow wiring (composer scripts, CI, hooks) is
//! printed, never written.
use crate::render::{ColorChoice, Look, Renderer};
use crate::{agent, migrate, root};
use anyhow::{Result, bail};
use dialoguer::console::Term;
use pun_config::composer::{Composer, Locked};
use pun_config::{AnalysePreset, LINT_SETS};
use std::io::IsTerminal;
use std::path::Path;

#[derive(clap::Args)]
pub(crate) struct InitArgs {
    /// The code style, `[format] preset` (default: laravel for a Laravel app, else psr12).
    #[arg(long, value_enum)]
    style: Option<Style>,
    /// The lint sets, comma-separated (default: dead-code,modernize).
    #[arg(long, value_delimiter = ',', value_name = "SETS")]
    lint_sets: Option<Vec<String>>,
    /// The analysis preset: basic, essential, recommended, strict or max (default: recommended).
    #[arg(long, value_name = "PRESET")]
    analyse: Option<String>,
    /// Don't ask: take the defaults and the flags.
    #[arg(long, short)]
    yes: bool,
}

/// The code styles init offers. `empty` stays valid in pun.toml but is not offered (init.md Decisions).
#[derive(Clone, Copy, PartialEq, Debug, clap::ValueEnum)]
enum Style {
    Laravel,
    Psr12,
}

const STYLES: [&str; 2] = ["laravel", "psr12"];
/// The lint sets ticked by default (init.md §2).
const DEFAULT_SETS: [&str; 2] = ["dead-code", "modernize"];

pub(crate) fn run(a: InitArgs) -> Result<u8> {
    let root = root()?;
    if root.join("pun.toml").exists() {
        bail!(
            "pun.toml exists; `pun init` doesn't overwrite it. Edit it, or move it away and run `pun init` again."
        );
    }
    let sets = match a.lint_sets {
        Some(sets) => valid_sets(sets)?,
        None => DEFAULT_SETS.map(String::from).to_vec(),
    };
    let analyse = match a.analyse.as_deref() {
        Some(p) => preset(p)?,
        None => AnalysePreset::Recommended,
    };
    let composer = pun_config::composer::detect(&root)?;
    let style = match a.style {
        Some(s) => s,
        None if composer.laravel.is_some() => Style::Laravel,
        None => Style::Psr12,
    };
    let human = !agent::enabled();
    let interactive = !(a.yes || agent::enabled() || ci())
        && std::io::stdin().is_terminal()
        && std::io::stdout().is_terminal();
    let old: Vec<String> = (migrate::detect(&root, Path::new("pint.json")).into_iter())
        .map(|d| d.1)
        .collect();
    if !old.is_empty() && !interactive {
        bail!(
            "Found {}: this project fits `pun migrate`, which turns {} into pun.toml. Nothing written.",
            and(&old),
            if old.len() == 1 { "it" } else { "them" }
        );
    }
    if human {
        println!(
            "\n  Pun · new project setup\n  {}\n",
            found(&root, &composer)
        );
    }
    let term = Term::stdout();
    if !old.is_empty() {
        let migrate = dialoguer::Confirm::with_theme(&theme())
            .with_prompt(format!(
                "Found {}. Migrate {} instead?",
                and(&old),
                if old.len() == 1 { "it" } else { "them" }
            ))
            .default(true)
            .interact_on(&term)?;
        if migrate {
            return migrate::run(migrate::MigrateArgs::default());
        }
    }
    let (style, sets, analyse) = if interactive {
        ask(&term, style, &sets, analyse)?
    } else {
        (style, sets, analyse)
    };
    let style = STYLES[style as usize];
    let toml = format!(
        "# Written by `pun init`.\n[format]\npreset = \"{style}\"\n\n[lint]\nsets = [{}]\n\n[analyse]\npreset = \"{analyse}\"\n",
        sets.iter()
            .map(|s| format!("\"{s}\""))
            .collect::<Vec<_>>()
            .join(", ")
    );
    pun_config::from_toml(&root, &toml)?;
    std::fs::write(root.join("pun.toml"), &toml)?;
    if !human {
        let detail = [
            "pun.toml written".to_string(),
            format!("style {style}"),
            format!("lint sets {}", none_or(&sets.join(" "))),
            format!("analyse {analyse}"),
        ];
        let next = ["next: run `pun check`, `pun check --fix` before committing; add `pun check` to CI and composer scripts".to_string()];
        print!("{}", agent::render("ok", &detail, "", &next, false));
        return Ok(0);
    }
    if !interactive {
        println!("Code style: {style}");
        println!("Lint sets: {}", none_or(&sets.join(", ")));
        println!("Analysis preset: {analyse}");
    }
    if let Some(eco) = ecosystems(&composer) {
        println!("\n  {eco} rules turn on by themselves (composer.lock).");
    }
    let r = Renderer::new(Look::stdout(ColorChoice::Auto), &root);
    println!("\n  {}", r.done("Wrote pun.toml").trim_end());
    println!("\n  Done. Run `pun check` any time; `pun check --fix` before you commit.");
    println!("  Add `pun check` to CI and your composer scripts.");
    Ok(0)
}

/// The three questions, each starting on its default.
fn ask(
    term: &Term,
    style: Style,
    sets: &[String],
    analyse: AnalysePreset,
) -> Result<(Style, Vec<String>, AnalysePreset)> {
    let theme = theme();
    let style = dialoguer::Select::with_theme(&theme)
        .with_prompt("Code style")
        .items(STYLES)
        .default(style as usize)
        .interact_on(term)?;
    let ticked: Vec<bool> = LINT_SETS
        .iter()
        .map(|s| sets.iter().any(|t| t == s))
        .collect();
    let sets = dialoguer::MultiSelect::with_theme(&theme)
        .with_prompt("Lint sets (space toggles, enter confirms)")
        .items(LINT_SETS)
        .defaults(&ticked)
        .interact_on(term)?;
    let names = AnalysePreset::ALL.map(AnalysePreset::name);
    let analyse = dialoguer::Select::with_theme(&theme)
        .with_prompt("Analysis preset")
        .items(names)
        .default(analyse as usize)
        .interact_on(term)?;
    Ok((
        [Style::Laravel, Style::Psr12][style],
        sets.into_iter().map(|i| LINT_SETS[i].to_string()).collect(),
        AnalysePreset::ALL[analyse],
    ))
}

/// The prompts' look (`? Code style ›`, then `✔ Code style · laravel`; context/plan/init.md §2), shared with
/// `pun migrate`'s checklist.
pub(crate) fn theme() -> dialoguer::theme::ColorfulTheme {
    dialoguer::theme::ColorfulTheme::default()
}

/// `--lint-sets`, each checked against the lint sets with schema.rs's wording. `--lint-sets ""` is no sets.
fn valid_sets(sets: Vec<String>) -> Result<Vec<String>> {
    let sets: Vec<String> = sets.into_iter().filter(|s| !s.is_empty()).collect();
    for s in &sets {
        if !LINT_SETS.contains(&s.as_str()) {
            bail!(
                "--lint-sets: `{s}` is not a lint set; the sets are {}",
                quoted(&LINT_SETS)
            );
        }
    }
    Ok(sets)
}

/// `--analyse`, with schema.rs's wording for an unknown preset.
fn preset(p: &str) -> Result<AnalysePreset> {
    match AnalysePreset::ALL.into_iter().find(|a| a.name() == p) {
        Some(a) => Ok(a),
        None => bail!(
            "--analyse {p}: expected {}",
            quoted(&AnalysePreset::ALL.map(AnalysePreset::name))
        ),
    }
}

/// schema.rs's list: each item in backticks, comma-separated.
fn quoted(items: &[&str]) -> String {
    let q: Vec<String> = items.iter().map(|s| format!("`{s}`")).collect();
    q.join(", ")
}

/// `a`, `a and b`, `a, b and c`.
fn and(items: &[String]) -> String {
    match items {
        [rest @ .., last] if !rest.is_empty() => format!("{} and {last}", rest.join(", ")),
        _ => items.join(""),
    }
}

fn none_or(s: &str) -> String {
    if s.is_empty() {
        "none".into()
    } else {
        s.into()
    }
}

/// `CI` set to anything but empty, `0` or `false`, as CI services do.
fn ci() -> bool {
    std::env::var("CI").is_ok_and(|v| !matches!(v.as_str(), "" | "0" | "false"))
}

/// The header's second line: what composer.json and composer.lock say, and which of them were read.
fn found(root: &Path, c: &Composer) -> String {
    let version = |l: &Locked| match l.semver {
        Some(v) => v.major.to_string(),
        None => l.version.clone(),
    };
    let mut parts = vec![];
    if let Some(l) = &c.laravel {
        parts.push(format!("Laravel {}", version(l)));
    }
    if let Some(l) = &c.pest {
        parts.push(format!("Pest {}", version(l)));
    } else if let Some(l) = &c.phpunit {
        // Pest runs on PHPUnit, so PHPUnit is named only without Pest.
        parts.push(format!("PHPUnit {}", version(l)));
    }
    if let Some(php) = &c.php {
        parts.push(format!("PHP {}", php.constraint));
    }
    if parts.is_empty() {
        return "No Laravel, Pest, PHPUnit or PHP version found in composer.json or composer.lock"
            .into();
    }
    let files: Vec<String> = (c.sources.iter())
        .map(|s| s.strip_prefix(root).unwrap_or(s).display().to_string())
        .collect();
    format!("Found {} ({})", parts.join(", "), files.join(", "))
}

/// The ecosystems composer.lock turns on, as the line after the questions names them.
fn ecosystems(c: &Composer) -> Option<String> {
    let names: Vec<String> = [
        (c.laravel.is_some(), "Laravel"),
        (c.pest.is_some(), "Pest"),
        (c.phpunit.is_some(), "PHPUnit"),
    ]
    .into_iter()
    .filter(|e| e.0)
    .map(|e| e.1.to_string())
    .collect();
    (!names.is_empty()).then(|| and(&names))
}
