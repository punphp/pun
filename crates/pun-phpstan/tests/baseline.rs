//! `--generate-baseline` output against PHPStan 2.2.7's own formatters (tests/fixtures/baseline.json, exported by
//! scripts/export-baseline-fixtures.php from phpstan-src's formatter tests plus extra cases), and reading the files
//! back as PHPStan does (Neon::decode / `require`).

mod common;

use common::{fixture, tagged};
use pun_phpstan::ignore::Error;
use pun_phpstan::{Value, baseline, neon, phpfile};

#[test]
fn phpstan_baseline_formatters() {
    let all = fixture("baseline.json");
    let dir = all["dir"].as_str().unwrap();
    let mut failures = Vec::new();
    for case in all["cases"].as_array().unwrap() {
        let errors: Vec<Error> = case["errors"]
            .as_array()
            .unwrap()
            .iter()
            .map(|e| Error {
                identifier: e["identifier"].as_str().map(Into::into),
                can_be_ignored: e["ignorable"].as_bool().unwrap(),
                ..Error::new(
                    e["message"].as_str().unwrap(),
                    e["file"].as_str().unwrap(),
                    None,
                )
            })
            .collect();
        let raw = case["raw"].as_bool().unwrap();
        let (got, read) = if case["format"] == "neon" {
            let out = baseline::neon(dir, &errors, case["existing"].as_str().unwrap(), raw);
            let read = neon::decode(&out).map_err(|e| e.to_string());
            (out, read)
        } else {
            let out = baseline::php(dir, &errors, case["has_errors"].as_bool().unwrap(), raw);
            let read = phpfile::load_config(&out, "{dir}").map(Value::Array);
            (out, read)
        };
        if got != case["output"].as_str().unwrap() {
            failures.push(format!("{case}\n  got {got:?}"));
        } else if read.as_ref().map(tagged).ok().as_ref() != Some(&case["read"]) {
            failures.push(format!("read back {case}\n  got {read:?}"));
        }
    }
    assert!(
        failures.is_empty(),
        "{} differ:\n{}",
        failures.len(),
        failures.join("\n")
    );
}
