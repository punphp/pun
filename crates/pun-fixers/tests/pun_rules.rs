//! Pun/* run order. docs/MAGO-EXTRAS.md: the Mago ports run at priority 0; the pipeline orders by priority
//! (desc), then name (strcmp). array_syntax (37) and no_unused_imports (-10) are PHP-CS-Fixer's priorities
//! (tests/metadata.json), so every Pun/* rule lands between them, alphabetically.
extern crate punphp_fixers as pun_fixers;
use pun_config::Rule;
use pun_format::Pipeline;

#[test]
fn pun_rules_run_at_priority_zero() {
    let mut pun: Vec<&str> = pun_fixers::all()
        .iter()
        .map(|f| f.name())
        .filter(|n| n.starts_with("Pun/"))
        .collect();
    assert_eq!(pun.len(), 17);
    let rule = |name: &str| Rule {
        name: name.into(),
        options: serde_json::Value::Null,
    };
    let rules: Vec<Rule> = ["no_unused_imports"]
        .into_iter()
        .chain(pun.iter().rev().copied())
        .chain(["array_syntax"])
        .map(rule)
        .collect();
    let pipeline = Pipeline::new(&rules, &pun_fixers::all).unwrap();
    pun.sort();
    let want: Vec<&str> = ["array_syntax"]
        .into_iter()
        .chain(pun)
        .chain(["no_unused_imports"])
        .collect();
    assert_eq!(
        pipeline.fixers().map(|f| f.name()).collect::<Vec<_>>(),
        want
    );
}

/// docs/MAGO-EXTRAS.md (the `pun.toml` example) marks exactly these Pun/* rules risky.
#[test]
fn pun_risky_rules() {
    let mut risky: Vec<&str> = pun_fixers::all()
        .iter()
        .filter(|f| f.name().starts_with("Pun/") && f.is_risky())
        .map(|f| f.name())
        .collect();
    risky.sort();
    assert_eq!(
        risky,
        [
            "Pun/final_controller",
            "Pun/no_implicit_model_query",
            "Pun/prefer_array_spread",
            "Pun/prefer_first_class_callable",
            "Pun/use_dedicated_expectation"
        ]
    );
}
