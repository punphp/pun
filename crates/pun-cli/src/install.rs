//! `pun install`: installs the locked dependencies into `vendor/`. The manifest comes from pun.toml's dependency
//! sections, or composer.json in a project not migrated yet; the lock from pun.lock, or composer.lock (imported into
//! pun.lock when pun.toml holds the dependencies). Nothing is resolved: without a lock, install stops.

use crate::render::{ColorChoice, Look, Role};
use anyhow::{Context, Result, bail};
use pun_packages::fetch::{Auth, Fetcher, composer_dirs};
use pun_packages::install::{Options, dirs, install};
use pun_packages::lock::Lock;
use pun_packages::scripts::Runner;
use serde_json::{Map, Value};
use std::path::PathBuf;
use std::time::Instant;

#[derive(clap::Args)]
pub(crate) struct InstallArgs {
    /// Leave out the dev dependencies (`require-dev`).
    #[arg(long)]
    no_dev: bool,
    /// Put the PSR-0 and PSR-4 classes in the class map too.
    #[arg(short = 'o', long)]
    optimize_autoloader: bool,
    /// Load classes from the class map only (implies --optimize-autoloader).
    #[arg(short = 'a', long)]
    classmap_authoritative: bool,
    /// Cache found and missing classes in APCu.
    #[arg(long)]
    apcu_autoloader: bool,
    /// The APCu cache prefix (implies --apcu-autoloader); without it each dump picks a random one.
    #[arg(long, value_name = "PREFIX")]
    apcu_autoloader_prefix: Option<String>,
    /// Skip writing the autoloader.
    #[arg(long)]
    no_autoloader: bool,
    /// Skip the root package's scripts.
    #[arg(long)]
    no_scripts: bool,
    /// Skip the Composer plugins Pun supports.
    #[arg(long)]
    no_plugins: bool,
    /// Leave this platform requirement out of the platform check (`php`, `ext-*`, globs with `*`); repeatable.
    #[arg(long, value_name = "REQ")]
    ignore_platform_req: Vec<String>,
    /// Leave every platform requirement out of the platform check.
    #[arg(long)]
    ignore_platform_reqs: bool,
    /// Run in this directory instead of the current one.
    #[arg(short = 'd', long, value_name = "DIR")]
    working_dir: Option<PathBuf>,
    /// Accepted for Composer's CI flags; Pun never asks and always installs from dists.
    #[arg(short = 'n', long, hide = true)]
    no_interaction: bool,
    #[arg(long, hide = true)]
    prefer_dist: bool,
    #[arg(long, hide = true)]
    no_progress: bool,
}

