//! php-parser's name-resolution failure boundary, captured from PHPStan 2.3.1's public Analyser by
//! fixtures/parser/capture.php. The CLI's severe-only presentation would hide the neighboring valid files.

use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

use pun_analyse::{Project, analyse_project};
use serde_json::{Value, json};

struct Inputs(PathBuf);

impl Inputs {
    fn new() -> Self {
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let root = std::env::temp_dir().join(format!(
            "pun-parser-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir_all(&root).unwrap();
        Self(root)
    }

    fn write(&self, name: &str, code: &str) -> PathBuf {
        self.write_source(name, &format!("<?php\n{code}\n"))
    }

    fn write_source(&self, name: &str, source: &str) -> PathBuf {
        let path = self.0.join(format!("{name}.php"));
        std::fs::write(&path, source).unwrap();
        path
    }
}

impl Drop for Inputs {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.0).unwrap();
    }
}

fn errors(project: &Project) -> Value {
    Value::Array(
        analyse_project(project)
            .unwrap()
            .errors
            .into_iter()
            .map(|e| {
                json!({
                    "file": std::path::Path::new(&e.file).file_name().unwrap().to_str().unwrap(),
                    "line": e.line, "identifier": e.identifier, "message": e.message,
                    "tip": e.tip, "ignorable": e.can_be_ignored,
                })
            })
            .collect(),
    )
}

#[test]
fn name_roles_parser_precedence_and_resolver_visit_order() {
    let inputs: Value = serde_json::from_str(include_str!("fixtures/parser/inputs.json")).unwrap();
    let root = Inputs::new();
    let files = inputs
        .as_object()
        .unwrap()
        .iter()
        .map(|(name, code)| root.write(name, code.as_str().unwrap()))
        .collect();
    let expected: Value =
        serde_json::from_str(include_str!("fixtures/parser/expected.json")).unwrap();
    let project = Project::new(files, Vec::new());
    assert_eq!(errors(&project), expected);
}

#[test]
fn invalid_scanned_files_export_no_classes_functions_or_constants() {
    let inputs: Value = serde_json::from_str(include_str!("fixtures/parser/scans.json")).unwrap();
    let expected: Value =
        serde_json::from_str(include_str!("fixtures/parser/expected-scans.json")).unwrap();
    for (name, code) in inputs.as_object().unwrap() {
        let root = Inputs::new();
        let main = root.write(
            "main",
            include_str!("fixtures/parser/scan-main.php")
                .strip_prefix("<?php\n")
                .unwrap(),
        );
        let symbols = root.write("symbols", code.as_str().unwrap());
        assert_eq!(
            errors(&Project::new(vec![main], vec![symbols])),
            expected[name],
            "{name}"
        );
    }
}

#[test]
fn parser_errors_survive_rule_filters_and_configured_ignores() {
    use pun_phpstan::{Array, Key, Value as Php};
    let root = Inputs::new();
    let invalid = root.write(
        "attribute",
        "#[\\self] class Invalid {} // @phpstan-ignore phpstan.parse\n// @phpstan-ignore",
    );
    let valid = root.write("valid", "function duplicate($x, $x) {} missing_control();");
    let mut project = Project::new(vec![invalid, valid], Vec::new());
    project.reported = Some(std::sync::Arc::new(|_| false));
    project.report_ignores_without_comments = true;
    let mut entry = Array::new();
    entry.set(Key::from("identifier"), Php::str("phpstan.parse"));
    project.ignore_errors = pun_phpstan::ignore::IgnoredErrors::new(
        &Array::list([Php::Array(entry), Php::str("#^NoSuchError$#")]),
        true,
    );
    let analysis = analyse_project(&project).unwrap();
    assert!(pun_analyse::reports("phpstan.parse"));
    assert_eq!(analysis.errors.len(), 1);
    let error = &analysis.errors[0];
    assert_eq!(
        (error.identifier.as_str(), error.line, error.can_be_ignored),
        ("phpstan.parse", 2, false)
    );
    assert_eq!(
        analysis.general,
        [
            "Error message \"'\\self' is an invalid class name on line 2\" cannot be ignored, use excludePaths instead.",
            "Ignored error pattern #^NoSuchError$# was not matched in reported errors."
        ]
    );
}

