//! PHPParser's List_/Array_ roles and byRef ArrayItem values, checked through PHPStan's public Analyser.

use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

use pun_analyse::{Project, analyse_project};
use serde_json::{Value, json};

struct Inputs(PathBuf);

impl Inputs {
    fn new() -> Self {
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let root = std::env::temp_dir().join(format!(
            "pun-assignment-targets-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir_all(&root).unwrap();
        Self(root)
    }

    fn write(&self, name: &str, source: &str) -> PathBuf {
        let path = self.0.join(format!("{name}.php"));
        std::fs::write(&path, source).unwrap();
        path
    }
}

impl Drop for Inputs {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.0).unwrap();
    }
}

fn errors(project: &Project) -> Value {
    json!(
        analyse_project(project)
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
    )
}

#[test]
fn destructuring_target_roles_preserve_invalid_values_variable_writes_and_offset_reads() {
    let inputs: Value =
        serde_json::from_str(include_str!("fixtures/assignment-targets/inputs.json")).unwrap();
    let expected: Value =
        serde_json::from_str(include_str!("fixtures/assignment-targets/expected.json")).unwrap();
    let root = Inputs::new();
    let files = inputs
        .as_object()
        .unwrap()
        .iter()
        .map(|(name, source)| root.write(name, source.as_str().unwrap()))
        .collect();
    let mut project = Project::new(files, Vec::new());
    project.php_version = 80500;
    assert_eq!(errors(&project), expected["inputs"]);
}

#[test]
fn destructuring_nullsafe_checks_follow_item_roles_and_precede_other_assignment_errors() {
    let inputs: Value =
        serde_json::from_str(include_str!("fixtures/assignment-targets/nullsafe.json")).unwrap();
    let expected: Value =
        serde_json::from_str(include_str!("fixtures/assignment-targets/expected.json")).unwrap();
    let root = Inputs::new();
    let files = inputs
        .as_object()
        .unwrap()
        .iter()
        .map(|(name, source)| root.write(name, source.as_str().unwrap()))
        .collect();
    let mut project = Project::new(files, Vec::new());
    project.php_version = 80500;
    assert_eq!(errors(&project), expected["nullsafe"]);
}

#[test]
fn destructuring_target_roles_match_phpstan_at_strict_levels() {
    // Literal-array negative controls initialize their variables to distinguish assignability
    // from undefined-variable inference, which has a separate owner.
    let inputs: Value =
        serde_json::from_str(include_str!("fixtures/assignment-targets/levels.json")).unwrap();
    let expected: Value =
        serde_json::from_str(include_str!("fixtures/assignment-targets/expected.json")).unwrap();
    let root = Inputs::new();
    let files = inputs
        .as_object()
        .unwrap()
        .iter()
        .map(|(name, source)| root.write(name, source.as_str().unwrap()))
        .collect::<Vec<_>>();
    for level in [0, 1, 2, 7, 8, 9, 10] {
        let mut project = Project::new(files.clone(), Vec::new());
        project.php_version = 80500;
        project.level = level;
        // config.level1.neon enables this independently of the rule-registration level.
        project.options.set(pun_analyse::ProjectOptions::CHECK_MAYBE_UNDEFINED_VARIABLES, level >= 1);
        assert_eq!(
            errors(&project),
            expected["levels"][level.to_string()],
            "level {level}"
        );
    }
}
