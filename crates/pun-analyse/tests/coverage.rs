//! Branches PHPStan's own rule fixtures miss: each directory in tests/fixtures/coverage holds PHP files and one or
//! more phpstan*.neon, and expected-<neon>.json is PHPStan's run on it (`scripts/export-analyse-expected.py <dir>
//! [neon]`), restricted to the identifiers Pun reports.

use std::path::Path;

use pun_analyse::{analyse, reports};
use pun_phpstan::config::{Options, resolve};
use serde_json::Value;

type Key = (String, i64, String, String, Option<String>);

/// PHPStan 2.3 errors Pun does not report yet (case directory, file, line, identifier); one Pun reports fails the
/// test as extra.
const GAPS: &[(&str, &str, i64, &str)] = &[
    // PHPStan 2.3.1's loop scope: `while (true) { ...; $x = 1; if ($x) break; }` never reaches a second iteration, so
    // $x is undefined at the loop start; Pun merges the end-of-body scope back in
    ("var_names", "vars.php", 84, "varTag.variableNotFound"),
    // Mago types an int mixed after an invalid `$i += 'x'`, so NullCoalesceRule cannot tell it is not nullable
    // The conservative declaration-effects producer does not follow labels/goto or general branches.
    ("level4-unused", "src/labels.php", 5, "void.pure"),
    // `||` conditions and branch joins remain unknown, not proven empty.
    ("strict-operators", "operators.php", 95, "void.pure"),
    // First-class callable creation has no declaration-effects producer yet.
    ("types", "types.php", 72, "void.pure"),
    // Foreach scope/effect joins remain unported for declaration purity.
    ("types", "types.php", 281, "void.pure"),
    // Local class declarations need their own producer, separate from their methods' bodies.
    ("types-level4", "checks.php", 203, "void.pure"),
    // ConstantResolver::resolveConstantType() types a fetch of a `dynamicConstantNames` constant with its configured
    // type (`int` here); Pun types the fetch with the defined value 42, which `positive-int` accepts
    (
        "dynamic-constant-values",
        "value-assigned-to-global-constant.php",
        5,
        "constant.value",
    ),
    // Calling a `callable(A|B, int): B` parameter infers its own A and B again from the arguments, so the result is A
    // where PHPStan keeps the function's B: the return reads `TReduceInitial`, not `TReduceInitial|TReduceReturnType`
    ("max", "template-self-reference.php", 32, "return.type"),
    // PHPStan reflects each declaration of a duplicated class name; the codebase keeps one class-like per name, so
    // the methods of the other declaration have no metadata to check
    ("duplicate-classes", "a.php", 3, "missingType.return"),
];

/// Errors Pun reports where PHPStan does not, by an inference difference (case directory, file, line, identifier).
/// Each must still be reported: one that disappears fails the test as missing.
const EXTRA: &[(&str, &str, i64, &str)] = &[
    // PHPStan makes a holder an ErrorType after an invalid offset write (`$a[1] = 2`, `$closed['x'] = 1`, `$i['k'] = 1`);
    // Mago keeps its type, so NonexistentOffsetInArrayDimFetchRule still checks the later `.=` offsets
    ("types", "offsets.php", 122, "offsetAccess.notFound"),
    (
        "types",
        "offsets.php",
        124,
        "offsetAccess.nonOffsetAccessible",
    ),
    (
        "types",
        "offsets.php",
        125,
        "offsetAccess.nonOffsetAccessible",
    ),
    // The GAPS entry's message with `TReduceInitial` alone
    ("max", "template-self-reference.php", 32, "return.type"),
    // PHPStan 2.3.5 reports an unmatched entry in its config file. These two match PHPStan's errors that Pun does not
    // report: expr.resultUnused on the `$f()` calls (unused-variable.php:278, 509, 543) and booleanOr.leftAlwaysFalse
    // (unused-variable-usage-flow.php:21)
    ("unused-variable", "phpstan.neon", 10, "ignore.unmatched"),
    ("unused-variable", "phpstan.neon", 20, "ignore.unmatched"),
];