#[test]
fn recovered_grammar_does_not_fabricate_a_name_resolution_failure() {
    // PHPStan 2.3.1 --level=0 --error-format=raw grammar.php reports only:
    // Syntax error, unexpected EOF, expecting T_VARIABLE on line 4 [identifier=phpstan.parse]
    // Grammar-message translation is not implemented. In particular, do not misreport the earlier \self
    // instead: RichParser never starts its NameResolver after the collecting parser has failed.
    let root = Inputs::new();
    let file = root.write(
        "grammar",
        include_str!("fixtures/parser/grammar.php")
            .strip_prefix("<?php\n")
            .unwrap(),
    );
    let analysis = analyse_project(&Project::new(vec![file], Vec::new())).unwrap();
    assert!(
        analysis
            .errors
            .iter()
            .all(|e| e.identifier != "phpstan.parse")
    );
}

#[test]
fn member_modifier_failures_precede_name_resolution_and_rule_callbacks() {
    let inputs: Value =
        serde_json::from_str(include_str!("fixtures/parser/modifier-inputs.json")).unwrap();
    let expected: Value =
        serde_json::from_str(include_str!("fixtures/parser/modifier-expected.json")).unwrap();
    let root = Inputs::new();
    let files = inputs
        .as_object()
        .unwrap()
        .iter()
        .map(|(name, code)| {
            let namespace = name.replace('-', "_");
            root.write(
                name,
                &format!(
                    "namespace Modifier_{namespace};\n{}",
                    code.as_str().unwrap()
                ),
            )
        })
        .collect();
    assert_eq!(
        errors(&Project::new(files, Vec::new())),
        expected["analysed"]
    );
}

#[test]
fn original_abstract_final_property_hook_parser_failures() {
    let root =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/rules/data/Properties/data");
    let files = [
        "abstract-final-property-hook-parse-error.php",
        "final-property-hooks.php",
    ]
    .iter()
    .map(|name| root.join(name))
    .collect();
    let expected: Value =
        serde_json::from_str(include_str!("fixtures/parser/modifier-expected.json")).unwrap();
    assert_eq!(
        errors(&Project::new(files, Vec::new())),
        expected["original"]
    );
}

#[test]
fn invalid_member_modifiers_in_scanned_files_export_no_symbols() {
    let inputs: Value =
        serde_json::from_str(include_str!("fixtures/parser/modifier-scans.json")).unwrap();
    let expected: Value =
        serde_json::from_str(include_str!("fixtures/parser/modifier-expected.json")).unwrap();
    for (name, code) in inputs.as_object().unwrap() {
        let root = Inputs::new();
        let main = root.write(
            "main",
            include_str!("fixtures/parser/scan-main.php")
                .strip_prefix("<?php\n")
                .unwrap(),
        );
        let symbols = root.write("symbols", code.as_str().unwrap());
        assert_eq!(
            errors(&Project::new(vec![main], vec![symbols])),
            expected["scanned"][name],
            "{name}"
        );
    }
}

#[test]
fn parameter_parser_failures_precede_name_resolution_and_preserve_type_controls() {
    let inputs: Value =
        serde_json::from_str(include_str!("fixtures/parser/parameter-inputs.json")).unwrap();
    let expected: Value =
        serde_json::from_str(include_str!("fixtures/parser/parameter-expected.json")).unwrap();
    let root = Inputs::new();
    let files = inputs
        .as_object()
        .unwrap()
        .iter()
        .map(|(name, code)| {
            let namespace = name.replace('-', "_");
            root.write(
                name,
                &format!(
                    "namespace Parameter_{namespace};\n{}",
                    code.as_str().unwrap()
                ),
            )
        })
        .collect();
    assert_eq!(
        errors(&Project::new(files, Vec::new())),
        expected["analysed"]
    );
}

#[test]
fn original_void_parameter_typehint_is_a_non_ignorable_parser_error() {
    let file = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/rules/data/Methods/data/void-parameter-typehint.php");
    let expected: Value =
        serde_json::from_str(include_str!("fixtures/parser/parameter-expected.json")).unwrap();
    assert_eq!(
        errors(&Project::new(vec![file], Vec::new())),
        expected["original"]
    );
}

#[test]
fn original_function_and_closure_void_parameters_are_parser_errors() {
    let file = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/rules/data/Functions/data/void-parameter-typehint.php");
    let expected: Value =
        serde_json::from_str(include_str!("fixtures/parser/parameter-expected.json")).unwrap();
    assert_eq!(
        errors(&Project::new(vec![file], Vec::new())),
        expected["original-functions"]
    );
}

