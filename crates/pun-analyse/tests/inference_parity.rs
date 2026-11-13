//! Inference PHPStan 2.3.5 reports at level max on reduced cases from a Laravel application
//! (fixtures/inference-parity/phpstan.neon): templates bound by class-strings, closures and their captures,
//! array_filter() and array_reduce() callbacks, integer division, object shapes and preg_match() matches.
//! fixtures/bleeding-edge holds cases that bleedingEdge.neon changes, with PHPStan's output with and without it.

use std::path::Path;

use pun_analyse::{Project, ProjectOptions, reports, run};
use serde_json::Value;

/// Pun's errors at level max on `files` of `fixture` against PHPStan's `expected` output.
fn assert_parity(fixture: &str, files: &[&str], options: ProjectOptions, expected: &str) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(fixture);
    let mut project = Project::new(files.iter().map(|f| root.join(f)).collect(), Vec::new());
    project.level = 10;
    project.options |= options;
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
        serde_json::from_slice(&std::fs::read(root.join(expected)).unwrap()).unwrap();
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
    assert_eq!(got, want, "{fixture}/{expected}");
}

#[test]
fn reduced_application_cases() {
    assert_parity(
        "inference-parity",
        &["a.php", "b.php", "c.php", "d.php", "e.php"],
        ProjectOptions::empty(),
        "expected-phpstan.json",
    );
}

/// `[analyse] bleeding-edge`: template arguments a body decides (`box([1, null])` is `Box<0|1, 1|null>`) and closure
/// parameters typed by what the closure is passed to (b.php); without it, the same cases and array_flip() and
/// array_intersect_key() shapes (a.php). Bleeding edge's unsealed array shapes are not ported, so a.php runs only
/// without it.
#[test]
fn bleeding_edge_inference() {
    assert_parity(
        "bleeding-edge",
        &["b.php"],
        ProjectOptions::UNRESOLVED_TEMPLATE_ARGUMENTS,
        "expected-bleeding-edge.json",
    );
    assert_parity(
        "bleeding-edge",
        &["a.php", "b.php"],
        ProjectOptions::empty(),
        "expected-without.json",
    );
}
