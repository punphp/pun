//! nette/neon's own test suite (tests/fixtures/neon.json, exported by scripts/export-neon-fixtures.php with the real
//! nette/neon v3.3.4 results) plus extra inputs for branches it doesn't reach (tests/fixtures/neon-extra.json, same
//! exporter).

mod common;

use common::{fixture, tagged, untagged};
use pun_phpstan::neon;

fn run(name: &str) {
    let mut failures = Vec::new();
    let cases = fixture(name);
    for case in cases.as_array().unwrap() {
        if let Some(input) = case.get("input").or(case.get("input_base64")) {
            let bytes = match case.get("input") {
                Some(s) => s.as_str().unwrap().as_bytes().to_vec(),
                None => base64(input.as_str().unwrap()),
            };
            let got = match neon::decode_bytes(&bytes) {
                Ok(v) => serde_json::json!({ "value": tagged(&v) }),
                Err(e) => serde_json::json!({ "error": e.0 }),
            };
            let want = match case.get("error") {
                Some(e) => serde_json::json!({ "error": e }),
                None => serde_json::json!({ "value": case["value"] }),
            };
            if got != want {
                failures.push(format!("decode {input}\n  want {want}\n  got  {got}"));
            }
        } else if case.get("output").is_some() && case["encode"].get("object").is_none() {
            // Objects (stdClass) never occur in PHPStan configs; `Value` has no object, so those encoder cases are skipped.
            let v = untagged(&case["encode"]);
            let got = neon::encode(
                &v,
                case["block"].as_bool().unwrap(),
                case["indentation"].as_str().unwrap(),
            );
            if got != case["output"].as_str().unwrap() {
                failures.push(format!(
                    "encode {}\n  want {:?}\n  got  {got:?}",
                    case["encode"], case["output"]
                ));
            }
        }
    }
    assert!(
        failures.is_empty(),
        "{} of {} cases differ:\n{}",
        failures.len(),
        cases.as_array().unwrap().len(),
        failures.join("\n")
    );
}

fn base64(s: &str) -> Vec<u8> {
    let alphabet = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = Vec::new();
    let mut buf = 0u32;
    let mut bits = 0;
    for c in s.bytes().filter(|&c| c != b'=') {
        buf = buf << 6 | alphabet.iter().position(|&a| a == c).unwrap() as u32;
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            out.push((buf >> bits) as u8);
        }
    }
    out
}

#[test]
fn nette_neon_test_suite() {
    run("neon.json");
}

#[test]
fn extra_inputs() {
    run("neon-extra.json");
}