#[test]
fn invalid_parameters_in_scanned_files_export_no_symbols() {
    let inputs: Value =
        serde_json::from_str(include_str!("fixtures/parser/parameter-scans.json")).unwrap();
    let expected: Value =
        serde_json::from_str(include_str!("fixtures/parser/parameter-expected.json")).unwrap();
    for (name, code) in inputs.as_object().unwrap() {
        let root = Inputs::new();
        let main = root.write(
            "main",
            include_str!("fixtures/parser/scan-main.php")
                .strip_prefix("<?php\n")
                .unwrap(),
        );
        let symbols = root.write("symbols", code.as_str().unwrap());
        assert_eq!(
            errors(&Project::new(vec![main], vec![symbols])),
            expected["scanned"][name],
            "{name}"
        );
    }
}

#[test]
fn class_modifier_failures_preserve_priority_token_lines_and_parser_precedence() {
    let inputs: Value =
        serde_json::from_str(include_str!("fixtures/parser/class-modifier-inputs.json")).unwrap();
    let expected: Value =
        serde_json::from_str(include_str!("fixtures/parser/class-modifier-expected.json")).unwrap();
    let root = Inputs::new();
    let files = inputs
        .as_object()
        .unwrap()
        .iter()
        .map(|(name, code)| {
            let namespace = name.replace('-', "_");
            root.write(
                name,
                &format!(
                    "namespace ClassModifier_{namespace};\n{}",
                    code.as_str().unwrap()
                ),
            )
        })
        .collect();
    assert_eq!(
        errors(&Project::new(files, Vec::new())),
        expected["analysed"]
    );
}

#[test]
fn invalid_class_modifiers_in_scanned_files_export_no_symbols() {
    let inputs: Value =
        serde_json::from_str(include_str!("fixtures/parser/class-modifier-scans.json")).unwrap();
    let expected: Value =
        serde_json::from_str(include_str!("fixtures/parser/class-modifier-expected.json")).unwrap();
    for (name, code) in inputs.as_object().unwrap() {
        let root = Inputs::new();
        let main = root.write(
            "main",
            include_str!("fixtures/parser/scan-main.php")
                .strip_prefix("<?php\n")
                .unwrap(),
        );
        let symbols = root.write("symbols", code.as_str().unwrap());
        assert_eq!(
            errors(&Project::new(vec![main], vec![symbols])),
            expected["scanned"][name],
            "{name}"
        );
    }
}

#[test]
fn unsupported_class_visibility_does_not_invent_member_modifier_errors() {
    // PHPStan 2.3.1 reports only "Syntax error, unexpected T_PUBLIC on line 2" (phpstan.parse).
    // Mago accepts this generic modifier sequence. Grammar translation remains unsupported, but a class
    // header must not produce the member checker's "Multiple access type modifiers" semantic error.
    let root = Inputs::new();
    let file = root.write(
        "unsupported-class-visibility",
        include_str!("fixtures/parser/class-modifier-unsupported.php")
            .strip_prefix("<?php\n")
            .unwrap(),
    );
    let analysis = analyse_project(&Project::new(vec![file], Vec::new())).unwrap();
    assert!(
        analysis
            .errors
            .iter()
            .all(|e| { e.identifier != "phpstan.parse" || e.message.starts_with("Syntax error,") })
    );
}

#[test]
fn readonly_class_headers_follow_the_target_lexer_version() {
    let inputs: Value =
        serde_json::from_str(include_str!("fixtures/parser/readonly-version-inputs.json")).unwrap();
    let expected: Value = serde_json::from_str(include_str!(
        "fixtures/parser/readonly-version-expected.json"
    ))
    .unwrap();
    let root = Inputs::new();
    let files: Vec<_> = inputs
        .as_object()
        .unwrap()
        .iter()
        .map(|(name, code)| {
            let namespace = name.replace('-', "_");
            root.write(
                name,
                &format!(
                    "namespace ReadonlyVersion_{namespace};\n{}",
                    code.as_str().unwrap()
                ),
            )
        })
        .collect();
    for version in [80000, 80100, 80200, 80400, 80500] {
        let mut project = Project::new(files.clone(), Vec::new());
        project.php_version = version;
        assert_eq!(
            errors(&project),
            expected["analysed"][version.to_string()],
            "{version}"
        );
    }
}