pub(crate) fn run(a: InstallArgs) -> Result<u8> {
    let started = Instant::now();
    let root = match &a.working_dir {
        Some(d) => d
            .canonicalize()
            .with_context(|| format!("can't open {}", d.display()))?,
        None => crate::root()?,
    };
    let look = Look::stderr(ColorChoice::Auto);
    let mut notes = vec![];
    let (manifest, lock, lock_name) = match pun_config::manifest::read(&root)? {
        Some(m) => {
            let lock = match Lock::read(&root)? {
                Some(lock) => lock,
                None => {
                    let composer = root.join("composer.lock");
                    let Ok(text) = std::fs::read_to_string(&composer) else {
                        bail!(
                            "pun.toml has dependencies but there is no pun.lock or composer.lock; Pun can't resolve versions yet, so run `composer update` once and then `pun install` again"
                        );
                    };
                    let lock = Lock::from_composer_lock(&text).context("invalid composer.lock")?;
                    std::fs::write(root.join("pun.lock"), lock.to_toml())
                        .context("can't write pun.lock")?;
                    notes.push("wrote pun.lock from composer.lock".to_string());
                    lock
                }
            };
            (m.json, lock, "pun.lock")
        }
        None => {
            let Ok(text) = std::fs::read_to_string(root.join("composer.json")) else {
                bail!(
                    "no dependencies here: pun.toml has no [require] and there is no composer.json"
                );
            };
            let json: Map<String, Value> =
                serde_json::from_str(&text).context("invalid composer.json")?;
            match Lock::read(&root)? {
                Some(lock) => (json, lock, "pun.lock"),
                None => {
                    let Ok(text) = std::fs::read_to_string(root.join("composer.lock")) else {
                        bail!(
                            "there is no composer.lock; Pun can't resolve versions yet, so run `composer update` once and then `pun install` again"
                        );
                    };
                    notes.push(
                        "run `pun migrate` to move the dependencies into pun.toml and pun.lock"
                            .to_string(),
                    );
                    (
                        json,
                        Lock::from_composer_lock(&text).context("invalid composer.lock")?,
                        "composer.lock",
                    )
                }
            }
        }
    };
    let options = Options {
        no_dev: a.no_dev,
        optimize: a.optimize_autoloader,
        authoritative: a.classmap_authoritative,
        apcu: a.apcu_autoloader,
        apcu_prefix: a.apcu_autoloader_prefix.clone(),
        no_autoloader: a.no_autoloader,
        no_plugins: a.no_plugins,
        ignore_platform: a.ignore_platform_req.clone(),
        ignore_all_platform: a.ignore_platform_reqs,
    };
    let secure_http =
        manifest.get("config").and_then(|c| c.get("secure-http")) != Some(&Value::Bool(false));
    let (home, cache) = composer_dirs().unzip();
    let fetcher = Fetcher::new(
        cache,
        Auth::load(&root, home.as_deref(), std::env::var("COMPOSER_AUTH").ok()),
        secure_http,
    );
    let (vendor, bin_dir) = dirs(&root.to_string_lossy(), &manifest);
    let mut scripts = Runner::new(&root, &vendor, &bin_dir, &manifest, !a.no_dev);
    let total = lock.packages.len() + if a.no_dev { 0 } else { lock.packages_dev.len() };
    eprintln!(
        "Installing {} from {lock_name}",
        crate::agent::count(total, "package")
    );
    if !a.no_scripts {
        scripts.dispatch("pre-install-cmd", &[])?;
    }
    let mut events = |event: &str| {
        if a.no_scripts {
            Ok(())
        } else {
            scripts.dispatch(event, &[])
        }
    };
    let report = install(&root, &manifest, &lock, &fetcher, &options, &mut events)?;
    drop(events);
    if !a.no_scripts {
        scripts.dispatch("post-install-cmd", &[])?;
    }

    let ok = look.paint(Role::Ok, look.glyph("✔", "+"));
    let warn = look.paint(Role::Warning, look.glyph("⚠", "!"));
    if report.stale_lock {
        eprintln!(
            " {warn} {lock_name} is older than the dependencies it locks: they changed since it was written, and Pun can't resolve versions yet, so run `composer update` and `pun migrate` to refresh it"
        );
    }
    let mut changes = vec![];
    for (what, n) in [
        ("installed", report.installed.len()),
        ("updated", report.updated.len()),
        ("removed", report.removed.len()),
    ] {
        if n > 0 {
            changes.push(format!("{n} {what}"));
        }
    }
    if report.kept > 0 {
        changes.push(format!("{} up to date", report.kept));
    }
    let cached = if report.cached > 0 {
        format!(" ({} from the cache)", report.cached)
    } else {
        String::new()
    };
    eprintln!(
        " {ok} packages: {}{cached}",
        if changes.is_empty() {
            "none".into()
        } else {
            changes.join(", ")
        }
    );
    if !a.no_autoloader {
        let noun = if report.classes == 1 {
            "class"
        } else {
            "classes"
        };
        eprintln!(" {ok} autoloader: {} {noun}", report.classes);
    }
    for w in report.warnings.iter().chain(&notes) {
        eprintln!(" {warn} {w}");
    }
    let secs = started.elapsed().as_secs_f64();
    eprintln!(
        "{}",
        look.paint(Role::Summary, &format!("Installed in {secs:.1}s"))
    );
    Ok(0)
}
