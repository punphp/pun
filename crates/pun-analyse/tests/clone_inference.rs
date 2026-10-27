//! Clone result types and operand evaluation through the public Analyzer.
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::path::Path;
use std::sync::{Arc, Mutex};

use mago_allocator::LocalArena;
use mago_analyzer::Analyzer;
use mago_analyzer::analysis_result::AnalysisResult;
use mago_analyzer::plugin::{ExpressionHook, HookContext, HookResult, PluginRegistry};
use mago_analyzer::settings::Settings;
use mago_codex::populator::populate_codebase;
use mago_codex::ttype::TType;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::callable::{TCallable, TCallableSignature};
use mago_codex::ttype::atomic::object::TObject;
use mago_codex::ttype::cast::cast_atomic_to_callable;
use mago_codex::ttype::union::TUnion;
use mago_database::file::File;
use mago_names::resolver::NameResolver;
use mago_span::{HasSpan, Position, Span};
use mago_syntax::cst::Expression;
use mago_syntax::parser::parse_file;
use serde_json::Value;

struct Observer(Arc<Mutex<BTreeMap<u32, usize>>>);
impl ExpressionHook for Observer {
    fn after_expression(
        &self,
        expr: &Expression<'_>,
        _: &mut HookContext<'_, '_>,
    ) -> HookResult<()> {
        *self
            .0
            .lock()
            .unwrap()
            .entry(expr.span().start.offset)
            .or_default() += 1;
        Ok(())
    }
}

fn describe(ty: &TUnion) -> String {
    let mut parts: Vec<_> = ty
        .types
        .iter()
        .map(|atomic| match atomic {
            TAtomic::GenericParameter(p) => {
                let bound = describe(&p.constraint);
                let name = p.parameter_name.to_string();
                let scope = p.defining_entity.id_word().to_string();
                if let Some(intersections) = &p.intersection_types {
                    assert!(p.constraint.is_mixed());
                    assert_eq!(intersections.as_slice(), &[TAtomic::Object(TObject::Any)]);
                    format!("object&{name} (function {scope}, argument)")
                } else {
                    format!("{name} of {bound} (function {scope}, argument)")
                }
            }
            TAtomic::Object(TObject::Enum(e)) => e.name.to_string(),
            TAtomic::Object(TObject::Named(n)) if n.is_static => {
                assert!(!n.is_this);
                format!("static({})", n.name)
            }
            TAtomic::Callable(c) if c.is_closure() => "Closure".to_owned(),
            TAtomic::Object(TObject::HasMethod(h)) => {
                assert!(h.has_method(b"__invoke"));
                let callable = &h.intersection_types.as_ref().unwrap()[0];
                if callable
                    == &TAtomic::Callable(TCallable::Signature(TCallableSignature::mixed(false)))
                {
                    "callable-object".to_owned()
                } else {
                    let id = callable.get_id().to_string();
                    format!(
                        "{}&object",
                        id.strip_prefix('(').unwrap().strip_suffix(')').unwrap()
                    )
                }
            }
            _ => atomic.get_id().to_string(),
        })
        .collect();
    parts.sort();
    parts.join("|")
}