#[test]
fn reverse_readonly_keyword_failures_reject_all_scanned_symbols() {
    let inputs: Value =
        serde_json::from_str(include_str!("fixtures/parser/readonly-version-scans.json")).unwrap();
    let expected: Value = serde_json::from_str(include_str!(
        "fixtures/parser/readonly-version-expected.json"
    ))
    .unwrap();
    for (name, code) in inputs.as_object().unwrap() {
        let root = Inputs::new();
        let main = root.write(
            "main",
            include_str!("fixtures/parser/scan-main.php")
                .strip_prefix("<?php\n")
                .unwrap(),
        );
        let symbols = root.write("symbols", code.as_str().unwrap());
        for version in [80000, 80100, 80200, 80400, 80500] {
            let mut project = Project::new(vec![main.clone()], vec![symbols.clone()]);
            project.php_version = version;
            assert_eq!(
                errors(&project),
                expected["scanned"][name][version.to_string()],
                "{name} {version}"
            );
        }
    }
}

#[test]
fn try_parser_failures_preserve_keyword_lines_and_precede_names_and_rules() {
    let inputs: Value =
        serde_json::from_str(include_str!("fixtures/parser/try-inputs.json")).unwrap();
    let expected: Value =
        serde_json::from_str(include_str!("fixtures/parser/try-expected.json")).unwrap();
    let root = Inputs::new();
    let files = inputs
        .as_object()
        .unwrap()
        .iter()
        .map(|(name, code)| {
            let namespace = name.replace('-', "_");
            root.write(
                name,
                &format!(
                    "namespace TryParser_{namespace};\n{}",
                    code.as_str().unwrap()
                ),
            )
        })
        .collect();
    assert_eq!(
        errors(&Project::new(files, Vec::new())),
        expected["analysed"]["errors"]
    );
}

#[test]
fn try_checks_collect_after_body_parameters_and_nested_tries() {
    use pun_phpstan::{Array, Key, Value as Php};
    let root = Inputs::new();
    let file = root.write(
        "try-order",
        include_str!("fixtures/parser/try-order.php")
            .strip_prefix("<?php\n")
            .unwrap(),
    );
    let mut project = Project::new(vec![file], Vec::new());
    let mut entry = Array::new();
    entry.set(Key::from("identifier"), Php::str("phpstan.parse"));
    project.ignore_errors =
        pun_phpstan::ignore::IgnoredErrors::new(&Array::list([Php::Array(entry)]), true);
    let expected: Value =
        serde_json::from_str(include_str!("fixtures/parser/try-expected.json")).unwrap();
    // The public error list sorts by line, but non-ignorable configured-ignore messages retain collection
    // order. An entry-hook try check would produce the same sorted errors and the wrong ignore messages.
    assert_eq!(
        json!(analyse_project(&project).unwrap().general),
        expected["order"]["general"]
    );
    assert_eq!(errors(&project), expected["order"]["errors"]);
}

#[test]
fn invalid_tries_in_scanned_files_export_no_symbols() {
    let inputs: Value =
        serde_json::from_str(include_str!("fixtures/parser/try-scans.json")).unwrap();
    let expected: Value =
        serde_json::from_str(include_str!("fixtures/parser/try-expected.json")).unwrap();
    for (name, code) in inputs.as_object().unwrap() {
        let root = Inputs::new();
        let main = root.write(
            "main",
            include_str!("fixtures/parser/scan-main.php")
                .strip_prefix("<?php\n")
                .unwrap(),
        );
        let symbols = root.write("symbols", code.as_str().unwrap());
        assert_eq!(
            errors(&Project::new(vec![main], vec![symbols])),
            expected["scanned"][name]["errors"],
            "{name}"
        );
    }
}

#[test]
fn hook_parser_failures_preserve_name_parenthesis_and_brace_locations() {
    let inputs: Value =
        serde_json::from_str(include_str!("fixtures/parser/hook-inputs.json")).unwrap();
    let expected: Value =
        serde_json::from_str(include_str!("fixtures/parser/hook-expected.json")).unwrap();
    let root = Inputs::new();
    let files = inputs
        .as_object()
        .unwrap()
        .iter()
        .map(|(name, code)| {
            let namespace = name.replace('-', "_");
            root.write(
                name,
                &format!(
                    "namespace HookParser_{namespace};\n{}",
                    code.as_str().unwrap()
                ),
            )
        })
        .collect();
    assert_eq!(
        errors(&Project::new(files, Vec::new())),
        expected["analysed"]["errors"]
    );
}

