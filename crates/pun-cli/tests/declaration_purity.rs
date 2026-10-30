//! Canonical rule configuration controls the supported `void.pure` slice even while the complete
//! PureFunctionRule/PureMethodRule mappings remain unported.
mod common;

use common::{PUN, clean_env, project};
use serde_json::Value;
use std::process::Command;

#[test]
fn canonical_declaration_purity_configuration_controls_runtime_reporting() {
    let dir = project(
        "declaration-purity",
        "configuration",
        &[(
            "matrix.php",
            include_str!("../../pun-analyse/tests/fixtures/declaration-purity/matrix.php"),
        )],
    );
    let capture: Value = serde_json::from_str(include_str!(
        "../../pun-analyse/tests/fixtures/declaration-purity/expected.json"
    ))
    .unwrap();
    let expected: Vec<_> = capture["2"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|e| e["file"] == "matrix.php" && e["identifier"] == "void.pure")
        .map(|e| (e["line"].clone(), e["message"].clone()))
        .collect();
    for (preset, rule, enabled) in [
        ("basic", "", false),
        ("essential", "", true),
        (
            "essential",
            "[analyse.rules]\n\"analyse/functions/invalid-pure\" = \"off\"\n",
            false,
        ),
        (
            "basic",
            "[analyse.rules]\n\"analyse/functions/invalid-pure\" = \"on\"\n",
            true,
        ),
    ] {
        std::fs::write(
            dir.join("pun.toml"),
            format!("[php]\nversion = \"8.5\"\n[analyse]\npreset = \"{preset}\"\npaths = [\"matrix.php\"]\n{rule}"),
        )
        .unwrap();
        let output = clean_env(&mut Command::new(PUN), &["PATH", "HOME"])
            .current_dir(&dir)
            .env("PUN_AGENT", "0")
            .env("RAYON_NUM_THREADS", "2")
            .args(["analyse", "--format=json"])
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(1), "{output:?}");
        assert!(output.stderr.is_empty(), "{output:?}");
        let result: Value = serde_json::from_slice(&output.stdout).unwrap();
        let actual: Vec<_> = result["files"]
            .as_object()
            .unwrap()
            .values()
            .flat_map(|file| file["messages"].as_array().unwrap())
            .filter(|e| e["identifier"] == "void.pure")
            .map(|e| (e["line"].clone(), e["message"].clone()))
            .collect();
        let want = if enabled { expected.as_slice() } else { &[] };
        assert_eq!(actual, want, "preset {preset}, {rule}");
    }
    std::fs::remove_dir_all(dir).unwrap();
}