#[test]
fn clone_results_and_single_operand_walk_match_public_phpstan_controls() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/clone");
    let source = std::fs::read(root.join("values.php")).unwrap();
    let oracle: Value =
        serde_json::from_slice(&std::fs::read(root.join("expected.json")).unwrap()).unwrap();
    let older: Value =
        serde_json::from_slice(&std::fs::read(root.join("expected-227.json")).unwrap()).unwrap();
    assert_eq!(oracle, older);
    let file = File::ephemeral(Cow::Borrowed(b"values.php"), Cow::Owned(source));
    let arena = LocalArena::new();
    let program = parse_file(&arena, &file);
    assert!(!program.has_errors());
    let names = NameResolver::new(&arena).resolve(program);
    let prelude = mago_prelude::Prelude::build();
    let mut codebase = prelude.metadata;
    codebase.extend(mago_codex::scanner::scan_program(
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
    let visits = Arc::new(Mutex::new(BTreeMap::new()));
    let mut plugins = PluginRegistry::new();
    plugins.register_expression_hook(Observer(Arc::clone(&visits)));
    let mut result = AnalysisResult::new(references);
    let artifacts = Analyzer::new(
        &arena,
        &file,
        &names,
        &codebase,
        &plugins,
        Settings::default(),
    )
    .analyze_with_artifacts(program, &mut result)
    .unwrap();
    let mut cases: Vec<_> = oracle["expressions"].as_object().unwrap().iter().collect();
    cases.sort_by_key(|(offset, _)| offset.parse::<u32>().unwrap());
    for (offset, expected) in cases {
        let start: u32 = offset.parse().unwrap();
        let expression = expected["expression"].as_str().unwrap();
        let span = Span::new(
            file.id,
            Position::new(start),
            Position::new(start + expression.len() as u32),
        );
        let ty = artifacts.get_expression_type(&span).unwrap();
        let mut want = expected["type"]
            .as_str()
            .unwrap()
            .replace("*NEVER*", "never");
        // Codex canonicalizes function scope names, but template parameter names remain case-sensitive.
        if let Some((prefix, scope)) = want.split_once(" (function ") {
            want = format!("{prefix} (function {}", scope.to_ascii_lowercase());
        }
        let mut parts: Vec<_> = want.split('|').collect();
        parts.sort();
        want = parts.join("|");
        let got = describe(ty);
        assert_eq!(got, want, "{expected}; {ty:?}");
        assert_eq!(visits.lock().unwrap().get(&start), Some(&1), "{expression}");
        if let Some(operand) = expected.get("operand") {
            let offset = operand["offset"].as_u64().unwrap() as u32;
            let operand_span = Span::new(
                file.id,
                Position::new(offset),
                Position::new(offset + operand["length"].as_u64().unwrap() as u32),
            );
            let input = artifacts.get_expression_type(&operand_span).unwrap();
            assert_eq!(
                visits.lock().unwrap().get(&offset),
                Some(&1),
                "operand of {expression}"
            );
            if operand["type"] == "mixed" {
                assert!(
                    matches!(input.types.as_ref(), [TAtomic::Mixed(m)] if Value::Bool(m.is_explicit()) == operand["explicit"])
                );
            }
            if let [TAtomic::Object(TObject::HasMethod(h))] = ty.types.as_ref() {
                assert_eq!(h.intersection_types.as_deref(), Some(input.types.as_ref()));
                assert_eq!(
                    cast_atomic_to_callable(&ty.types[0], &codebase, None),
                    cast_atomic_to_callable(&input.types[0], &codebase, None),
                );
            }
            if expected["type"] == "Closure" {
                assert_eq!(ty, input);
            }
        }
    }
}

#[test]
fn neighboring_clone_and_operand_diagnostics_match_phpstan() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/clone");
    let oracle: Value =
        serde_json::from_slice(&std::fs::read(root.join("expected.json")).unwrap()).unwrap();
    let mut project = pun_analyse::Project::new(vec![root.join("values.php")], Vec::new());
    project.level = 7;
    let mut got: Vec<_> = pun_analyse::run(&project)
        .unwrap()
        .into_iter()
        .filter(|error| {
            matches!(
                error.identifier.as_str(),
                "clone.nonObject" | "function.notFound"
            )
        })
        .map(|error| {
            serde_json::json!([
                error.line,
                error.identifier,
                error.message,
                error.tip,
                error.can_be_ignored
            ])
        })
        .collect();
    let mut expected: Vec<_> = oracle["errors"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|error| {
            matches!(
                error[1].as_str().unwrap(),
                "clone.nonObject" | "function.notFound"
            )
        })
        .cloned()
        .collect();
    got.sort_by_key(Value::to_string);
    expected.sort_by_key(Value::to_string);
    assert_eq!(got, expected);
}
