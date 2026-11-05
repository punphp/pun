//! Failed constants have continuing ErrorType values; dynamic names have genuine mixed values.

use std::borrow::Cow;
use std::collections::BTreeMap;
use std::path::Path;
use std::sync::{Arc, Mutex};

use mago_allocator::LocalArena;
use mago_analyzer::Analyzer;
use mago_analyzer::analysis_result::AnalysisResult;
use mago_analyzer::code::IssueCode;
use mago_analyzer::error::AnalysisError;
use mago_analyzer::plugin::{
    ExpressionHook, ExpressionHookResult, HookContext, HookResult, PluginError,
    create_registry_with_plugins,
};
use mago_analyzer::settings::Settings;
use mago_codex::populator::populate_codebase;
use mago_codex::scanner::scan_program;
use mago_codex::ttype::TType;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::union::{NeverKind, TUnion};
use mago_database::file::{File, FileType};
use mago_names::resolver::NameResolver;
use mago_span::HasSpan;
use mago_syntax::cst::{Access, Expression, Variable};
use mago_syntax::parser::parse_file;
use pun_analyse::{Project, analyse_project};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

fn oracle() -> Value {
    serde_json::from_slice(include_bytes!(
        "fixtures/failed-class-constants/expected.json"
    ))
    .unwrap()
}

fn identity(ty: &TUnion) -> Value {
    let mut ints: Vec<_> = ty
        .types
        .iter()
        .filter_map(TAtomic::get_literal_int_value)
        .collect();
    let mut strings: Vec<_> = ty
        .types
        .iter()
        .filter_map(|atomic| {
            atomic
                .get_literal_string_value()
                .map(|value| String::from_utf8_lossy(value).into_owned())
                .or_else(|| atomic.get_class_string_value().map(|name| name.to_string()))
        })
        .collect();
    let mut objects: Vec<_> = ty
        .types
        .iter()
        .filter_map(|atomic| match atomic {
            TAtomic::Object(object) => object.get_name().map(|name| name.to_string()),
            _ => None,
        })
        .collect();
    ints.sort();
    ints.dedup();
    strings.sort();
    strings.dedup();
    objects.sort();
    objects.dedup();
    let never = ty.never_kind().map(|kind| match kind {
        NeverKind::Implicit => "implicit",
        NeverKind::Explicit => "explicit",
        NeverKind::NonAccepting => "nonaccepting",
    });
    json!({"error": ty.is_error(), "mixed": ty.is_mixed() && !ty.is_error(),
        "never": never, "ints": ints, "strings": strings, "objects": objects})
}

#[derive(Default)]
struct Observed {
    expressions: BTreeMap<String, Value>,
    native: BTreeMap<String, Option<Value>>,
    descriptions: BTreeMap<String, String>,
}

struct Observer(Arc<Mutex<Observed>>, Vec<u8>, Option<u32>);
impl ExpressionHook for Observer {
    fn is_observer(&self) -> bool {
        true
    }

    fn before_expression(
        &self,
        expression: &Expression<'_>,
        _: &mut HookContext<'_, '_>,
    ) -> HookResult<ExpressionHookResult> {
        if self.2 == Some(expression.span().start.offset) {
            return Err(PluginError::Internal {
                reason: "selector analysis failed".into(),
            });
        }
        Ok(ExpressionHookResult::Continue)
    }

    fn after_expression(
        &self,
        expression: &Expression<'_>,
        context: &mut HookContext<'_, '_>,
    ) -> HookResult<()> {
        let span = expression.span();
        let source = &self.1[span.start.offset as usize..span.end.offset as usize];
        if !matches!(
            expression,
            Expression::Access(Access::ClassConstant(_)) | Expression::Assignment(_)
        ) && !source.starts_with(b"'after-")
            && source != b"stop()"
        {
            return Ok(());
        }
        let key = span.start.offset.to_string();
        let mut observed = self.0.lock().unwrap();
        let visits = observed
            .expressions
            .get(&key)
            .map_or(1, |value| value["visits"].as_u64().unwrap() + 1);
        observed.descriptions.insert(
            key.clone(),
            context
                .get_expression_type(expression)
                .unwrap()
                .get_id()
                .to_string(),
        );
        let mut record = json!({"full": identity(context.get_expression_type(expression).unwrap()), "visits": visits});
        if let Expression::Assignment(assignment) = expression
            && let Expression::Variable(Variable::Direct(variable)) = assignment.lhs
        {
            record["assigned"] = identity(context.get_variable_type(variable.name).unwrap());
        }
        observed.native.insert(
            key.clone(),
            context.get_native_expression_type(expression).map(identity),
        );
        observed.expressions.insert(key, record);
        Ok(())
    }
}

