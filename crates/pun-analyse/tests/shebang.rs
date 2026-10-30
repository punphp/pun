//! Leading inline text must not hide PHP code from parser admission or reflection. Complete expectations come
//! from PHPStan 2.3.1's public Analyser via fixtures/parser/capture-shebang.php, not its severe-only CLI output.

use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

use pun_analyse::{Project, analyse_project};
use serde_json::{Value, json};

struct Inputs(PathBuf);

impl Inputs {
    fn new() -> Self {
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let path = std::env::temp_dir().join(format!(
            "pun-shebang-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir_all(&path).unwrap();
        Self(path)
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
            .map(|error| json!({
                "file": std::path::Path::new(&error.file).file_name().unwrap().to_str().unwrap(),
                "line": error.line, "identifier": error.identifier, "message": error.message,
                "tip": error.tip, "ignorable": error.can_be_ignored,
            }))
            .collect::<Vec<_>>()
    )
}

#[test]
fn shebang_preserves_parser_failures_and_executable_diagnostics() {
    let inputs: Value =
        serde_json::from_str(include_str!("fixtures/parser/shebang-inputs.json")).unwrap();
    let expected: Value =
        serde_json::from_str(include_str!("fixtures/parser/shebang-expected.json")).unwrap();
    let mut actual = serde_json::Map::new();
    for (name, source) in inputs.as_object().unwrap() {
        let root = Inputs::new();
        let file = root.write("input", source.as_str().unwrap());
        actual.insert(name.clone(), errors(&Project::new(vec![file], Vec::new())));
    }
    assert_eq!(Value::Object(actual), expected["analysed"]);
}

#[test]
fn shebang_restores_valid_scanned_symbols_without_admitting_invalid_declarations() {
    let inputs: Value =
        serde_json::from_str(include_str!("fixtures/parser/shebang-scans.json")).unwrap();
    let expected: Value =
        serde_json::from_str(include_str!("fixtures/parser/shebang-expected.json")).unwrap();
    let mut actual = serde_json::Map::new();
    for (name, source) in inputs.as_object().unwrap() {
        let root = Inputs::new();
        let main = root.write("main", include_str!("fixtures/parser/scan-main.php"));
        let symbols = root.write("symbols", source.as_str().unwrap());
        let mut project = Project::new(vec![main], vec![symbols]);
        project.level = 2;
        actual.insert(name.clone(), errors(&project));
    }
    assert_eq!(Value::Object(actual), expected["scanned"]);
}
