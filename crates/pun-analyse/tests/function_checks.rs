//! PrintfHelper edge cases through the public analyser. Expectations come from PHPStan 2.3.1 at level 0,
//! with phpVersion 70400 and 80511: positional/star placeholders, scanf validation and array-size unions.

use std::path::Path;

use pun_analyse::{Project, run};

const IDENTIFIERS: &[&str] = &[
    "argument.printf",
    "argument.sprintf",
    "argument.sscanf",
    "argument.fscanf",
    "argument.vprintf",
    "argument.vsprintf",
];

#[test]
fn printf_parameters_match_phpstan() {
    type Diagnostic = (i64, String, String, Option<String>);
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    for fixture in ["function_checks", "function_pipe"] {
        let expected: std::collections::BTreeMap<u32, Vec<Diagnostic>> =
            serde_json::from_slice(&std::fs::read(root.join(format!("{fixture}.json"))).unwrap())
                .unwrap();
        for (php_version, mut want) in expected {
            let mut project = Project::new(vec![root.join(format!("{fixture}.php"))], Vec::new());
            project.php_version = php_version;
            let mut got: Vec<_> = run(&project)
                .unwrap()
                .into_iter()
                .filter(|e| IDENTIFIERS.contains(&e.identifier.as_str()))
                .map(|e| (e.line, e.identifier, e.message, e.tip))
                .collect();
            want.sort();
            got.sort();
            assert_eq!(got, want, "{fixture}, phpVersion {php_version}");
        }
    }
}
