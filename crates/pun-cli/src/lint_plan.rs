//! Which lint rules run on a file, at which level and whether their fixes apply (context/plan/lint.md §3, Build 2;
//! oxlint's model): pun.toml's resolved `[lint]` rules, then each `[[lint.overrides]]` whose `files` globs match the
//! file (later ones win), then `-A` / `-W` / `-D` in the order given. `-A` turns a rule or group off, `-W` and `-D`
//! turn it on at `warn` or `error`; a group means every rule of it Pun can run.
//!
//! A rule runs natively (a `pun_lint` rule on Mago's tree), in the token pass (a PHP-CS-Fixer fixer), or in the
//! engine (`lint/comments/no-unused-ignore`). A rule named in pun.toml, an override or on the command line that Pun
//! can't run is a config error; one a set or a group turns on is skipped.

use anyhow::{Result, bail};
use pun_problem::Level;
use pun_workspace::Exclusions;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::Path;
use std::sync::LazyLock;

/// The engine's own rule: an `@pun-ignore` ID that suppressed nothing.
pub(crate) const UNUSED_IGNORE: &str = "lint/comments/no-unused-ignore";

/// `-A`, `-W` and `-D`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum CliLevel {
    Allow,
    Warn,
    Deny,
}

/// How a rule runs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Engine {
    /// A `pun_lint` rule.
    Native,
    /// A token fixer, by its PHP-CS-Fixer name.
    Token(&'static str),
    /// Reported by the engine itself.
    Builtin,
}

/// A rule that is on: its level, whether `--fix` applies its fix (`fix = "off"` makes it report-only), its options.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct On {
    pub(crate) level: Level,
    pub(crate) fix: bool,
    pub(crate) options: Value,
}

/// Lint rule IDs a ported token fixer backs, to the fixer's name.
fn token_rules() -> &'static HashMap<&'static str, &'static str> {
    static T: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
        let ported: BTreeSet<&'static str> = pun_fixers::all().iter().map(|f| f.name()).collect();
        (pun_config::fixer_entries().iter())
            .filter_map(|(fixer, e)| {
                let id = e.get("pun")?.as_str()?;
                let fixer = ported.get(fixer.as_str())?;
                id.starts_with("lint/").then_some((id, *fixer))
            })
            .collect()
    });
    &T
}

/// The native rules' IDs.
pub(crate) fn native_rules() -> &'static BTreeSet<&'static str> {
    static T: LazyLock<BTreeSet<&'static str>> =
        LazyLock::new(|| pun_lint::rules::all().iter().map(|r| r.id()).collect());
    &T
}

/// How `id` runs, or `None` when Pun can't run it yet.
pub(crate) fn engine(id: &str) -> Option<Engine> {
    if native_rules().contains(id) {
        return Some(Engine::Native);
    }
    if id == UNUSED_IGNORE {
        return Some(Engine::Builtin);
    }
    token_rules().get(id).map(|f| Engine::Token(f))
}

fn level(l: pun_config::Level) -> Level {
    match l {
        pun_config::Level::Warn => Level::Warning,
        pun_config::Level::Error => Level::Error,
    }
}

pub(crate) struct Plan {
    base: BTreeMap<String, On>,
    overrides: Vec<(Exclusions, Vec<pun_config::LintSetting>)>,
    cli: Vec<(CliLevel, Vec<String>)>,
}

