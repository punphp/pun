//! Rector's catch decoration is an observation, never a new reachable catch edge.

use std::borrow::Cow;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use mago_allocator::LocalArena;
use mago_analyzer::plugin::{
    HookContext, HookResult, PluginRegistry, Provider, ProviderMeta, StatementHook,
};
use mago_analyzer::settings::Settings;
use mago_analyzer::{Analyzer, analysis_result::AnalysisResult, artifacts::AnalysisArtifacts};
use mago_codex::ttype::TType;
use mago_codex::{populator::populate_codebase, scanner::scan_program};
use mago_database::file::File;
use mago_names::resolver::NameResolver;
use mago_span::{HasSpan, Span};
use mago_syntax::{
    cst::{Statement, TriviaKind},
    parser::parse_file,
};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

const INPUT: &[u8] = include_bytes!("fixtures/unmatched-catch-observation/input.php");

type Returns = Vec<(usize, Vec<String>, Vec<String>, Vec<String>)>;
struct FlowObserver(Arc<Mutex<Returns>>);
impl Provider for FlowObserver {
    fn meta() -> &'static ProviderMeta {
        static META: ProviderMeta =
            ProviderMeta::new("catch-flow", "catch-flow", "observed try boundaries");
        &META
    }
}
impl StatementHook for FlowObserver {
    fn after_statement(
        &self,
        statement: &Statement<'_>,
        context: &mut HookContext<'_, '_>,
    ) -> HookResult<()> {
        if let Statement::Try(_) = statement {
            self.0.lock().unwrap().push((
                statement.span().start.offset as usize,
                context
                    .artifacts()
                    .inferred_return_types
                    .iter()
                    .map(|ty| ty.get_id().to_string())
                    .collect(),
                context
                    .artifacts()
                    .inferred_yield_key_types
                    .iter()
                    .map(|ty| ty.get_id().to_string())
                    .collect(),
                context
                    .artifacts()
                    .inferred_yield_value_types
                    .iter()
                    .map(|ty| ty.get_id().to_string())
                    .collect(),
            ));
        }
        Ok(())
    }
}

fn analyze(dead: bool, native: bool, docs: bool) -> (AnalysisArtifacts, AnalysisResult, Returns) {
    let arena = LocalArena::new();
    let mut source = INPUT.to_vec();
    if !docs {
        let file = File::ephemeral(Cow::Borrowed(b"input.php"), Cow::Borrowed(INPUT));
        let program = parse_file(&arena, &file);
        for trivia in &program.trivia {
            if trivia.kind == TriviaKind::DocBlockComment {
                for byte in
                    &mut source[trivia.span.start.offset as usize..trivia.span.end.offset as usize]
                {
                    if !matches!(*byte, b'\n' | b'\r') {
                        *byte = b' ';
                    }
                }
            }
        }
    }
    let file = File::ephemeral(Cow::Borrowed(b"input.php"), Cow::Owned(source));
    let program = parse_file(&arena, &file);
    assert!(!program.has_errors());
    let names = NameResolver::new(&arena).resolve(program);
    let mut settings = Settings {
        analyze_dead_code: dead,
        ..Default::default()
    };
    settings.track_native_types = native;
    settings.track_variable_flow = true;
    let prelude = mago_prelude::Prelude::build();
    let mut codebase = prelude.metadata;
    codebase.extend(scan_program(
        &arena,
        &file,
        program,
        &names,
        settings.version,
    ));
    let mut references = prelude.symbol_references;
    populate_codebase(
        &mut codebase,
        &mut references,
        Default::default(),
        Default::default(),
    );
    let flow = Arc::<Mutex<Returns>>::default();
    let mut plugins = PluginRegistry::new();
    plugins.register_statement_hook(FlowObserver(Arc::clone(&flow)));
    let analyzer = Analyzer::new(&arena, &file, &names, &codebase, &plugins, settings);
    let mut result = AnalysisResult::new(references);
    let artifacts = analyzer
        .analyze_with_artifacts(program, &mut result)
        .unwrap();
    let returns = std::mem::take(&mut *flow.lock().unwrap());
    (artifacts, result, returns)
}

// Display adaptation only: retain every union member and literal value.
fn canonical(value: &str) -> String {
    let mut members: Vec<_> = value
        .split('|')
        .map(|value| {
            if value.parse::<i64>().is_ok() {
                format!("int({value})")
            } else if value.starts_with('\'') {
                format!("string({value})")
            } else {
                value.to_ascii_lowercase()
            }
        })
        .collect();
    members.sort();
    members.join("|")
}