fn check(dir: &Path, neon: &str) -> Result<(), String> {
    let root = dir.to_string_lossy().into_owned();
    let options = Options {
        config: Some(format!("{root}/{neon}.neon")),
        ..Options::for_project(&root)
    };
    let config = resolve(&options)?;
    let prefix = format!("{root}/");
    let mut got: Vec<Key> = analyse(dir, &config)
        .unwrap()
        .into_iter()
        .map(|e| {
            let file = e.file.strip_prefix(&prefix).unwrap_or(&e.file).to_string();
            (file, e.line, e.identifier, e.message, e.tip)
        })
        .collect();
    got.sort();
    // An entry in a config file belongs to that config's run only
    let ours = |file: &str| !file.ends_with(".neon") || file == format!("{neon}.neon");
    for (case, file, line, id) in EXTRA
        .iter()
        .filter(|(case, file, ..)| dir.ends_with(case) && ours(file))
    {
        let at = got
            .iter()
            .position(|k| k.0 == *file && k.1 == *line && k.2 == *id)
            .ok_or_else(|| {
                format!("EXTRA lists {case} {file}:{line} {id}, which Pun no longer reports")
            })?;
        got.remove(at);
    }
    let expected: Value =
        serde_json::from_slice(&std::fs::read(dir.join(format!("expected-{neon}.json"))).unwrap())
            .unwrap();
    let mut want: Vec<Key> = expected["errors"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|e| e["identifier"].as_str().is_some_and(reports))
        .filter(|e| {
            !GAPS.iter().any(|(case, file, line, id)| {
                dir.ends_with(case)
                    && e["file"] == *file
                    && e["line"] == *line
                    && e["identifier"] == *id
            })
        })
        .map(|e| {
            (
                e["file"].as_str().unwrap().to_string(),
                e["line"].as_i64().unwrap(),
                e["identifier"].as_str().unwrap().to_string(),
                e["message"].as_str().unwrap().to_string(),
                e["tip"].as_str().map(str::to_string),
            )
        })
        .collect();
    want.sort();
    if got == want {
        return Ok(());
    }
    let missing: Vec<_> = want.iter().filter(|k| !got.contains(k)).collect();
    let extra: Vec<_> = got.iter().filter(|k| !want.contains(k)).collect();
    Err(format!("missing {missing:#?}\nextra {extra:#?}"))
}

#[test]
fn coverage_fixtures() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/coverage");
    let mut cases: Vec<_> = std::fs::read_dir(&root)
        .unwrap()
        .flatten()
        .map(|e| e.path())
        .collect();
    cases.sort();
    // PUN_COVERAGE_FILTER=types runs only the cases whose directory name contains it
    let filter = std::env::var("PUN_COVERAGE_FILTER").unwrap_or_default();
    cases.retain(|d| d.file_name().unwrap().to_string_lossy().contains(&filter));
    let mut failures = Vec::new();
    for dir in cases {
        let mut neons: Vec<String> = std::fs::read_dir(&dir)
            .unwrap()
            .flatten()
            .filter_map(|e| {
                let name = e.file_name().into_string().unwrap();
                name.strip_suffix(".neon").map(str::to_string)
            })
            .collect();
        neons.sort();
        for neon in neons {
            if let Err(e) = check(&dir, &neon) {
                failures.push(format!("{}/{neon}: {e}", dir.display()));
            }
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n\n"));
}

/// The original level-0 rule case must match without consulting or changing its gap allowance.
#[test]
fn array_shape_template_key_original_rule_fixture() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/rules");
    let oracle: Value = serde_json::from_slice(
        &std::fs::read(root.join("Methods/CallMethodsRuleTest.json")).unwrap(),
    )
    .unwrap();
    let case = oracle["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|case| case["method"] == "testArrayShapeTemplateKey")
        .unwrap();
    let data = root.join("data");
    let paths = case["files"]
        .as_array()
        .unwrap()
        .iter()
        .map(|file| data.join(file.as_str().unwrap()))
        .collect();
    let actual: Vec<_> = pun_analyse::run(&pun_analyse::Project::new(paths, Vec::new()))
        .unwrap()
        .into_iter()
        .map(|error| {
            serde_json::json!({
                "file": Path::new(&error.file).strip_prefix(&data).unwrap(),
                "line": error.line,
                "identifier": error.identifier,
                "message": error.message,
                "tip": error.tip,
            })
        })
        .collect();
    assert_eq!(serde_json::json!(actual), case["errors"]);
}

/// A circular class hierarchy stops PHPStan with "Reflection error: Circular reference to class" (no file errors to
/// compare); the PHPDoc inheritance walks must still end.
#[test]
fn circular_hierarchy_ends() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/cycle");
    let mut project = pun_analyse::Project::new(vec![dir.join("cycle.php")], Vec::new());
    project.level = 6;
    assert!(pun_analyse::run(&project).is_ok());
}

