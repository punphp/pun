//! `lint` against real PHP: every case in pun-tokens' tests/fixtures/lint_cases.json (scripts/export-lint-cases.py)
//! carries what `token_get_all($code, TOKEN_PARSE)` threw, as PHP-CS-Fixer's TokenizerLintingResult words it.

#[test]
fn lint_matches_token_get_all() {
    let cases: Vec<serde_json::Value> = serde_json::from_str(include_str!("../../pun-tokens/tests/fixtures/lint_cases.json")).unwrap();
    let mut failures = Vec::new();
    for case in &cases {
        let code = case["code"].as_str().unwrap();
        let got = punphp_format::lint(code.as_bytes()).err().map(|e| e.message);
        if got.as_deref() != case["error"].as_str() {
            failures.push(format!("{code:?}\n  php: {}\n  pun: {got:?}", case["error"]));
        }
    }
    assert!(failures.is_empty(), "{} of {} differ:\n{}", failures.len(), cases.len(), failures.join("\n"));
}