#[test]
fn hook_checks_follow_parameters_and_bodies_but_precede_promoted_parameter_checks() {
    use pun_phpstan::{Array, Key, Value as Php};
    let root = Inputs::new();
    let file = root.write(
        "hook-order",
        include_str!("fixtures/parser/hook-order.php")
            .strip_prefix("<?php\n")
            .unwrap(),
    );
    let mut project = Project::new(vec![file], Vec::new());
    let mut entry = Array::new();
    entry.set(Key::from("identifier"), Php::str("phpstan.parse"));
    project.ignore_errors =
        pun_phpstan::ignore::IgnoredErrors::new(&Array::list([Php::Array(entry)]), true);
    let expected: Value =
        serde_json::from_str(include_str!("fixtures/parser/hook-expected.json")).unwrap();
    // Configured-ignore messages expose collection order before analyse_project sorts the error list.
    assert_eq!(
        json!(analyse_project(&project).unwrap().general),
        expected["order"]["general"]
    );
    assert_eq!(errors(&project), expected["order"]["errors"]);
}

#[test]
fn invalid_hooks_in_scanned_files_export_no_symbols() {
    let inputs: Value =
        serde_json::from_str(include_str!("fixtures/parser/hook-scans.json")).unwrap();
    let expected: Value =
        serde_json::from_str(include_str!("fixtures/parser/hook-expected.json")).unwrap();
    for (name, code) in inputs.as_object().unwrap() {
        let root = Inputs::new();
        let main = root.write(
            "main",
            include_str!("fixtures/parser/scan-main.php")
                .strip_prefix("<?php\n")
                .unwrap(),
        );
        let symbols = root.write("symbols", code.as_str().unwrap());
        assert_eq!(
            errors(&Project::new(vec![main], vec![symbols])),
            expected["scanned"][name]["errors"],
            "{name}"
        );
    }
}

#[test]
fn namespace_parser_failures_preserve_child_keyword_lines_and_parser_precedence() {
    let inputs: Value =
        serde_json::from_str(include_str!("fixtures/parser/namespace-inputs.json")).unwrap();
    let expected: Value =
        serde_json::from_str(include_str!("fixtures/parser/namespace-expected.json")).unwrap();
    let root = Inputs::new();
    let files = inputs
        .as_object()
        .unwrap()
        .iter()
        .map(|(name, code)| root.write(name, code.as_str().unwrap()))
        .collect();
    assert_eq!(
        errors(&Project::new(files, Vec::new())),
        expected["analysed"]["errors"]
    );
}

#[test]
fn namespace_checks_follow_complete_bodies_then_collect_direct_children_in_order() {
    use pun_phpstan::{Array, Key, Value as Php};
    let root = Inputs::new();
    let file = root.write(
        "namespace-order",
        include_str!("fixtures/parser/namespace-order.php")
            .strip_prefix("<?php\n")
            .unwrap(),
    );
    let mut project = Project::new(vec![file], Vec::new());
    let mut entry = Array::new();
    entry.set(Key::from("identifier"), Php::str("phpstan.parse"));
    project.ignore_errors =
        pun_phpstan::ignore::IgnoredErrors::new(&Array::list([Php::Array(entry)]), true);
    let expected: Value =
        serde_json::from_str(include_str!("fixtures/parser/namespace-expected.json")).unwrap();
    // The non-ignorable configured-ignore messages expose collection order before error sorting. Checking
    // each child on entry or exit would incorrectly interleave nested-namespace errors with later siblings.
    assert_eq!(
        json!(analyse_project(&project).unwrap().general),
        expected["order"]["general"]
    );
    assert_eq!(errors(&project), expected["order"]["errors"]);
}

#[test]
fn invalid_nested_namespaces_in_scanned_files_export_no_symbols() {
    let inputs: Value =
        serde_json::from_str(include_str!("fixtures/parser/namespace-scans.json")).unwrap();
    let expected: Value =
        serde_json::from_str(include_str!("fixtures/parser/namespace-expected.json")).unwrap();
    for (name, code) in inputs.as_object().unwrap() {
        let root = Inputs::new();
        let main = root.write(
            "main",
            include_str!("fixtures/parser/scan-main.php")
                .strip_prefix("<?php\n")
                .unwrap(),
        );
        let symbols = root.write("symbols", code.as_str().unwrap());
        assert_eq!(
            errors(&Project::new(vec![main], vec![symbols])),
            expected["scanned"][name]["errors"],
            "{name}"
        );
    }
}

