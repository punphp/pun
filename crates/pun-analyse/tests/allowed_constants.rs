//! Shared FunctionCallParametersCheck constant policies, compared with the PHPStan 2.3.1 phar.

use std::path::Path;

use pun_analyse::{Project, reports, run};
use serde_json::Value;

fn check(file: &str, expected: &str, level: u8, version: u32) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/allowed-constants");
    let oracle: Value =
        serde_json::from_slice(&std::fs::read(root.join(expected)).unwrap()).unwrap();
    let mut project = Project::new(vec![root.join(file)], Vec::new());
    project.level = level;
    project.php_version = version;
    let mut got: Vec<_> = run(&project)
        .unwrap()
        .into_iter()
        .map(|e| (e.line, e.identifier, e.message, e.tip))
        .collect();
    let mut want: Vec<_> = oracle["errors"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|e| reports(e["identifier"].as_str().unwrap()))
        .map(|e| {
            (
                e["line"].as_i64().unwrap(),
                e["identifier"].as_str().unwrap().to_string(),
                e["message"].as_str().unwrap().to_string(),
                e["tip"].as_str().map(str::to_string),
            )
        })
        .collect();
    got.sort();
    want.sort();
    assert_eq!(got, want, "{file}: level {level}, PHP {version}");
}

#[test]
fn builtin_constant_identity_and_argument_binding_match_phpstan() {
    check("calls.php", "expected-phpstan.json", 5, 80500);
}

#[test]
fn constant_policies_start_at_level_five_and_php_eight() {
    check(
        "boundaries.php",
        "expected-phpstan-boundaries.json",
        5,
        80000,
    );
    check("boundaries.php", "expected-phpstan-level4.json", 4, 80000);
    check("boundaries.php", "expected-phpstan-php74.json", 5, 70400);
}

#[test]
fn upstream_constant_parameter_fixtures_are_strict_without_gaps() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    let data = root.join("rules/data");
    let prefix = format!("{}/", data.display());
    let identifiers = [
        "argument.invalidConstant",
        "argument.exclusiveConstants",
        "argument.bitmaskNotAllowed",
    ];
    let mut checked = 0;
    for level in [5, 9] {
        for suite in [
            "Functions/CallToFunctionParametersRuleTest",
            "Methods/CallMethodsRuleTest",
            "Functions/CallUserFuncRuleTest",
        ] {
            // CallUserFuncRule has only a level-five export.
            if level == 9 && suite == "Functions/CallUserFuncRuleTest" {
                continue;
            }
            let oracle: Value = serde_json::from_slice(
                &std::fs::read(root.join(format!("levels/{level}/{suite}.json"))).unwrap(),
            )
            .unwrap();
            for case in oracle["cases"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|c| c["method"].as_str().unwrap().contains("ConstantParameter"))
            {
                checked += 1;
                let paths = |key: &str| {
                    case[key]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|p| data.join(p.as_str().unwrap()))
                        .collect()
                };
                let mut project = Project::new(paths("files"), paths("scan"));
                project.level = level;
                let mut got: Vec<_> = run(&project)
                    .unwrap()
                    .into_iter()
                    .filter(|e| identifiers.contains(&e.identifier.as_str()))
                    .map(|e| {
                        (
                            e.file.strip_prefix(&prefix).unwrap().to_string(),
                            e.line,
                            e.identifier,
                            e.message,
                        )
                    })
                    .collect();
                let mut want: Vec<_> = case["errors"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .filter(|e| identifiers.contains(&e["identifier"].as_str().unwrap()))
                    .map(|e| {
                        (
                            e["file"].as_str().unwrap().to_string(),
                            e["line"].as_i64().unwrap(),
                            e["identifier"].as_str().unwrap().to_string(),
                            e["message"].as_str().unwrap().to_string(),
                        )
                    })
                    .collect();
                got.sort();
                want.sort();
                assert_eq!(got, want, "level {level}: {suite}::{}", case["method"]);
            }
        }
    }
    assert_eq!(checked, 5);
}
