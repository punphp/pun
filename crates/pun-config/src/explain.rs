//! `pun explain <rule>`: a rule's summary, group, sets, fix safety, upstream names and docs link, all from
//! rules/mapping.toml (context/plan/config.md Build 8).

use crate::{mapping, renamed, rules};
use anyhow::{Result, bail};
use std::fmt::Write;

/// The docs site's rule pages: a rule's page is this followed by its ID.
pub const RULE_DOCS_URL: &str = "https://pun.laravel.cloud/docs/rules/";

/// The text `pun explain <name>` prints. `name` is a rule ID, a rule ID without its command (`arrays/syntax`), an
/// upstream name (`no_alias_functions`, `method.notFound`) or a Pun name from before the rule IDs; anything but the
/// ID starts with a line naming the ID.
pub fn explain(name: &str) -> Result<String> {
    let exact = rules()
        .iter()
        .find(|(id, _)| id == name || id.split_once('/').unwrap().1 == name);
    if let Some((id, _)) = exact {
        return Ok(text(id));
    }
    // An upstream key as written (every entry has `pun`, `drop` or `unsupported`), then the loose match pun.toml's
    // errors use for other spellings and the old Pun names.
    let id = match entries().into_iter().find(|(_, k, _)| k == name) {
        Some((tool, _, e)) => {
            let str = |k| e.get(k).and_then(toml::Value::as_str).map(str::to_string);
            match str("pun") {
                Some(pun) if crate::rule(&pun).is_some() => Some(pun),
                Some(pun) => bail!("`{name}` ({tool}) is `{pun}` in pun.toml, not a rule"),
                None => bail!(
                    "`{name}` ({tool}) has no Pun rule: {}",
                    str("drop").or(str("unsupported")).unwrap()
                ),
            }
        }
        None => renamed(name).map(str::to_string),
    };
    match id {
        Some(id) => Ok(format!("`{name}` is `{id}` in Pun.\n\n{}", text(&id))),
        None => bail!("`{name}` is not a Pun rule or an upstream rule name"),
    }
}

/// Every `[upstream.<tool>]` entry, in file order: (tool, upstream key, entry).
fn entries() -> Vec<(String, String, toml::Value)> {
    let mut out = vec![];
    for (tool, s) in mapping::under("upstream") {
        for (k, e) in s.table() {
            out.push((tool.to_string(), k.clone(), e.clone()));
        }
    }
    out
}

/// The explanation of a rule ID that exists.
fn text(id: &str) -> String {
    let (_, e) = rules().iter().find(|(r, _)| r == id).unwrap();
    let str = |k| e.get(k).and_then(toml::Value::as_str);
    let list = |k| -> Vec<&str> {
        (e.get(k).and_then(toml::Value::as_array))
            .map(|a| a.iter().filter_map(toml::Value::as_str).collect())
            .unwrap_or_default()
    };
    let (command, rest) = id.split_once('/').unwrap();
    let group = rest.split_once('/').unwrap().0;
    let mut out = format!("{id}\n");
    if let Some(s) = str("summary") {
        out += &format!("{s}\n");
    }
    out += "\n";
    let mut row = |label: &str, value: &str| writeln!(out, "{label:<10}{value}").unwrap();
    row("group", &format!("{group}: {}", group_description(group)));
    let opt_in = |v: Vec<&str>| match v.is_empty() {
        true => "none (opt-in by name or group)".to_string(),
        false => v.join(", "),
    };
    match command {
        "format" => row("presets", &opt_in(list("presets"))),
        "lint" => row("sets", &opt_in(list("sets"))),
        _ => row("preset", &opt_in(str("preset").into_iter().collect())),
    }
    if let Some(i) = str("intent") {
        row("intent", i);
    }
    row("fix", str("fix").unwrap_or("none"));
    let fixed_by = list("fixed_by");
    if !fixed_by.is_empty() {
        row("fixed by", &fixed_by.join(", "));
    }
    // The upstream rows that map here (every `merged` source has one), then the Mago analyzer codes that also report
    // for it (`also`).
    let names: Vec<(String, String)> = (entries().into_iter())
        .filter(|(_, _, e)| e.get("pun").and_then(toml::Value::as_str) == Some(id))
        .map(|(tool, k, _)| (tool, k))
        .chain(
            (entries().into_iter())
                .filter(|(_, _, e)| {
                    (e.get("also").and_then(toml::Value::as_array))
                        .is_some_and(|a| a.iter().any(|r| r.as_str() == Some(id)))
                })
                .map(|(tool, k, _)| (format!("{tool}, also"), k)),
        )
        .collect();
    if names.is_empty() {
        row("upstream", "none (Pun's own rule)");
    }
    for (i, (tool, k)) in names.iter().enumerate() {
        row(
            if i == 0 { "upstream" } else { "" },
            &format!("{k} ({tool})"),
        );
    }
    row("docs", &format!("{RULE_DOCS_URL}{id}"));
    out
}

/// A group's one-line description from `[meta.groups]`.
fn group_description(group: &str) -> String {
    let groups = mapping::section(&["meta", "groups"]).table();
    groups[group].as_str().unwrap().to_string()
}