#[test]
fn constant_attribute_parser_failures_preserve_first_group_location_and_valid_declarations() {
    let inputs: Value = serde_json::from_str(include_str!(
        "fixtures/parser/constant-attribute-inputs.json"
    ))
    .unwrap();
    let expected: Value = serde_json::from_str(include_str!(
        "fixtures/parser/constant-attribute-expected.json"
    ))
    .unwrap();
    let root = Inputs::new();
    let files = inputs
        .as_object()
        .unwrap()
        .iter()
        .map(|(name, code)| {
            let namespace = name.replace('-', "_");
            root.write(
                name,
                &format!(
                    "namespace ConstantAttribute_{namespace};\n{}",
                    code.as_str().unwrap()
                ),
            )
        })
        .collect();
    assert_eq!(
        errors(&Project::new(files, Vec::new())),
        expected["analysed"]["errors"]
    );
}

#[test]
fn constant_attribute_checks_follow_value_expressions_and_precede_name_resolution() {
    use pun_phpstan::{Array, Key, Value as Php};
    let root = Inputs::new();
    let file = root.write(
        "constant-attribute-order",
        include_str!("fixtures/parser/constant-attribute-order.php")
            .strip_prefix("<?php\n")
            .unwrap(),
    );
    let mut project = Project::new(vec![file], Vec::new());
    let mut entry = Array::new();
    entry.set(Key::from("identifier"), Php::str("phpstan.parse"));
    project.ignore_errors =
        pun_phpstan::ignore::IgnoredErrors::new(&Array::list([Php::Array(entry)]), true);
    let expected: Value = serde_json::from_str(include_str!(
        "fixtures/parser/constant-attribute-expected.json"
    ))
    .unwrap();
    // Ignoring a non-ignorable parser error exposes its collection order, before the public error sorting.
    assert_eq!(
        json!(analyse_project(&project).unwrap().general),
        expected["order"]["general"]
    );
    assert_eq!(errors(&project), expected["order"]["errors"]);
}

#[test]
fn invalid_constant_attributes_in_scanned_files_export_no_symbols() {
    let inputs: Value = serde_json::from_str(include_str!(
        "fixtures/parser/constant-attribute-scans.json"
    ))
    .unwrap();
    let expected: Value = serde_json::from_str(include_str!(
        "fixtures/parser/constant-attribute-expected.json"
    ))
    .unwrap();
    for (name, code) in inputs.as_object().unwrap() {
        let root = Inputs::new();
        let main = root.write(
            "main",
            include_str!("fixtures/parser/scan-main.php")
                .strip_prefix("<?php\n")
                .unwrap(),
        );
        let symbols = root.write("symbols", code.as_str().unwrap());
        assert_eq!(
            errors(&Project::new(vec![main], vec![symbols])),
            expected["scanned"][name]["errors"],
            "{name}"
        );
    }
}

#[test]
fn namespace_postprocessing_preserves_placement_styles_tags_comments_and_statement_starts() {
    let inputs: Value =
        serde_json::from_str(include_str!("fixtures/parser/namespace-post-inputs.json")).unwrap();
    let expected: Value =
        serde_json::from_str(include_str!("fixtures/parser/namespace-post-expected.json")).unwrap();
    let root = Inputs::new();
    let files = inputs
        .as_object()
        .unwrap()
        .iter()
        .map(|(name, code)| root.write_source(name, code.as_str().unwrap()))
        .collect();
    assert_eq!(
        errors(&Project::new(files, Vec::new())),
        expected["analysed"]["errors"]
    );
}

