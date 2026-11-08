//! Types a method call binds to its receiver, and closure parameters the called method's callable types: complete
//! PHPStan 2.3.1 diagnostics at level max for tests/fixtures/called-on-types/calls.php.
//!
//! - CalledOnTypeUnresolvedMethodPrototypeReflection: `static` and `$this` in a parameter's PHPDoc (also nested, and
//!   a trait method's) are the object a method is called on, and stay late on `$this`.
//! - A closure parameter declared with a generic class without type arguments is that plain class.
//! - MutatingScope::enterAnonymousFunctionWithoutReflection(): a declared closure parameter takes the callable's
//!   narrower parameter type (`intersectButNotNever()`).
//!
//! Expected: tests/fixtures/called-on-types/expected-phpstan.json, `phpstan analyse --level max` of calls.php.

use std::path::Path;

use pun_analyse::{Project, run};
use serde_json::Value;

#[test]
fn called_on_types() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/called-on-types");
    let oracle: Value = serde_json::from_str(include_str!(
        "fixtures/called-on-types/expected-phpstan.json"
    ))
    .unwrap();
    let mut project = Project::new(vec![root.join("calls.php")], Vec::new());
    project.level = 10;
    let mut got: Vec<_> = run(&project)
        .unwrap()
        .into_iter()
        .map(|e| (e.line, e.identifier, e.message, e.tip))
        .collect();
    let mut want: Vec<_> = oracle
        .as_array()
        .unwrap()
        .iter()
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
    assert_eq!(got, want);
}
