//! PHPStan-family identifiers (`method.notFound`, `larastan.configCollection`) and their rules/mapping.toml entries:
//! `[upstream.phpstan]`, `larastan`, `phpstan-strict-rules` and `phpstan-deprecation-rules`, keyed by identifier, where
//! `%s` in a key (`pattern = true`) stands for a run of identifier characters, as in PHPStan's own identifier list.

use crate::mapping;

const SECTIONS: [&str; 4] = [
    "phpstan",
    "larastan",
    "phpstan-strict-rules",
    "phpstan-deprecation-rules",
];

/// The mapping entry of an identifier: its own row in one of the sections, else the first `%s` row that matches it.
pub(crate) fn entry(identifier: &str) -> Option<&'static toml::Value> {
    let sections = SECTIONS.map(|s| mapping::section(&["upstream", s]).table());
    if let Some(e) = sections.iter().find_map(|t| t.get(identifier)) {
        return Some(e);
    }
    sections.iter().find_map(|t| {
        t.iter().find_map(|(k, e)| {
            let parts: Vec<&str> = k.split("%s").collect();
            (parts.len() > 1 && glob(&parts, identifier)).then_some(e)
        })
    })
}

/// The Pun rule (`analyse/undefined/method`, or a lint or format rule) of an identifier.
pub fn rule_of_identifier(identifier: &str) -> Option<&'static str> {
    entry(identifier)?.get("pun")?.as_str()
}

/// Whether `s` is `parts` joined by non-empty runs of identifier characters.
fn glob(parts: &[&str], s: &str) -> bool {
    let Some(rest) = s.strip_prefix(parts[0]) else {
        return false;
    };
    if parts.len() == 1 {
        return rest.is_empty();
    }
    let word = |c: &u8| c.is_ascii_alphanumeric() || *c == b'_';
    let run = rest.bytes().take_while(word).count();
    (1..=run).any(|n| glob(&parts[1..], &rest[n..]))
}