impl Plan {
    /// The plan of `cfg`'s `[lint]` and the command line's levels. `Err` holds the rules Pun can't run that are
    /// named somewhere.
    pub(crate) fn new(
        cfg: &pun_config::Config,
        cli: &[(CliLevel, String)],
    ) -> Result<std::result::Result<Plan, Vec<String>>> {
        let mut unported = vec![];
        let mut base = BTreeMap::new();
        for r in &cfg.lint.rules {
            if engine(&r.id).is_none() {
                if r.named {
                    unported.push(r.id.clone());
                }
                continue;
            }
            let on = On {
                level: level(r.level),
                fix: r.fix,
                options: r.options.clone(),
            };
            base.insert(r.id.clone(), on);
        }
        let mut overrides = vec![];
        for o in &cfg.lint.overrides {
            for s in &o.rules {
                if s.level.is_some() && engine(&s.id).is_none() {
                    unported.push(s.id.clone());
                }
            }
            overrides.push((
                Exclusions::globs(&cfg.root, o.files.clone())?,
                o.rules.clone(),
            ));
        }
        let mut levels = vec![];
        for (l, name) in cli {
            let ids = targets(name)?;
            if *l != CliLevel::Allow {
                unported.extend(ids.iter().filter(|id| engine(id).is_none()).cloned());
            }
            levels.push((*l, ids));
        }
        if !unported.is_empty() {
            unported.dedup();
            return Ok(Err(unported));
        }
        Ok(Ok(Plan {
            base,
            overrides,
            cli: levels,
        }))
    }

    /// Every rule that is on for some file: the base rules, those an override turns on, those `-W` / `-D` name.
    pub(crate) fn mentioned(&self) -> BTreeSet<&str> {
        let overridden = (self.overrides.iter()).flat_map(|(_, s)| {
            s.iter()
                .filter(|s| s.level.is_some())
                .map(|s| s.id.as_str())
        });
        let cli = (self.cli.iter())
            .filter(|(l, _)| *l != CliLevel::Allow)
            .flat_map(|(_, ids)| ids.iter().map(String::as_str));
        (self.base.keys().map(String::as_str))
            .chain(overridden)
            .chain(cli)
            .filter(|id| engine(id).is_some())
            .collect()
    }

    /// The rules on for `path`, by ID.
    pub(crate) fn for_file(&self, path: &Path) -> BTreeMap<String, On> {
        let mut on = self.base.clone();
        for (globs, settings) in &self.overrides {
            if !globs.is_excluded(path) {
                continue;
            }
            for s in settings {
                match s.level {
                    None => drop(on.remove(&s.id)),
                    Some(l) => {
                        // pun-config rejects options in an override: the rule keeps its [lint.rules] options.
                        let options = on.get(&s.id).map_or(Value::Null, |o| o.options.clone());
                        let rule = On {
                            level: level(l),
                            fix: s.fix,
                            options,
                        };
                        on.insert(s.id.clone(), rule);
                    }
                }
            }
        }
        for (l, ids) in &self.cli {
            for id in ids.iter().filter(|id| engine(id).is_some()) {
                let level = match l {
                    CliLevel::Allow => {
                        on.remove(id);
                        continue;
                    }
                    CliLevel::Warn => Level::Warning,
                    CliLevel::Deny => Level::Error,
                };
                on.entry(id.clone())
                    .and_modify(|o| o.level = level)
                    .or_insert(On {
                        level,
                        fix: true,
                        options: Value::Null,
                    });
            }
        }
        on
    }
}

/// The rule IDs a `-A` / `-W` / `-D` value names: a rule (`lint/style/x` or `style/x`), or a group (`lint/style` or
/// `style`) as every rule of it Pun can run.
fn targets(name: &str) -> Result<Vec<String>> {
    let id = match name.strip_prefix("lint/") {
        Some(_) => name.to_string(),
        None => format!("lint/{name}"),
    };
    if pun_config::rule(&id).is_some() {
        return Ok(vec![id]);
    }
    let prefix = format!("{id}/");
    let in_group: Vec<String> = (native_rules().iter().copied())
        .chain(token_rules().keys().copied())
        .chain([UNUSED_IGNORE])
        .filter(|r| r.starts_with(&prefix) && r.matches('/').count() == 2)
        .map(str::to_string)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    if in_group.is_empty() && !is_group(&id) {
        bail!("`{name}` is not a lint rule or group");
    }
    Ok(in_group)
}

/// Whether `lint/<group>` names a group of rules/mapping.toml's `[meta.groups]`.
fn is_group(id: &str) -> bool {
    (id.strip_prefix("lint/")).is_some_and(|group| {
        !group.contains('/') && pun_config::groups().iter().any(|g| g == group)
    })
}
