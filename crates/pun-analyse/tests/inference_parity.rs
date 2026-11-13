//! Inference PHPStan 2.3.5 reports at level max on reduced cases from a Laravel application
//! (fixtures/inference-parity/phpstan.neon): templates bound by class-strings, closures and their captures,
//! array_filter() and array_reduce() callbacks, integer division, object shapes and preg_match() matches.

use std::path::Path;

use pun_analyse::{Project, reports, run};
use serde_json::Value;

#[test]
fn reduced_application_cases() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/inference-parity");
    let files = ["a.php", "b.php", "c.php", "d.php", "e.php"];
    let mut project = Project::new(files.iter().map(|f| root.join(f)).collect(), Vec::new());
    project.level = 10;
    let prefix = format!("{}/", root.display());
    let got: Vec<_> = run(&project)
        .unwrap()
        .into_iter()
        .map(|e| {
            let file = e.file.strip_prefix(&prefix).unwrap_or(&e.file).to_string();
            (file, e.line, e.message)
        })
        .collect();
    let oracle: Value =
        serde_json::from_slice(&std::fs::read(root.join("expected-phpstan.json")).unwrap())
            .unwrap();
    let mut want = Vec::new();
    for (file, entry) in oracle["files"].as_object().unwrap() {
        for e in entry["messages"].as_array().unwrap() {
            if reports(e["identifier"].as_str().unwrap_or("")) {
                want.push((
                    file.clone(),
                    e["line"].as_i64().unwrap(),
                    e["message"].as_str().unwrap().to_string(),
                ));
            }
        }
    }
    want.sort();
    assert_eq!(got, want);
}