fn native_case(name: &str, expected: &Value, native: bool, fail_selector: bool) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/failed-class-constants");
    let source = std::fs::read(root.join(name)).unwrap();
    assert_eq!(
        Sha256::digest(&source)
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect::<String>(),
        expected["sha256"].as_str().unwrap()
    );
    let file = File::ephemeral(
        Cow::Owned(name.as_bytes().to_vec()),
        Cow::Owned(source.clone()),
    );
    let arena = LocalArena::new();
    let program = parse_file(&arena, &file);
    assert!(!program.has_errors(), "{name}: {:?}", program.errors);
    let names = NameResolver::new(&arena).resolve(program);
    let prelude = mago_prelude::Prelude::build();
    let mut codebase = prelude.metadata;
    // Use Pun's real scanned PHPStan API, not an unknown-function fallback for dumpType().
    if source.windows(8).any(|part| part == b"PHPStan\\") {
        let api = File::new(
            Cow::Borrowed(b"phpstan-api.php"),
            FileType::Vendored,
            None,
            Cow::Borrowed(include_bytes!("../data/phpstan-api.php")),
        );
        let api_program = parse_file(&arena, &api);
        let api_names = NameResolver::new(&arena).resolve(api_program);
        codebase.extend(scan_program(
            &arena,
            &api,
            api_program,
            &api_names,
            Default::default(),
        ));
    }
    codebase.extend(scan_program(
        &arena,
        &file,
        program,
        &names,
        Default::default(),
    ));
    let mut references = prelude.symbol_references;
    populate_codebase(
        &mut codebase,
        &mut references,
        Default::default(),
        Default::default(),
    );
    let observed = Arc::<Mutex<Observed>>::default();
    let mut plugins = create_registry_with_plugins(&[], false);
    let failed_operand = fail_selector.then(|| {
        source
            .windows(b"$constName".len())
            .rposition(|part| part == b"$constName")
            .unwrap() as u32
    });
    plugins.register_expression_hook(Observer(
        Arc::clone(&observed),
        source.clone(),
        failed_operand,
    ));
    let mut result = AnalysisResult::new(references);
    let mut settings = Settings::default();
    settings.track_native_types = native;
    let analysed = Analyzer::new(&arena, &file, &names, &codebase, &plugins, settings)
        .analyze_with_artifacts(program, &mut result);
    if fail_selector {
        assert!(matches!(
            analysed,
            Err(AnalysisError::Plugin(PluginError::Internal { ref reason }))
                if reason == "selector analysis failed"
        ));
        return;
    }
    analysed.unwrap();
    let observed = observed.lock().unwrap();
    let mut wanted = BTreeMap::new();
    for (offset, record) in expected["expressions"].as_object().unwrap() {
        if matches!(
            record["type"].as_str(),
            Some("class-string" | "class-string&literal-string")
        ) {
            assert_eq!(
                observed.descriptions.get(offset).map(String::as_str),
                record["type"].as_str(),
                "unnamed class-string kind for {name}: {record}, native={native}"
            );
        }
        let mut value = json!({"full": record["full"], "visits": record["visits"]});
        if let Some(assigned) = record.get("assigned") {
            value["assigned"] = assigned.clone();
        }
        wanted.insert(offset.clone(), value);
        // The partial native API cannot type class constants yet. Do not fabricate a full-type fallback.
        if record["kind"] == "constant" || !native {
            assert_eq!(
                observed.native.get(offset),
                Some(&None),
                "{name}: {record}, native={native}"
            );
        } else if let Some(Some(value)) = observed.native.get(offset) {
            assert_eq!(
                value, &record["native"],
                "{name}: {record}, native={native}"
            );
        }
    }
    assert_eq!(
        observed.expressions, wanted,
        "{name}, native={native}; issues: {:?}",
        result.issues
    );

    let mut actual: Vec<_> = result
        .issues
        .iter()
        .map(|issue| {
            let span = issue
                .annotations
                .iter()
                .find(|annotation| annotation.kind.is_primary())
                .unwrap()
                .span;
            (
                i64::from(file.line_number(span.start.offset)) + 1,
                issue.code.as_deref().unwrap(),
            )
        })
        .collect();
    let mut wanted = Vec::new();
    for error in expected["diagnostics"]["8"].as_array().unwrap() {
        let code = match error[1].as_str().unwrap() {
            "class.notFound" => Some(IssueCode::NonExistentClassLike),
            "classConstant.notFound" => Some(IssueCode::NonExistentClassConstant),
            "outOfClass.self" => Some(IssueCode::SelfOutsideClassScope),
            "outOfClass.static" => Some(IssueCode::StaticOutsideClassScope),
            "outOfClass.parent" => Some(IssueCode::ParentOutsideClassScope),
            "class.noParent" => Some(IssueCode::InvalidParentType),
            "classConstant.nameType" => Some(IssueCode::InvalidConstantSelector),
            "classConstant.dynamicString" => Some(IssueCode::InvalidClassConstantOnString),
            "deadCode.unreachable" => Some(IssueCode::UnevaluatedCode),
            // These are public PHPStan rules, not native resolver diagnostics.
            "classConstant.nonObject" => None,
            "phpstan.dumpType" => None,
            identifier => panic!("unclassified oracle diagnostic: {identifier}: {error}"),
        };
        if let Some(code) = code {
            wanted.push((error[0].as_i64().unwrap(), code.as_str()));
        }
    }
    // Native mixed warnings distinguish genuine mixed from ErrorType and exempt $_ variables.
    for record in expected["expressions"].as_object().unwrap().values() {
        if record["assigned"]["mixed"] == true
            && !record["expression"].as_str().unwrap().starts_with("$_")
        {
            wanted.push((
                record["line"].as_i64().unwrap(),
                IssueCode::MixedAssignment.as_str(),
            ));
        }
    }
    actual.sort();
    wanted.sort();
    assert_eq!(
        actual, wanted,
        "complete native issues for {name}, native={native}: {:?}",
        result.issues
    );
}

