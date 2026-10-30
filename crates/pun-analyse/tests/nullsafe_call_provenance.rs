//! PHPStan 2.3.1 keeps genuine null returns distinct from a nullsafe short circuit.
use std::borrow::Cow;

use mago_allocator::LocalArena;
use mago_analyzer::{
    Analyzer, analysis_result::AnalysisResult, observations::CallScopeRequest,
    plugin::PluginRegistry, settings::Settings,
};
use mago_codex::{
    populator::populate_codebase, reference::SymbolReferences, scanner::scan_program,
    ttype::atomic::TAtomic,
};
use mago_database::file::File;
use mago_names::resolver::NameResolver;
use mago_span::{Position, Span};
use mago_syntax::parser::parse_file;
use serde_json::Value;

#[test]
fn declared_null_and_nullable_template_bounds_are_not_short_circuit_provenance() {
    const SOURCE: &[u8] =
        include_bytes!("fixtures/call-scope-observations/nullsafe-call-provenance.php");
    let oracle: Value = serde_json::from_str(include_str!(
        "fixtures/call-scope-observations/nullsafe-call-provenance-expected.json"
    ))
    .unwrap();
    for native in [false, true] {
        for observe in [false, true] {
            let arena = LocalArena::new();
            let file = File::ephemeral(
                Cow::Borrowed(b"nullsafe-call-provenance.php"),
                Cow::Borrowed(SOURCE),
            );
            let program = parse_file(&arena, &file);
            assert!(!program.has_errors());
            let names = NameResolver::new(&arena).resolve(program);
            let mut settings = Settings {
                track_native_types: native,
                ..Settings::default()
            };
            if observe {
                for (position, call) in oracle["calls"].as_object().unwrap() {
                    let start = position.parse::<u32>().unwrap();
                    settings.call_scope_observations.push(CallScopeRequest {
                        call: Span::new(
                            file.id,
                            Position::new(start),
                            Position::new(
                                start + call["expression"].as_str().unwrap().len() as u32,
                            ),
                        ),
                        expressions: call["names"][0]["types"]
                            .as_array()
                            .unwrap()
                            .iter()
                            .map(|expression| {
                                Span::new(
                                    file.id,
                                    Position::new(expression["start"].as_u64().unwrap() as u32),
                                    Position::new(expression["end"].as_u64().unwrap() as u32),
                                )
                            })
                            .collect(),
                    });
                }
            }
            let mut codebase = scan_program(&arena, &file, program, &names, settings.version);
            let mut references = SymbolReferences::new();
            populate_codebase(
                &mut codebase,
                &mut references,
                Default::default(),
                Default::default(),
            );
            let registry = PluginRegistry::new();
            let mut result = AnalysisResult::new(references);
            let artifacts = Analyzer::new(&arena, &file, &names, &codebase, &registry, settings)
                .analyze_with_artifacts(program, &mut result)
                .unwrap();
            assert_eq!(
                artifacts.call_scope_observations.len(),
                if observe {
                    oracle["calls"].as_object().unwrap().len()
                } else {
                    0
                }
            );
            for (call, introduced) in [
                ("$sink?->$name()", false),
                ("$sink?->voidOnly()", true),
                ("$sink?->nullOnly()", false),
                ("$sink?->templateBound()", false),
            ] {
                let start = SOURCE
                    .windows(call.len())
                    .position(|bytes| bytes == call.as_bytes())
                    .unwrap() as u32;
                let span = Span::new(
                    file.id,
                    Position::new(start),
                    Position::new(start + call.len() as u32),
                );
                // Both fixture formals explicitly include null, independently of their ensured twin.
                let receiver = &artifacts.nullsafe_method_receiver_types[&span];
                assert_eq!(receiver.types.len(), 2, "{call}: {receiver:?}");
                assert!(
                    receiver.types.contains(&TAtomic::Null),
                    "{call}: {receiver:?}"
                );
                let raw = artifacts.get_raw_expression_type(&span).unwrap();
                assert_eq!(
                    raw.has_nullsafe_null(),
                    introduced,
                    "native={native} observe={observe}: {call}: {raw:?}"
                );
                if call == "$sink?->$name()" {
                    assert_eq!(raw.types.len(), 2);
                    assert!(raw.types.contains(&TAtomic::Null));
                    assert!(raw.types.contains(&TAtomic::Void));
                    assert_eq!(
                        artifacts.get_expression_type(&span).unwrap().types.as_ref(),
                        &[TAtomic::Null]
                    );
                }
            }
        }
    }
}
