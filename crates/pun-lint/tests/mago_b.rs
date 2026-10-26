//! Mago's own examples and test snippets for the rules of the second Mago batch, ported from Mago 1.57.0's linter
//! (copied from mago-linter's rule files into tests/mago/<group>-<name>/Mago), and Pun's own inputs (Own/). Each
//! `<case>.expect` is what Mago 1.57.0 (`mago --php-version 8.4 lint --only <code> --reporting-format json`)
//! reported for the input: one line per issue, `start end message` of its primary annotation, sorted by start. A
//! `<case>.json` holds the rule's options, as both Mago's `[linter.rules]` and pun.toml take them; a
//! `<case>.filename` the name Mago saw the file under. Pun/ holds cases where Pun knowingly differs from Mago, with
//! the expectation derived by hand (each rule's module docs say why).

use std::path::{Path, PathBuf};

use mago_syntax::cst::Node;
use pun_lint::{Linter, Rule, rules};

fn cases(dir: &str) -> Vec<PathBuf> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/mago")
        .join(dir);
    let mut paths = Vec::new();
    for sub in std::fs::read_dir(&root).unwrap() {
        for entry in std::fs::read_dir(sub.unwrap().path()).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().is_some_and(|e| e == "php") {
                paths.push(path);
            }
        }
    }
    paths.sort();
    assert!(!paths.is_empty(), "no cases in {}", root.display());
    paths
}

/// A rule called on every node, not only the kinds of its `targets()`.
struct Everywhere(Box<dyn Rule>);

impl Rule for Everywhere {
    fn id(&self) -> &'static str {
        self.0.id()
    }

    fn min_php_version(&self) -> u32 {
        self.0.min_php_version()
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &pun_lint::Context<'_>,
        out: &mut Vec<pun_lint::Report>,
    ) {
        self.0.visit(node, ancestors, ctx, out);
    }
}

/// Runs every case of `dir` through the rule `id`, configured with the case's options. `targets()` only spares the
/// engine work, so the rule also runs on every node and must report the same.
fn run(id: &str, dir: &str) {
    for path in cases(dir) {
        let name = path.file_name().unwrap().to_string_lossy().into_owned();
        for everywhere in [false, true] {
            let mut rule = rules::all()
                .into_iter()
                .find(|r| r.id() == id)
                .unwrap_or_else(|| panic!("{id} is not registered"));
            let options = std::fs::read_to_string(path.with_extension("json"))
                .map_or(serde_json::Value::Null, |s| {
                    serde_json::from_str(&s).unwrap()
                });
            rule.configure(&options).unwrap();
            if everywhere {
                rule = Box::new(Everywhere(rule));
            }
            let linter = Linter::new(vec![rule], 80400);
            let file = std::fs::read_to_string(path.with_extension("filename"))
                .map_or(path.clone(), |f| PathBuf::from(f.trim()));
            let input = std::fs::read_to_string(&path).unwrap();
            let expected = std::fs::read_to_string(path.with_extension("expect")).unwrap();
            let got: String = linter
                .check(&file, &input)
                .unwrap()
                .iter()
                .map(|p| format!("{} {} {}\n", p.span.start, p.span.end, p.message))
                .collect();
            assert_eq!(got, expected, "{dir}/{name} (everywhere: {everywhere})");
            if let Ok(fixed) = std::fs::read_to_string(path.with_extension("fixed")) {
                assert_eq!(
                    linter.fix(&file, &input).unwrap().source,
                    fixed,
                    "{dir}/{name}: fix (everywhere: {everywhere})"
                );
            }
        }
    }
}

/// Each option is checked for its type, and an unknown one is refused.
fn rejects(id: &str, bad: &[serde_json::Value]) {
    for options in bad {
        let mut rule = rules::all().into_iter().find(|r| r.id() == id).unwrap();
        assert!(rule.configure(options).is_err(), "{id}: {options}");
    }
}

macro_rules! mago {
    ($($test:ident $group:literal $name:literal;)*) => {
        $(
            #[test]
            fn $test() {
                run(concat!("lint/", $group, "/", $name), concat!($group, "-", $name));
            }
        )*
    };
}