#[test]
fn namespace_postprocessing_follows_body_checks_and_mixed_styles_suppress_outside_errors() {
    use pun_phpstan::{Array, Key, Value as Php};
    let inputs: Value =
        serde_json::from_str(include_str!("fixtures/parser/namespace-post-order.json")).unwrap();
    let expected: Value =
        serde_json::from_str(include_str!("fixtures/parser/namespace-post-expected.json")).unwrap();
    for (name, code) in inputs.as_object().unwrap() {
        let root = Inputs::new();
        let file = root.write_source("order", code.as_str().unwrap());
        let mut project = Project::new(vec![file], Vec::new());
        let mut entry = Array::new();
        entry.set(Key::from("identifier"), Php::str("phpstan.parse"));
        project.ignore_errors =
            pun_phpstan::ignore::IgnoredErrors::new(&Array::list([Php::Array(entry)]), true);
        // The configured-ignore messages preserve collection order before public diagnostics are sorted.
        assert_eq!(
            json!(analyse_project(&project).unwrap().general),
            expected["order"][name]["general"],
            "{name}"
        );
        assert_eq!(
            errors(&project),
            expected["order"][name]["errors"],
            "{name}"
        );
    }
}

#[test]
fn namespace_postprocessing_rejects_scanned_symbols_and_preserves_valid_neighbors() {
    let inputs: Value =
        serde_json::from_str(include_str!("fixtures/parser/namespace-post-scans.json")).unwrap();
    let expected: Value =
        serde_json::from_str(include_str!("fixtures/parser/namespace-post-expected.json")).unwrap();
    for (name, code) in inputs.as_object().unwrap() {
        let root = Inputs::new();
        let main = root.write_source("main", include_str!("fixtures/parser/scan-main.php"));
        let symbols = root.write_source("symbols", code.as_str().unwrap());
        assert_eq!(
            errors(&Project::new(vec![main], vec![symbols])),
            expected["scanned"][name]["errors"],
            "{name}"
        );
    }
}

#[test]
fn entry_brace_errors_match_phpstan_and_suppress_suffix_analysis() {
    let inputs: Value =
        serde_json::from_str(include_str!("fixtures/parser/entry-brace-inputs.json")).unwrap();
    let expected: Value =
        serde_json::from_str(include_str!("fixtures/parser/entry-brace-expected.json")).unwrap();
    for (name, source) in inputs.as_object().unwrap() {
        let root = Inputs::new();
        let file = root.write_source("input", source.as_str().unwrap());
        assert_eq!(
            errors(&Project::new(vec![file], Vec::new())),
            expected["analysed"][name]["errors"],
            "{name}"
        );
    }
}

#[test]
fn entry_brace_failures_reject_scanned_suffix_symbols() {
    let inputs: Value =
        serde_json::from_str(include_str!("fixtures/parser/entry-brace-scans.json")).unwrap();
    let expected: Value =
        serde_json::from_str(include_str!("fixtures/parser/entry-brace-expected.json")).unwrap();
    for (name, source) in inputs.as_object().unwrap() {
        let root = Inputs::new();
        let main = root.write_source("main", include_str!("fixtures/parser/scan-main.php"));
        let symbols = root.write_source("symbols", source.as_str().unwrap());
        assert_eq!(
            errors(&Project::new(vec![main], vec![symbols])),
            expected["scanned"][name]["errors"],
            "{name}"
        );
    }
}

#[test]
fn entry_brace_errors_survive_configured_ignores_and_rule_filters() {
    use pun_phpstan::{Array, Key, Value as Php};
    let root = Inputs::new();
    let file = root.write_source(
        "order",
        include_str!("fixtures/parser/entry-brace-order.php"),
    );
    let mut project = Project::new(vec![file], Vec::new());
    project.reported = Some(std::sync::Arc::new(|_| false));
    project.report_ignores_without_comments = true;
    let mut entry = Array::new();
    entry.set(Key::from("identifier"), Php::str("phpstan.parse"));
    project.ignore_errors = pun_phpstan::ignore::IgnoredErrors::new(
        &Array::list([Php::Array(entry), Php::str("#^NoSuchError$#")]),
        true,
    );
    let expected: Value =
        serde_json::from_str(include_str!("fixtures/parser/entry-brace-expected.json")).unwrap();
    let analysis = analyse_project(&project).unwrap();
    assert_eq!(json!(analysis.general), expected["order"]["general"]);
    assert_eq!(errors(&project), expected["order"]["errors"]);
}

