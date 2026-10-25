//! The maintained engine retains Mago 1.57.0's primary issues and PHPStan's failed-read cascade policy.
//! expected.json is Mago's `mago analyze issues.php --reporting-format json` (primary span, 1-based).
//! expected-phpstan.json is PHPStan 2.3.1's messages from `analyse --debug --no-progress --error-format=json
//! --level=9 issues.php`: the undefined echo input has no secondary echo.nonString error.
//! debug-panic.php, analysed first, was written to make Mago's analyser panic in a debug build (the run skips such a
//! file and goes on); it no longer panics and is kept as a regression input.

use std::path::Path;

use pun_analyse::{Project, run_mago};
use serde_json::Value;

#[test]
fn issues_match_upstream_oracles() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/mago");
    let file = dir.join("issues.php");
    let analysed = vec![
        dir.join("debug-panic.php"),
        file.clone(),
        dir.join("missing.php"),
    ];
    let run = run_mago(&Project::new(analysed, Vec::new()), None).unwrap();
    let got: Vec<_> = run
        .issues
        .iter()
        .filter(|i| i.file != dir.join("debug-panic.php").display().to_string())
        .map(|i| (i.file.clone(), i.line, i.code.clone(), i.message.clone()))
        .collect();
    let expected: Value =
        serde_json::from_slice(&std::fs::read(dir.join("expected.json")).unwrap()).unwrap();
    let phpstan: Value =
        serde_json::from_slice(&std::fs::read(dir.join("expected-phpstan.json")).unwrap()).unwrap();
    let phpstan = phpstan.as_array().unwrap();
    let want: Vec<_> = expected
        .as_array()
        .unwrap()
        .iter()
        // Preserve the original Mago capture. Only its failed-read echo cascade differs intentionally;
        // the independent PHPStan capture retains the primary error and rejects a second type error.
        .filter(|e| {
            e["code"] != "mixed-argument"
                || !phpstan
                    .iter()
                    .any(|p| p["line"] == e["line"] && p["identifier"] == "variable.undefined")
                || phpstan
                    .iter()
                    .any(|p| p["line"] == e["line"] && p["identifier"] == "echo.nonString")
        })
        .map(|e| {
            let line = e["line"].as_u64().unwrap() as u32;
            let (code, message) = (e["code"].as_str().unwrap(), e["message"].as_str().unwrap());
            (
                file.display().to_string(),
                line,
                code.to_string(),
                message.to_string(),
            )
        })
        .collect();
    assert_eq!(got, want);
}
