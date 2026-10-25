//! MethodCallableRule and StaticMethodCallableRule diagnostics from PHPStan 2.3.1.

use std::path::Path;

use pun_analyse::{Project, run};
use pun_phpstan::config::{Options, resolve};

type Diagnostic = (String, i64, String, String, Option<String>, bool);

fn member_error(identifier: &str, message: &str) -> bool {
    (identifier.starts_with("method.") || identifier.starts_with("staticMethod."))
        && !identifier.ends_with(".resultUnused")
        && !identifier.ends_with(".unused")
        || identifier.starts_with("outOfClass.")
        || matches!(identifier, "argument.type" | "arguments.count")
        || identifier == "class.noParent"
        || identifier == "callable.notSupported"
        || identifier == "class.notFound" && message.starts_with("Call to ")
}

#[test]
fn method_callable_members_match_phpstan() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/method-callable");
    let expected: std::collections::BTreeMap<String, Vec<Diagnostic>> =
        serde_json::from_slice(&std::fs::read(root.join("expected.json")).unwrap()).unwrap();
    for (case, mut want) in expected {
        let config = resolve(&Options {
            config: Some(
                root.join(format!("{case}.neon"))
                    .to_string_lossy()
                    .into_owned(),
            ),
            ..Options::for_project(root.to_str().unwrap())
        })
        .unwrap();
        let project = Project::from_config(&root, &config.parameters);
        let mut got: Vec<_> = run(&project)
            .unwrap()
            .into_iter()
            .filter(|e| member_error(&e.identifier, &e.message))
            .map(|e| {
                (
                    Path::new(&e.file)
                        .file_name()
                        .unwrap()
                        .to_string_lossy()
                        .into_owned(),
                    e.line,
                    e.identifier,
                    e.message,
                    e.tip,
                    e.can_be_ignored,
                )
            })
            .collect();
        got.sort();
        want.sort();
        assert_eq!(got, want, "{case}");
    }
}

/// PHPStan 2.3.1 crashes on PHP 8.5 bound arguments. Its virtual callable nodes are emitted only for a single
/// ellipsis; pipe callables are rewritten to ordinary calls instead (NodeScopeResolver::processExprNode()).
#[test]
fn partial_applications_and_pipes_do_not_enter_member_callable_rules() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/method-callable");
    let mut project = Project::new(vec![root.join("partial.php")], Vec::new());
    project.php_version = 80500;
    project.level = 7;
    let errors: Vec<_> = run(&project)
        .unwrap()
        .into_iter()
        .filter(|e| member_error(&e.identifier, &e.message))
        .collect();
    assert_eq!(errors, []);
}
