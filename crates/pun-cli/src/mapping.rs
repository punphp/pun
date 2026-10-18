//! What pun-cli reads from rules/mapping.toml besides pun-config's API: the groups and the analyse rule of each
//! PHPStan-family identifier.
// ponytail: pun-config owns rules/mapping.toml; the `pun_config::{groups, analyse_rule_of}` request row in PLAN.md
// §3 replaces this second reading of the file.

use std::collections::BTreeMap;
use std::sync::LazyLock;

const MAPPING: &str = include_str!("../../pun-config/rules/mapping.toml");

/// The body of `[<header>]`, up to the next table header.
fn section(header: &str) -> Option<&'static str> {
    let header = format!("\n[{header}]\n");
    let start = MAPPING.find(&header)?;
    let body = &MAPPING[start + header.len()..];
    Some(&body[..body.find("\n[").unwrap_or(body.len())])
}

/// `[meta.groups]`'s names.
pub(crate) fn groups() -> &'static [String] {
    static T: LazyLock<Vec<String>> = LazyLock::new(|| {
        let table: toml::Table =
            toml::from_str(section("meta.groups").unwrap_or("")).expect("rules/mapping.toml");
        table.keys().cloned().collect()
    });
    &T
}

/// The Pun rule of a PHPStan, Larastan, strict-rules or deprecation-rules identifier: the `variant` entries of
/// rules/mapping.toml, where `%s` matches any text.
pub(crate) fn rule_of_identifier(identifier: &str) -> Option<&'static str> {
    type Entries = (BTreeMap<String, String>, Vec<(Vec<String>, String)>);
    static ENTRIES: LazyLock<Entries> = LazyLock::new(|| {
        let mut exact = BTreeMap::new();
        let mut patterns = Vec::new();
        for tool in [
            "phpstan",
            "larastan",
            "phpstan-strict-rules",
            "phpstan-deprecation-rules",
        ] {
            let Some(body) = section(&format!("upstream.{tool}")) else {
                continue;
            };
            let table: toml::Table = toml::from_str(body).expect("rules/mapping.toml");
            for (key, entry) in table {
                let Some(pun) = entry.get("pun").and_then(toml::Value::as_str) else {
                    continue;
                };
                if key.contains("%s") {
                    patterns.push((key.split("%s").map(str::to_string).collect(), pun.into()));
                } else {
                    exact.insert(key, pun.to_string());
                }
            }
        }
        (exact, patterns)
    });
    let (exact, patterns) = &*ENTRIES;
    if let Some(r) = exact.get(identifier) {
        return Some(r.as_str());
    }
    (patterns.iter())
        .find(|(parts, _)| wildcard(parts, identifier))
        .map(|(_, r)| r.as_str())
}

/// Whether `text` is `parts` joined by any text (`%s`).
fn wildcard(parts: &[String], text: &str) -> bool {
    let (first, rest) = parts.split_first().expect("split gives one part");
    let Some(mut text) = text.strip_prefix(first.as_str()) else {
        return false;
    };
    let Some((last, middle)) = rest.split_last() else {
        return text.is_empty();
    };
    for part in middle {
        match text.find(part.as_str()) {
            Some(i) => text = &text[i + part.len()..],
            None => return false,
        }
    }
    text.len() >= last.len() && text.ends_with(last.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Values read off rules/mapping.toml by hand.
    #[test]
    fn identifiers_and_groups() {
        assert_eq!(
            rule_of_identifier("function.notFound"),
            Some("analyse/undefined/function")
        );
        // `%s.alwaysTrue`
        assert_eq!(
            rule_of_identifier("greater.alwaysTrue"),
            Some("analyse/dead-code/number-comparison-always-constant")
        );
        assert_eq!(
            rule_of_identifier("larastan.configCollection"),
            Some("analyse/laravel/config-collection")
        );
        assert_eq!(rule_of_identifier("no.such.identifier"), None);
        assert!(groups().iter().any(|g| g == "dead-code"));
        assert!(!groups().iter().any(|g| g == "dead"));
    }

    #[test]
    fn wildcards() {
        let parts = |p: &str| p.split("%s").map(str::to_string).collect::<Vec<_>>();
        assert!(wildcard(&parts("%s.always%s"), "a.alwaysb"));
        assert!(wildcard(&parts("%s.always%s"), ".always"));
        assert!(!wildcard(&parts("%s.always%s"), "a.alway"));
        assert!(wildcard(&parts("a%sb%sc"), "a1b2c"));
        assert!(!wildcard(&parts("a%sb%sc"), "a1c"));
        assert!(!wildcard(&parts("ab"), "abc"));
        assert!(wildcard(&parts("ab"), "ab"));
    }
}