#[test]
fn original_constant_failures_keep_error_and_mixed_values_and_assignments() {
    let expected = oracle();
    for (name, record) in expected["files"].as_object().unwrap() {
        if name == "controls.php" || name == "never.php" {
            continue;
        }
        for native in [false, true] {
            native_case(name, record, native, false);
        }
    }
}

#[test]
fn asymmetric_values_unions_and_single_operand_visits() {
    let expected = oracle();
    for native in [false, true] {
        native_case(
            "controls.php",
            &expected["files"]["controls.php"],
            native,
            false,
        );
    }
}

#[test]
fn dynamic_selector_values_keep_real_operand_termination() {
    let expected = oracle();
    for native in [false, true] {
        native_case("never.php", &expected["files"]["never.php"], native, false);
    }
}

#[test]
fn selector_analysis_failure_is_not_replaced_by_an_error_type_value() {
    let expected = oracle();
    native_case(
        "dynamic-invalid.php",
        &expected["files"]["dynamic-invalid.php"],
        false,
        true,
    );
}

#[test]
fn complete_public_diagnostics_match_the_pinned_oracle() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/failed-class-constants");
    let expected = oracle();
    for (name, record) in expected["files"].as_object().unwrap() {
        for level in [0, 8, 10] {
            let mut project = Project::new(vec![root.join(name)], Vec::new());
            project.php_version = 80500;
            project.level = level;
            project.options.set(
                pun_analyse::ProjectOptions::CHECK_MAYBE_UNDEFINED_VARIABLES,
                level >= 1,
            );
            let result = analyse_project(&project).unwrap();
            assert!(
                result.general.is_empty(),
                "{name}, level {level}: {:?}",
                result.general
            );
            let mut actual: Vec<_> = result
                .errors
                .into_iter()
                .map(|error| {
                    (
                        error.line,
                        error.identifier,
                        error.message,
                        error.tip,
                        error.can_be_ignored,
                    )
                })
                .collect();
            actual.sort();
            assert_eq!(
                serde_json::to_value(actual).unwrap(),
                record["diagnostics"][level.to_string()],
                "{name}, level {level}"
            );
        }
    }
}

#[test]
fn original_prerequisite_boundary_keeps_all_seven_complete_records() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/failed-class-constants/boundary-input");
    let options = pun_phpstan::config::Options {
        config: Some(root.join("phpstan.neon").to_string_lossy().into_owned()),
        ..pun_phpstan::config::Options::for_project(&root.to_string_lossy())
    };
    let config = pun_phpstan::config::resolve(&options).unwrap();
    let expected: Value = serde_json::from_str(include_str!(
        "fixtures/failed-class-constants/boundary-input/expected-phpstan.json"
    ))
    .unwrap();
    let mut actual: Vec<_> = pun_analyse::analyse(&root, &config)
        .unwrap()
        .into_iter()
        .map(|error| {
            json!({
                "file": Path::new(&error.file).file_name().unwrap().to_str().unwrap(),
                "line": error.line, "identifier": error.identifier,
                "message": error.message, "tip": error.tip,
            })
        })
        .collect();
    let mut wanted = expected["errors"].as_array().unwrap().clone();
    assert_eq!(wanted.len(), 7);
    actual.sort_by_key(Value::to_string);
    wanted.sort_by_key(Value::to_string);
    assert_eq!(actual, wanted);
}