mago! {
    no_boolean_flag_parameter "complexity" "no-boolean-flag-parameter";
    no_else_clause "complexity" "no-else-clause";
    no_empty_file "complexity" "no-empty-file";
    no_excessive_cyclomatic "complexity" "no-excessive-cyclomatic";
    no_excessive_enum_cases "complexity" "no-excessive-enum-cases";
    no_excessive_halstead "complexity" "no-excessive-halstead";
    no_excessive_kan_defect "complexity" "no-excessive-kan-defect";
    no_excessive_methods "complexity" "no-excessive-methods";
    no_excessive_nesting "complexity" "no-excessive-nesting";
    no_excessive_parameters "complexity" "no-excessive-parameters";
    no_excessive_properties "complexity" "no-excessive-properties";
    no_goto "complexity" "no-goto";
    prefer_named_literal_argument "complexity" "prefer-named-literal-argument";
    require_assert_description "functions" "require-assert-description";
    no_restricted_functions "security" "no-restricted-functions";
    no_restricted_instantiation "security" "no-restricted-instantiation";
    no_ambiguous_constant "style" "no-ambiguous-constant";
    no_ambiguous_function "style" "no-ambiguous-function";
    no_ini_set "style" "no-ini-set";
    no_inline_html "style" "no-inline-html";
    no_invalid_phpdoc "style" "no-invalid-phpdoc";
    no_isset "style" "no-isset";
    no_multiple_classes_per_file "style" "no-multiple-classes-per-file";
    no_sprintf_concat "style" "no-sprintf-concat";
    prefer_create_from_timestamp "style" "prefer-create-from-timestamp";
    prefer_while_loop "style" "prefer-while-loop";
    require_namespace "style" "require-namespace";
    require_sorted_integer_keys "style" "require-sorted-integer-keys";
    require_todo_reference "style" "require-todo-reference";
    require_interpolation_style "strings" "require-interpolation-style";
    no_side_effects_with_declarations "style" "no-side-effects-with-declarations";
    require_naming_convention "style" "require-naming-convention";
    require_phpdoc "style" "require-phpdoc";
}

#[test]
fn options_are_checked() {
    use serde_json::json;
    let common = [
        json!(true),
        json!({"unknown": 1}),
        json!({"threshold": "a"}),
        json!({"threshold": -1}),
    ];
    for id in [
        "lint/complexity/no-excessive-cyclomatic",
        "lint/complexity/no-excessive-enum-cases",
        "lint/complexity/no-excessive-methods",
        "lint/complexity/no-excessive-nesting",
        "lint/complexity/no-excessive-parameters",
        "lint/complexity/no-excessive-properties",
        "lint/complexity/prefer-named-literal-argument",
    ] {
        rejects(id, &common);
    }
    rejects(
        "lint/complexity/no-excessive-parameters",
        &[json!({"threshold": 256})],
    );
    rejects(
        "lint/complexity/no-excessive-methods",
        &[json!({"threshold": 65536}), json!({"count-hooks": 1})],
    );
    rejects(
        "lint/complexity/no-excessive-halstead",
        &[json!({"volume-threshold": "x"}), json!({"x": 1.0})],
    );
    rejects(
        "lint/complexity/no-excessive-kan-defect",
        &[json!({"threshold": "x"}), json!({"x": 1.0})],
    );
    rejects(
        "lint/complexity/no-boolean-flag-parameter",
        &[json!({"exclude-setters": 1}), json!({"x": true})],
    );
    rejects("lint/complexity/no-goto", &[json!({"x": true})]);
    rejects(
        "lint/style/no-isset",
        &[json!({"allow-array-checks": 1}), json!({"x": true})],
    );
    rejects(
        "lint/style/require-phpdoc",
        &[
            json!({"functions": 1}),
            json!({"x": true}),
            json!({"exclusions": []}),
            json!({"exclusions": {"x": []}}),
            json!({"exclusions": {"functions": "a"}}),
            json!({"exclusions": {"functions": [1]}}),
        ],
    );
    rejects(
        "lint/style/require-naming-convention",
        &[
            json!({"class-name": true}),
            json!({"class-name": {"psr": 1}}),
            json!({"class-name": {"camel": true}}),
            json!({"constant-name": {"psr": true}}),
            json!({"enum-name": {"x": true}}),
            json!({"file-name": {}}),
        ],
    );
    rejects(
        "lint/style/no-side-effects-with-declarations",
        &[json!({"allow-class-alias": 1}), json!({"x": true})],
    );
    rejects(
        "lint/strings/require-interpolation-style",
        &[
            json!({"style": "other"}),
            json!({"style": 1}),
            json!({"x": "interpolation"}),
        ],
    );
    for (id, key) in [
        ("lint/security/no-restricted-functions", "functions"),
        ("lint/security/no-restricted-functions", "extensions"),
        ("lint/security/no-restricted-instantiation", "types"),
    ] {
        rejects(
            id,
            &[
                json!({ key: "a" }),
                json!({ key: [1] }),
                json!({ key: [{"help": "x"}] }),
                json!({ key: [{"name": 1}] }),
                json!({ key: [{"name": "a", "help": 1}] }),
                json!({ key: [{"name": "a", "level": "fatal"}] }),
                json!({ key: [{"name": "a", "level": 1}] }),
                json!({"other": []}),
            ],
        );
    }
}
