//! parametersSchema building and validation against PHPStan 2.2.7's ContainerFactory::validateParameters()
//! (tests/fixtures/schema.json, scripts/export-schema-fixtures.php).

mod common;

use common::fixture;
use pun_phpstan::config::ProjectFiles;
use pun_phpstan::{Value, di, schema};
use serde_json::json;

#[test]
fn container_factory_validate_parameters() {
    let dir = std::env::temp_dir().join(format!("pun-phpstan-schema-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let file = dir.join("schema.neon").to_string_lossy().into_owned();
    let files = ProjectFiles::new("/pun-root/phpstan.phar");
    let mut failures = Vec::new();
    for case in fixture("schema.json").as_array().unwrap() {
        std::fs::write(&file, case["neon"].as_str().unwrap()).unwrap();
        let data = di::load_neon(&files, &file, &[]).unwrap();
        let mut params = data
            .get_str("parameters")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();
        params.set("__validate".into(), Value::Bool(true));
        let section = data
            .get_str("parametersSchema")
            .and_then(Value::as_array)
            .cloned()
            .unwrap();
        let got = match schema::parameters_schema(&section) {
            Ok(s) => json!({ "messages": s.validate(&Value::Array(params), &["parameters"]) }),
            Err(e) => json!({ "exception": e }),
        };
        let want = match case.get("exception").and_then(|e| e.as_str()) {
            // PHP's own TypeError/ArgumentCountError (with the phar's path in it): only "fails" is comparable.
            Some(e) if e.contains(" called in ") || e.contains(" passed in ") => {
                if got.get("exception").is_some() {
                    continue;
                }
                json!({ "exception": e })
            }
            Some(e) => json!({ "exception": e }),
            None => json!({ "messages": case["messages"] }),
        };
        if got != want {
            failures.push(format!("{}\n  want {want}\n  got  {got}", case["neon"]));
        }
    }
    std::fs::remove_dir_all(&dir).unwrap();
    assert!(
        failures.is_empty(),
        "{} differ:\n{}",
        failures.len(),
        failures.join("\n")
    );
}
