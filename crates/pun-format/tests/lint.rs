//! `lint` against real PHP: every case in pun-tokens' tests/fixtures/lint_cases.json (scripts/export-lint-cases.py)
//! carries what `token_get_all($code, TOKEN_PARSE)` threw, as PHP-CS-Fixer's TokenizerLintingResult words it.

#[test]
fn lint_matches_token_get_all() {
    let cases: Vec<serde_json::Value> = serde_json::from_str(include_str!(
        "../../pun-tokens/tests/fixtures/lint_cases.json"
    ))
    .unwrap();
    let mut failures = Vec::new();
    for case in &cases {
        let code = case["code"].as_str().unwrap();
        let got = punphp_format::lint(code.as_bytes())
            .err()
            .map(|e| e.message);
        if got.as_deref() != case["error"].as_str().map(str::as_bytes) {
            failures.push(format!(
                "{code:?}\n  php: {}\n  pun: {:?}",
                case["error"],
                got.map(|m| String::from_utf8_lossy(&m).into_owned())
            ));
        }
    }
    assert!(
        failures.is_empty(),
        "{} of {} differ:\n{}",
        failures.len(),
        cases.len(),
        failures.join("\n")
    );
}

/// The unexpected token's text keeps the source's bytes: `%.*s` cuts `é` after its first byte, and Latin-1 stays
/// Latin-1. Expected: `token_get_all($code, TOKEN_PARSE)` on PHP 8.5, worded by TokenizerLintingResult.
#[test]
fn lint_keeps_the_source_bytes() {
    for (code, want) in [
        (
            &b"<?php\n$x = 1 'aaaaaaaaaaaaaaaaaaaaaaaaaaaaa\xc3\xa9 bbbbbbbbbbbbb';\n"[..],
            &b"Parse error: syntax error, unexpected single-quoted string \"aaaaaaaaaaaaaaaaaaaaaaaaaaaaa\xc3...\" on line 2."[..],
        ),
        (
            b"<?php\n$x = 1 'caf\xe9';\n",
            b"Parse error: syntax error, unexpected single-quoted string \"caf\xe9\" on line 2.",
        ),
    ] {
        assert_eq!(punphp_format::lint(code).unwrap_err().message, want);
    }
}