#[test]
fn catch_observations_match_pinned_rector_without_changing_live_flow_or_native_availability() {
    let rector: Value = serde_json::from_slice(include_bytes!(
        "fixtures/unmatched-catch-observation/rector.json"
    ))
    .unwrap();
    let ordinary: Value = serde_json::from_slice(include_bytes!(
        "fixtures/unmatched-catch-observation/phpstan-2.3.5.json"
    ))
    .unwrap();
    assert_eq!(
        Sha256::digest(INPUT)
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect::<String>(),
        rector["input_sha256"]
    );
    assert_eq!(rector["input_sha256"], ordinary["input_sha256"]);
    let mut failures = Vec::new();
    for docs in [false, true] {
        let (off, off_result, off_flow) = analyze(false, false, docs);
        for native in [false, true] {
            let (on, result, flow) = analyze(true, native, docs);
            let mut actual = BTreeMap::new();
            let mut expected = BTreeMap::new();
            for (name, record) in rector["records"].as_object().unwrap() {
                let span = Span::dummy(
                    record["start"].as_u64().unwrap() as u32,
                    record["end"].as_u64().unwrap() as u32,
                );
                let value = if docs {
                    on.get_expression_type(&span)
                } else {
                    on.get_keep_void_expression_type(&span)
                }
                .map(|ty| ty.get_id().to_string());
                actual.insert(name, value.map(|ty| canonical(&ty)));
                expected.insert(
                    name,
                    Some(canonical(
                        record[if docs { "full" } else { "native" }]
                            .as_str()
                            .unwrap(),
                    )),
                );
                if let Some(original) = ordinary["records"].get(name) {
                    let found = (if docs {
                        off.get_expression_type(&span)
                    } else {
                        off.get_keep_void_expression_type(&span)
                    })
                    .map(|ty| canonical(&ty.get_id().to_string()));
                    let wanted = Some(canonical(
                        original[if docs { "full" } else { "native" }]
                            .as_str()
                            .unwrap(),
                    ));
                    if found != wanted {
                        failures.push(json!({"ordinary":name,"docs":docs,"native":native,"actual":found,"expected":wanted}));
                    }
                } else {
                    assert!(off.get_expression_type(&span).is_none(), "ordinary {name}");
                }
                // Try is unsupported in the partial native-expression lane. Never backfill it from full types.
                if name != "channels-before" {
                    assert!(
                        on.get_native_expression_type(&span).is_none(),
                        "native {name}"
                    );
                }
            }
            assert_eq!(
                format!("{:?}", result.issues),
                format!("{:?}", off_result.issues)
            );
            // Preserve every live boundary and the one additional, genuinely walked
            // nested try. Its private artifacts have no returns or yields.
            let source = std::str::from_utf8(INPUT).unwrap();
            let nested = source.find("try {\n            $value = 5;").unwrap();
            let outer = source.find("try {\n        $value = 4;").unwrap();
            // PHPStan 2.3.3+ (the ordinary run) also walks a catch no throw point reaches, whose boundaries, returns
            // and yields Rector's pinned PHPStan 2.2.7 never sees: the observed flow is the ordinary one without them.
            let mut flow = flow.clone();
            let position = flow
                .iter()
                .position(|(offset, ..)| *offset == nested)
                .unwrap();
            assert_eq!(
                flow.remove(position),
                (nested, Vec::new(), Vec::new(), Vec::new())
            );
            assert_eq!(flow[position].0, outer);
            let mut rest = off_flow.iter();
            for (offset, returns, yields, sent) in &flow {
                let (_, r, y, s) = rest.find(|(o, ..)| o == offset).unwrap_or_else(|| {
                    panic!("observed boundary {offset} is not an ordinary one, in order")
                });
                for (observed, ordinary) in [(returns, r), (yields, y), (sent, s)] {
                    assert!(
                        observed.iter().all(|t| ordinary.contains(t)),
                        "{offset}: {observed:?} not in {ordinary:?}"
                    );
                }
            }
            assert_eq!(
                on.variable_writes
                    .keys()
                    .collect::<std::collections::BTreeSet<_>>(),
                off.variable_writes.keys().collect()
            );
            if actual != expected {
                failures
                    .push(json!({"docs":docs,"native":native,"actual":actual,"expected":expected}));
            }
            println!(
                "catch-observations={}",
                json!({"docs":docs,"native":native,"actual":actual,"expected":expected})
            );
        }
    }
    assert!(failures.is_empty(), "{}", json!(failures));
}

#[test]
fn default_analyse_keeps_complete_phpstan_2_3_5_records() {
    let expected: Value = serde_json::from_slice(include_bytes!(
        "fixtures/unmatched-catch-observation/phpstan-2.3.5.json"
    ))
    .unwrap();
    let mut project = pun_analyse::Project::new(
        vec![
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("tests/fixtures/unmatched-catch-observation/input.php"),
        ],
        Vec::new(),
    );
    project.php_version = 80500;
    project.level = 0;
    let analysis = pun_analyse::analyse_project(&project).unwrap();
    let actual: Vec<_> = analysis
        .errors
        .into_iter()
        .map(|error| {
            json!([
                error.line,
                error.identifier,
                error.message,
                error.tip,
                error.can_be_ignored
            ])
        })
        .collect();
    assert_eq!(json!(actual), expected["errors"]);
    assert!(analysis.general.is_empty(), "{:?}", analysis.general);
}
