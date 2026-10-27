//! Foreach entry holders and template identity, against PHPStan 2.3.1.

use std::path::Path;

use pun_analyse::{Project, run};
use pun_phpstan::config::{Options, resolve};
use serde_json::Value;

#[test]
fn foreach_entry_scopes_match_phpstan() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/scope-joins");
    let expected: Value =
        serde_json::from_slice(&std::fs::read(root.join("expected-foreach-entry.json")).unwrap())
            .unwrap();
    let mut failures = Vec::new();
    for mode in ["default", "quiet", "foreach-off", "loop-off"] {
        let config = resolve(&Options {
            config: Some(
                root.join(format!("{mode}.neon"))
                    .to_string_lossy()
                    .into_owned(),
            ),
            ..Options::for_project(&root.to_string_lossy())
        })
        .unwrap();
        let mut project = Project::from_config(&root, &config.parameters);
        project.analysed = vec![root.join("foreach-entry.php")];
        let mut actual: Vec<_> = run(&project)
            .unwrap()
            .into_iter()
            .map(|error| {
                (
                    Path::new(&error.file)
                        .file_name()
                        .unwrap()
                        .to_string_lossy()
                        .into_owned(),
                    error.line,
                    error.identifier,
                    error.message,
                    error.tip,
                    error.can_be_ignored,
                )
            })
            .collect();
        actual.sort();
        let actual = serde_json::to_value(actual).unwrap();
        if actual != expected[mode] {
            let want = expected[mode].as_array().unwrap();
            let got = actual.as_array().unwrap();
            let missing: Vec<_> = want.iter().filter(|e| !got.contains(e)).collect();
            let extra: Vec<_> = got.iter().filter(|e| !want.contains(e)).collect();
            failures.push(serde_json::json!({
                "mode": mode,
                "actual": got.len(),
                "expected": want.len(),
                "missing": missing,
                "extra": extra,
            }));
        }
    }
    assert!(
        failures.is_empty(),
        "{}",
        serde_json::to_string_pretty(&failures).unwrap()
    );
}
