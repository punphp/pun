//! rules/mapping.toml covers exactly the fixers the engine knows: every native fixer (pun_fixers::all) and every
//! fixer of the pinned Pint PHAR (src/gitlab_fixers.json, exported from it) has an entry, and every entry is one of
//! them. Bumping PHP-CS-Fixer or adding a fixer fails here until the mapping decides where the rule lives.
use std::collections::BTreeSet;

#[test]
fn mapping_covers_every_fixer_and_nothing_else() {
    let pint: serde_json::Map<String, serde_json::Value> =
        serde_json::from_str(include_str!("../src/gitlab_fixers.json")).unwrap();
    let native: BTreeSet<String> = pun_fixers::all().iter().map(|f| f.name().to_string()).collect();
    let known: BTreeSet<String> = pint.keys().cloned().chain(native.iter().cloned()).collect();
    let mapped: BTreeSet<String> = pun_config::fixer_entries().keys().cloned().collect();
    assert_eq!(
        (
            known.difference(&mapped).collect::<Vec<_>>(),
            mapped.difference(&known).collect::<Vec<_>>()
        ),
        (vec![], vec![]),
        "(fixers without a mapping entry, mapping entries that are no fixer)"
    );
    // A native fixer always maps to a Pun rule (a dropped or unsupported fixer would never run).
    let unmapped: Vec<_> = native
        .iter()
        .filter(|f| pun_config::rule_of(f).is_none())
        .collect();
    assert!(unmapped.is_empty(), "{unmapped:?}");
}