#[test]
fn literal_integer_division_overflow_matches_php_and_phpstan() {
    use std::borrow::Cow;

    use mago_allocator::LocalArena;
    use mago_analyzer::{
        Analyzer, analysis_result::AnalysisResult, plugin::PluginRegistry, settings::Settings,
    };
    use mago_codex::{
        populator::populate_codebase, reference::SymbolReferences, scanner::scan_program,
    };
    use mago_database::file::{File, FileId, FileType};
    use mago_names::resolver::NameResolver;
    use mago_span::{Position, Span};
    use mago_syntax::parser::parse_file;

    let code = include_bytes!("fixtures/coverage/integer-division-overflow/literals.php");
    let oracle: Value = serde_json::from_str(include_str!(
        "fixtures/coverage/integer-division-overflow/expected-types-phpstan.json"
    ))
    .unwrap();
    let runtime: Value = serde_json::from_str(include_str!(
        "fixtures/coverage/integer-division-overflow/expected-runtime-php.json"
    ))
    .unwrap();
    let analyze = |native| {
        let file = File::new(
            Cow::Borrowed(b"literals.php"),
            FileType::Host,
            None,
            Cow::Borrowed(code.as_slice()),
        );
        let arena = LocalArena::new();
        let program = parse_file(&arena, &file);
        assert!(!program.has_errors());
        let names = NameResolver::new(&arena).resolve(program);
        let settings = Settings {
            track_native_types: native,
            ..Default::default()
        };
        let mut codebase = scan_program(&arena, &file, program, &names, settings.version);
        let mut references = SymbolReferences::new();
        populate_codebase(
            &mut codebase,
            &mut references,
            Default::default(),
            Default::default(),
        );
        let registry = PluginRegistry::new();
        let analyzer = Analyzer::new(&arena, &file, &names, &codebase, &registry, settings);
        let mut result = AnalysisResult::new(references);
        let artifacts = analyzer
            .analyze_with_artifacts(program, &mut result)
            .unwrap();
        (artifacts, result)
    };
    let (enabled, result) = analyze(true);
    let (disabled, off_result) = analyze(false);
    assert_eq!(enabled.expression_types, disabled.expression_types);
    assert_eq!(result.symbol_references, off_result.symbol_references);
    assert_eq!(
        format!("{:?}", result.issues),
        format!("{:?}", off_result.issues)
    );
    assert_eq!(
        result
            .issues
            .iter()
            .filter(|issue| issue.code.as_deref() == Some("invalid-operand"))
            .count(),
        2
    );

    let oracle = oracle.as_array().unwrap();
    let runtime = runtime.as_array().unwrap();
    assert_eq!(oracle.len(), runtime.len());
    let mut offset = 0;
    let mut checked = 0;
    for (line, source) in std::str::from_utf8(code)
        .unwrap()
        .split_inclusive('\n')
        .enumerate()
    {
        if let Some((expression, _)) = source.split_once("// result") {
            let expression = expression.trim_end().strip_suffix(';').unwrap();
            let start = offset + expression.len() - expression.trim_start().len();
            let span = Span::new(
                FileId::zero(),
                Position::new(start as u32),
                Position::new((offset + expression.len()) as u32),
            );
            let ty = enabled.get_expression_type(&span).unwrap();
            let php = &runtime[checked];
            let phpstan = &oracle[checked];
            assert_eq!(php["line"], line + 1);
            assert_eq!(phpstan["line"], php["line"]);
            assert_eq!(php["expression"], expression.trim());
            let expected = phpstan["message"]
                .as_str()
                .unwrap()
                .strip_prefix("Dumped type: ")
                .unwrap();
            if php["exception"] == "DivisionByZeroError" {
                assert_eq!(expected, "*ERROR*");
                // PHPStan's ErrorType is not a scalar expectation. Verify Mago's rejection at
                // this expression; its existing invalid-operand fallback is outside this patch.
                assert!(result.issues.iter().any(|issue| {
                    issue.code.as_deref() == Some("invalid-operand")
                        && issue.annotations.iter().any(|annotation| {
                            annotation.kind.is_primary()
                                && annotation.span.start.offset >= span.start.offset
                                && annotation.span.end.offset <= span.end.offset
                        })
                }));
                assert!(enabled.get_native_expression_type(&span).is_none());
            } else {
                if php["type"] == "int" {
                    let value = php["value"].as_i64().unwrap();
                    assert_eq!(expected.parse::<i64>().unwrap(), value);
                    assert_eq!(ty.get_single_literal_int_value(), Some(value));
                } else {
                    assert_eq!(php["type"], "float");
                    let value = php["value"].as_f64().unwrap();
                    assert_eq!(expected.parse::<f64>().unwrap(), value);
                    assert_eq!(ty.get_single_literal_float_value(), Some(value));
                }
                assert_eq!(
                    enabled
                        .get_native_expression_type(&span)
                        .unwrap()
                        .get_single(),
                    ty.get_single()
                );
            }
            assert!(disabled.get_native_expression_type(&span).is_none());
            checked += 1;
        }
        offset += source.len();
    }
    assert_eq!(checked, oracle.len());
}
