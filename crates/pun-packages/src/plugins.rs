//! Composer plugins. Pun runs no PHP, so each plugin it supports is ported natively and runs at the point Composer
//! would call it; an allowed plugin without a port is reported, so the user knows its side effects are missing.

use crate::installed::Installed;
use crate::php::json_pretty_escaped;
use crate::version::name_matches;
use serde_json::{Map, Value};

/// A plugin's work after the autoloader is written (`post-autoload-dump`): files to write, relative to the vendor dir.
pub struct Output {
    pub files: Vec<(String, Vec<u8>)>,
    /// Allowed plugins pun has no port for.
    pub unsupported: Vec<String>,
}

/// Plugins whose Composer hooks change nothing an install from a lock needs, so skipping them loses nothing.
/// Discovery only acts when an update leaves an interface without an implementation.
const INERT: &[&str] = &["php-http/discovery"];

/// Whether `config.allow-plugins` lets `name` run: `true` or `false` for all, or name patterns (`*` globs) of which the
/// first that matches decides, as in Composer's `PluginManager::isPluginAllowed`.
pub fn allowed(config: &Map<String, Value>, name: &str) -> bool {
    match config.get("allow-plugins") {
        Some(Value::Bool(b)) => *b,
        Some(Value::Object(rules)) => (rules.iter())
            .find(|(pattern, _)| name_matches(pattern, name))
            .is_some_and(|(_, allow)| allow.as_bool() == Some(true)),
        _ => false,
    }
}

/// Runs the `post-autoload-dump` hooks of the installed, allowed plugins. `packages` are in `installed.json` order.
pub fn after_dump(root: &Map<String, Value>, packages: &[&Installed]) -> Output {
    let config = root
        .get("config")
        .and_then(Value::as_object)
        .cloned()
        .unwrap_or_default();
    let mut out = Output {
        files: vec![],
        unsupported: vec![],
    };
    for p in packages {
        if p.package.get("type").and_then(Value::as_str) != Some("composer-plugin") {
            continue;
        }
        let name = p.name();
        if !allowed(&config, &name) {
            continue;
        }
        match name.as_str() {
            "pestphp/pest-plugin" => out
                .files
                .push(("pest-plugins.json".into(), pest_plugins(root, packages))),
            n if INERT.contains(&n) => {}
            n => out.unsupported.push(n.to_string()),
        }
    }
    out
}

/// `pest:dump-plugins`: every package's `extra.pest.plugins`, then the root's, as `json_encode(…, JSON_PRETTY_PRINT)`.
fn pest_plugins(root: &Map<String, Value>, packages: &[&Installed]) -> Vec<u8> {
    let mut plugins = vec![];
    for extra in packages
        .iter()
        .map(|p| p.package.get("extra"))
        .chain([root.get("extra")])
    {
        if let Some(Value::Array(list)) = extra
            .and_then(|e| e.get("pest"))
            .and_then(|p| p.get("plugins"))
        {
            plugins.extend(list.iter().cloned());
        }
    }
    json_pretty_escaped(&Value::Array(plugins)).into_bytes()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn package(p: Value) -> Installed {
        Installed {
            package: p.as_object().unwrap().clone(),
            path: None,
            dev: false,
        }
    }

    #[test]
    fn allow_plugins_rules() {
        let config = |v: Value| json!({"allow-plugins": v}).as_object().unwrap().clone();
        assert!(allowed(&config(json!(true)), "acme/x"));
        assert!(!allowed(&config(json!(false)), "acme/x"));
        assert!(!allowed(&Map::new(), "acme/x"));
        // The first rule that matches decides (PluginManager::isPluginAllowed), names compare case-insensitively,
        // and `*` matches anywhere.
        let rules = config(json!({"acme/off": false, "acme/*": true, "*/pest-plugin": true}));
        assert!(allowed(&rules, "acme/x") && allowed(&rules, "Pestphp/Pest-Plugin"));
        assert!(!allowed(&rules, "acme/off") && !allowed(&rules, "other/x"));
        let rules = config(json!({"acme/*": true, "acme/off": false}));
        assert!(allowed(&rules, "acme/off"));
        // `packageNameToRegexp("a*b*c")` is `{^a.*b.*c$}i`.
        let rules = config(json!({"a*b*c": true}));
        assert!(allowed(&rules, "AxxByyC") && !allowed(&rules, "axxc"));
    }

    /// pest-plugins.json as pest-plugin's `pest:dump-plugins` writes it: every installed package's `extra.pest.plugins`
    /// in installed.json order, then the root's, as `json_encode(..., JSON_PRETTY_PRINT)` (PHP 8.5 gives the text below).
    #[test]
    fn after_dump_runs_the_allowed_ports() {
        let root = json!({"extra": {"pest": {"plugins": ["Root\\P"]}}, "config": {"allow-plugins": {"pestphp/pest-plugin": true, "php-http/discovery": true, "acme/plugin": true}}});
        let packages = [
            package(json!({"name": "pestphp/pest-plugin", "type": "composer-plugin"})),
            package(json!({"name": "php-http/discovery", "type": "composer-plugin"})),
            package(json!({"name": "acme/plugin", "type": "composer-plugin"})),
            package(json!({"name": "acme/denied", "type": "composer-plugin"})),
            package(json!({"name": "acme/lib", "extra": {"pest": {"plugins": ["A\\B"]}}})),
        ];
        let refs: Vec<&Installed> = packages.iter().collect();
        let out = after_dump(root.as_object().unwrap(), &refs);
        assert_eq!(
            out.files,
            [(
                "pest-plugins.json".to_string(),
                b"[\n    \"A\\\\B\",\n    \"Root\\\\P\"\n]".to_vec()
            )]
        );
        assert_eq!(out.unsupported, ["acme/plugin"]);
    }
}