#[test]
fn entry_brace_translation_leaves_other_grammar_and_lexical_contracts_unchanged() {
    let inputs: Value =
        serde_json::from_str(include_str!("fixtures/parser/entry-brace-unsupported.json")).unwrap();
    let upstream: Value =
        serde_json::from_str(include_str!("fixtures/parser/entry-brace-expected.json")).unwrap();
    for (name, source) in inputs.as_object().unwrap() {
        let root = Inputs::new();
        let file = root.write_source("input", source.as_str().unwrap());
        // These inputs have no supported rule failure. Their broader php-parser failures remain unported;
        // captures retain those diagnostics explicitly rather than presenting this boundary as full parity.
        assert_eq!(
            errors(&Project::new(vec![file], Vec::new())),
            json!([]),
            "{name}: broader upstream contract is {}",
            upstream["unsupported"][name]
        );
    }
}

#[test]
fn pipe_parentheses_preserve_direct_rhs_identity_starts_and_valid_neighbors() {
    let inputs: Value =
        serde_json::from_str(include_str!("fixtures/parser/pipe-parentheses-inputs.json")).unwrap();
    let expected: Value = serde_json::from_str(include_str!(
        "fixtures/parser/pipe-parentheses-expected.json"
    ))
    .unwrap();
    let root = Inputs::new();
    let files = inputs
        .as_object()
        .unwrap()
        .iter()
        .map(|(name, code)| root.write(name, code.as_str().unwrap()))
        .collect();
    let mut project = Project::new(files, Vec::new());
    project.php_version = 80500;
    assert_eq!(errors(&project), expected["analysed"]["errors"]);
}

#[test]
fn pipe_parentheses_collect_after_parameters_and_inner_pipes_before_namespace_postprocessing() {
    use pun_phpstan::{Array, Key, Value as Php};
    let root = Inputs::new();
    let file = root.write_source(
        "order",
        include_str!("fixtures/parser/pipe-parentheses-order.php"),
    );
    let mut project = Project::new(vec![file], Vec::new());
    project.php_version = 80500;
    project.reported = Some(std::sync::Arc::new(|_| false));
    let mut entry = Array::new();
    entry.set(Key::from("identifier"), Php::str("phpstan.parse"));
    project.ignore_errors =
        pun_phpstan::ignore::IgnoredErrors::new(&Array::list([Php::Array(entry)]), true);
    let expected: Value = serde_json::from_str(include_str!(
        "fixtures/parser/pipe-parentheses-expected.json"
    ))
    .unwrap();
    // Sorted errors alone cannot distinguish entry and exit checks. Configured-ignore messages retain
    // parameter, inner pipe, outer pipe, root namespace reduction order, even with every rule filtered out.
    assert_eq!(
        json!(analyse_project(&project).unwrap().general),
        expected["order"]["general"]
    );
    assert_eq!(errors(&project), expected["order"]["errors"]);
}

#[test]
fn pipe_parentheses_failures_in_scanned_files_reject_all_symbols() {
    let inputs: Value =
        serde_json::from_str(include_str!("fixtures/parser/pipe-parentheses-scans.json")).unwrap();
    let expected: Value = serde_json::from_str(include_str!(
        "fixtures/parser/pipe-parentheses-expected.json"
    ))
    .unwrap();
    for (name, code) in inputs.as_object().unwrap() {
        let root = Inputs::new();
        let main = root.write_source("main", include_str!("fixtures/parser/scan-main.php"));
        let symbols = root.write("symbols", code.as_str().unwrap());
        let mut project = Project::new(vec![main], vec![symbols]);
        project.php_version = 80500;
        assert_eq!(
            errors(&project),
            expected["scanned"][name]["errors"],
            "{name}"
        );
    }
}

#[test]
fn pipe_parentheses_do_not_translate_older_target_pipe_grammar() {
    let expected: Value = serde_json::from_str(include_str!(
        "fixtures/parser/pipe-parentheses-expected.json"
    ))
    .unwrap();
    let root = Inputs::new();
    let file = root.write("input", "1 |> fn($x) => $x;");
    for version in [80000, 80100, 80200, 80400] {
        let mut project = Project::new(vec![file.clone()], Vec::new());
        project.php_version = version;
        // Reverse PipeOperatorEmulator produces | and > before PHP 8.5, not the semantic T_PIPE check.
        // Allow the captured grammar diagnostics as they are ported, but never substitute this message.
        let got = errors(&project);
        let want = expected["older"][version.to_string()]["errors"]
            .as_array()
            .unwrap();
        assert!(
            got.as_array()
                .unwrap()
                .iter()
                .all(|error| want.contains(error)),
            "PHP {version}: got {got}, oracle {want:?}"
        );
    }
}
