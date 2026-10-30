//! Invalid literal-array targets still evaluate their children in PHPStan's running list-item scope.

use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

use pun_analyse::{Project, analyse_project};
use serde_json::{Value, json};

// The capture stays complete. Purity collectors and other unrelated rules have separate owners.
const CONTRACT: &[&str] = &[
    "variable.undefined",
    "variable.nameNotString",
    "assign.invalidExpr",
    "assign.this",
    "nullsafe.assign",
    "nullsafe.byRef",
    "offsetAccess.noDim",
    "phpstan.dumpType",
    "phpstan.parse",
];

struct Inputs(PathBuf);

impl Inputs {
    fn new() -> Self {
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let root = std::env::temp_dir().join(format!(
            "pun-assignment-target-reads-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir_all(&root).unwrap();
        Self(root)
    }
}

impl Drop for Inputs {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.0).unwrap();
    }
}

fn check(level: u8, boundary_reads: bool) {
    let inputs: Value = serde_json::from_str(if boundary_reads {
        include_str!("fixtures/assignment-target-reads/boundaries.json")
    } else {
        include_str!("fixtures/assignment-target-reads/inputs.json")
    })
    .unwrap();
    let expected: Value = serde_json::from_str(include_str!(
        "fixtures/assignment-target-reads/expected.json"
    ))
    .unwrap();
    let root = Inputs::new();
    let files = inputs
        .as_object()
        .unwrap()
        .iter()
        .map(|(name, source)| {
            let path = root.0.join(format!("{name}.php"));
            let source = if boundary_reads {
                &source["source"]
            } else {
                source
            };
            std::fs::write(&path, source.as_str().unwrap()).unwrap();
            path
        })
        .collect();
    let mut project = Project::new(files, Vec::new());
    project.php_version = 80500;
    project.level = level;
    project.check_maybe_undefined_variables = level >= 1;
    let actual = json!(
        analyse_project(&project)
            .unwrap()
            .errors
            .into_iter()
            .map(|error| {
                json!({
                    "file": std::path::Path::new(&error.file).file_name().unwrap().to_str().unwrap(),
                    "line": error.line, "identifier": error.identifier, "message": error.message,
                    "tip": error.tip, "ignorable": error.can_be_ignored,
                })
            })
            .collect::<Vec<_>>()
    );
    let selected = |errors: &Value| {
        errors
            .as_array()
            .unwrap()
            .iter()
            .filter(|error| {
                let name = error["file"]
                    .as_str()
                    .unwrap()
                    .strip_suffix(".php")
                    .unwrap();
                let Some(input) = inputs.get(name) else {
                    return false;
                };
                if boundary_reads {
                    error["identifier"] == "variable.undefined"
                        && input["read_lines"]
                            .as_array()
                            .unwrap()
                            .contains(&error["line"])
                } else {
                    CONTRACT.contains(&error["identifier"].as_str().unwrap())
                }
            })
            .cloned()
            .collect::<Vec<_>>()
    };
    assert_eq!(
        selected(&actual),
        selected(&expected[level.to_string()]),
        "level {level}"
    );
}

#[test]
fn literal_array_targets_preserve_level_zero_suppression_and_assignment_errors() {
    check(0, false);
}

#[test]
fn literal_array_targets_observe_reads_after_rhs_and_prior_list_items() {
    for level in [1, 2, 7, 8, 9, 10] {
        check(level, false);
    }
}

#[test]
fn unreachable_and_dynamic_targets_keep_their_own_read_observations() {
    // The full captures retain independent gaps: unreachable syntax-rule callbacks and
    // dynamic-variable writes. This boundary checks only the reads, not those other owners.
    for level in [0, 1, 2, 7, 8, 9, 10] {
        check(level, true);
    }
}
