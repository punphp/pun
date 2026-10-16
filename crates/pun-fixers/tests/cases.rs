//! Extra cases for branches the upstream fixtures miss: tests/cases/<fixer>.json is a list of
//! `{"fixer", "config"?, "input", "expected"}` where `expected` is the real PHP-CS-Fixer's output for
//! `input` (fixer alone, default whitespace config; scripts/oracle-case.php). Same checks as AbstractFixerTestCase::doTest.
//! `{"fixer", "config", "error"}` cases (scripts/oracle-config-error.php) pin the message of an invalid configuration.
extern crate punphp_fixers as pun_fixers;
use serde_json::Value;
use std::path::Path;

#[test]
fn oracle_cases() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/cases");
    let mut fail = 0;
    for name in pun_fixers::all().iter().map(|f| f.name()) {
        let Ok(raw) = std::fs::read(dir.join(format!("{}.json", name.replace('/', "_")))) else { continue };
        let cases: Vec<Value> = serde_json::from_slice(&raw).unwrap();
        let mut p = 0;
        for (i, case) in cases.iter().enumerate() {
            let mut fixer = pun_fixers::all().into_iter().find(|f| f.name() == name).unwrap();
            let config = case.get("config").cloned().unwrap_or(Value::Null);
            if let Some(error) = case.get("error") {
                // An invalid configuration: configure must fail with the oracle's message.
                match fixer.configure(&config) {
                    Err(e) if e.to_string() == error.as_str().unwrap() => p += 1,
                    got => {
                        fail += 1;
                        eprintln!("--- {name} case {i}: configure({config})\nexpected error: {error}\ngot: {:?}", got.err());
                    }
                }
                continue;
            }
            fixer.configure(&config).unwrap_or_else(|e| panic!("{name} case {i}: {e}"));
            let input = case["input"].as_str().unwrap();
            let expected = case["expected"].as_str().unwrap();
            let file = Path::new("test.php");
            let mut problems = Vec::new();
            let mut tokens = pun_tokens::Tokens::from_code(input.as_bytes());
            if tokens.count() > 0 && fixer.is_candidate(&tokens) {
                fixer.fix(file, &mut tokens);
            }
            let got = tokens.generate_code();
            if got != expected.as_bytes() {
                problems.push(format!("got:\n{}", String::from_utf8_lossy(&got)));
            }
            if input != expected && !tokens.is_changed() {
                problems.push("tokens not marked changed".into());
            }
            let mut tokens = pun_tokens::Tokens::from_code(expected.as_bytes());
            if tokens.count() > 0 && fixer.is_candidate(&tokens) {
                fixer.fix(file, &mut tokens);
            }
            if tokens.generate_code() != expected.as_bytes() {
                problems.push("expected code changed".into());
            }
            if problems.is_empty() {
                p += 1;
            } else {
                fail += 1;
                eprintln!("--- {name} case {i} ---\ninput:\n{input}\nexpected:\n{expected}\n{}", problems.join("\n"));
            }
        }
        eprintln!("{name:40} {p:4} oracle cases pass");
    }
    assert_eq!(fail, 0);
}

/// priority() and is_risky() of every ported fixer against the real ones (tests/metadata.json, from
/// scripts/fixer-metadata.php): a wrong priority reorders fixers and changes output.
#[test]
fn oracle_metadata() {
    let raw = std::fs::read(Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/metadata.json")).unwrap();
    let meta: Value = serde_json::from_slice(&raw).unwrap();
    let mut bad = Vec::new();
    // Pun/* are pun's own fixers: no upstream to compare with.
    for f in pun_fixers::all().into_iter().filter(|f| !f.name().starts_with("Pun/")) {
        let m = &meta[f.name()];
        if m.is_null() {
            bad.push(format!("{}: not a Pint fixer", f.name()));
        } else if m["priority"] != f.priority() || m["risky"] != f.is_risky() {
            bad.push(format!("{}: priority {} risky {}, upstream {}", f.name(), f.priority(), f.is_risky(), m));
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}
