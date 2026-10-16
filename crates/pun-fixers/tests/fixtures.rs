//! Runs PHP-CS-Fixer's own test cases (exported to crates/pun-harness/fixtures/<name>.json) against each ported fixer.
//! `PUN_FIXERS_ONLY=name cargo test -p punphp-fixers --test fixtures -- --nocapture`
use serde_json::Value;
use std::path::Path;

#[test]
fn upstream_fixtures() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../pun-harness/fixtures");
    let (mut pass, mut fail, mut skip) = (0, 0, 0);
    for mut fixer in pun_fixers::all() {
        let Ok(raw) = std::fs::read(dir.join(format!("{}.json", fixer.name().replace('/', "_")))) else { continue };
        let doc: Value = serde_json::from_slice(&raw).unwrap();
        let (mut p, mut f) = (0, 0);
        for test in doc["tests"].as_array().into_iter().flatten() {
            let method = test["method"].as_str().unwrap_or("");
            if !method.starts_with("testFix") { continue }
            for case in test["cases"].as_array().into_iter().flatten() {
                let args = case["args"].as_array().cloned().unwrap_or_default();
                let expected = args.first().and_then(Value::as_str).unwrap_or("");
                let input = args.get(1).and_then(Value::as_str).unwrap_or(expected);
                let config = args.get(2).cloned().filter(|c| !c.as_array().is_some_and(|a| a.is_empty())
                    && !c.as_object().is_some_and(|o| o.is_empty())).unwrap_or(Value::Null);
                if fixer.configure(&config).is_err() { skip += 1; continue }
                let mut tokens = pun_tokens::Tokens::from_code(input.as_bytes());
                if fixer.is_candidate(&tokens) { fixer.fix(Path::new("test.php"), &mut tokens); }
                tokens.clear_empty_tokens();
                let got = tokens.generate_code();
                if got == expected.as_bytes() { p += 1 } else {
                    f += 1;
                    if std::env::var("PUN_FIXTURE_VERBOSE").is_ok() {
                        eprintln!("--- {} case {} ---\nexpected:\n{expected}\ngot:\n{}", fixer.name(), case["key"], String::from_utf8_lossy(&got));
                    }
                }
            }
        }
        let _ = fixer.configure(&Value::Null);
        eprintln!("{:40} {p:4} pass {f:4} fail", fixer.name());
        pass += p; fail += f;
    }
    eprintln!("TOTAL {pass} pass, {fail} fail, {skip} skipped (unsupported config)");
    assert_eq!(fail, 0);
}
