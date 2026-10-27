//! Source-owned ASCII casing lattice; expectations follow PHP's byte case maps.

use std::collections::{BTreeSet, HashSet};

use mago_codex::metadata::CodebaseMetadata;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_codex::ttype::atomic::scalar::string::{TString, TStringCasing as C};
use mago_codex::ttype::combiner::CombinerOptions;
use mago_codex::ttype::comparator::{ComparisonResult, union_comparator};
use mago_codex::ttype::union::TUnion;
use mago_codex::ttype::{TType, combine_union_types, get_string_with_props, intersect_union_types};
use mago_word::word;

fn union(string: TString) -> TUnion {
    TUnion::from_atomic(TAtomic::Scalar(TScalar::String(string)))
}

fn general(casing: C) -> TString {
    TString::general_with_props(false, false, false, false, casing)
}

#[test]
fn intersections_and_joins_have_independent_guarantees() {
    use C::{Both as B, Lowercase as L, Unspecified as N, Uppercase as U};
    let states = [N, L, U, B];
    let meets = [[N, L, U, B], [L, L, B, B], [U, B, U, B], [B, B, B, B]];
    let joins = [[N, N, N, N], [N, L, N, L], [N, N, U, U], [N, L, U, B]];
    let codebase = CodebaseMetadata::default();
    for (i, left) in states.into_iter().enumerate() {
        for (j, right) in states.into_iter().enumerate() {
            let a = general(left);
            let b = general(right);
            assert_eq!(left.join(right), joins[i][j]);
            assert_eq!(a.intersect(&b), Some(general(meets[i][j])));
            let result = intersect_union_types(&union(a), &union(b), &codebase).unwrap();
            assert_eq!(result.get_single_string().unwrap().casing, meets[i][j]);
            let result =
                combine_union_types(&union(a), &union(b), &codebase, CombinerOptions::default());
            // Exact unions retain incomparable alternatives, unlike shared casing guarantees.
            let expected = if matches!((left, right), (L, U) | (U, L)) {
                TUnion::from_vec(vec![
                    TAtomic::Scalar(TScalar::String(general(L))),
                    TAtomic::Scalar(TScalar::String(general(U))),
                ])
            } else {
                union(general(joins[i][j]))
            };
            assert_eq!(result, expected, "{left:?} union {right:?}");
            // A value fits a container iff it has every guarantee the container requires.
            assert_eq!(
                union_comparator::is_contained_by(
                    &codebase,
                    &union(a),
                    &union(b),
                    false,
                    false,
                    false,
                    &mut ComparisonResult::new(),
                ),
                meets[i][j] == left,
                "{left:?} into {right:?}",
            );
        }
    }
}

#[test]
fn scalar_containers_absorb_every_incomparable_string_alternative() {
    let codebase = CodebaseMetadata::default();
    let strings = TUnion::from_vec(vec![
        TAtomic::Scalar(TScalar::String(general(C::Lowercase))),
        TAtomic::Scalar(TScalar::String(general(C::Uppercase))),
    ]);
    for container in [TScalar::Generic, TScalar::ArrayKey] {
        let container = TUnion::from_atomic(TAtomic::Scalar(container));
        for (left, right) in [(&strings, &container), (&container, &strings)] {
            // PHPStan's scalar and array-key contain every string, not just the general string bucket.
            assert_eq!(
                combine_union_types(left, right, &codebase, CombinerOptions::default()),
                container,
            );
        }
    }
}

#[test]
fn literal_absorption_and_intersection_use_ascii_not_alphabetic_bytes() {
    let codebase = CodebaseMetadata::default();
    for (bytes, casing, allowed_constraints) in [
        (&b""[..], C::Both, [true, true, true]),
        (&b"0! -"[..], C::Both, [true, true, true]),
        (&b"\xff\xc3\xa9"[..], C::Both, [true, true, true]),
        (&b"e!0"[..], C::Lowercase, [false, true, false]),
        (&b"E!0"[..], C::Uppercase, [false, false, true]),
        (&b"eE!"[..], C::Unspecified, [false, false, false]),
    ] {
        let literal = TString::known_literal(word(bytes));
        assert_eq!(literal.casing, casing);
        assert_eq!(literal.without_literal().casing, casing);
        assert_eq!(literal.with_unspecified_literal().casing, casing);
        for (constraint, allowed) in [C::Both, C::Lowercase, C::Uppercase]
            .into_iter()
            .zip(allowed_constraints)
        {
            let generic = general(constraint);
            for (left, right) in [(literal, generic), (generic, literal)] {
                assert_eq!(left.intersect(&right), allowed.then_some(literal));
                assert_eq!(
                    intersect_union_types(&union(left), &union(right), &codebase),
                    allowed.then(|| union(literal))
                );
                let result = combine_union_types(
                    &union(left),
                    &union(right),
                    &codebase,
                    CombinerOptions::default(),
                );
                if allowed {
                    assert_eq!(result, union(generic));
                }
            }
        }
    }
    // Numeric syntax alone cannot promise either casing: exponent letters differ.
    assert!(!TString::numeric().is_lowercase());
    assert!(!TString::numeric().is_uppercase());
    assert_eq!(TString::known_literal(word("1e2")).casing, C::Lowercase);
    assert_eq!(TString::known_literal(word("1E2")).casing, C::Uppercase);
    // A legacy single guarantee must not hide the known literal's other guarantee.
    for casing in [C::Unspecified, C::Lowercase, C::Uppercase, C::Both] {
        let literal = TString {
            casing,
            ..TString::known_literal(word("0!"))
        };
        assert!(literal.is_lowercase());
        assert!(literal.is_uppercase());
    }
}

#[test]
fn casing_constraints_apply_to_class_names_and_possible_identity() {
    use std::borrow::Cow;

    use mago_allocator::LocalArena;
    use mago_codex::scanner::scan_program;
    use mago_codex::ttype::atomic::scalar::class_like_string::TClassLikeString;
    use mago_database::file::File;
    use mago_names::resolver::NameResolver;
    use mago_php_version::PHPVersion;
    use mago_syntax::parser::parse_file;

    let arena = LocalArena::new();
    let file = File::ephemeral(
        Cow::Borrowed(b"functions.php"),
        Cow::Borrowed("<?php function é(): void {} function abc(): void {}".as_bytes()),
    );
    let program = parse_file(&arena, &file);
    let names = NameResolver::new(&arena).resolve(program);
    let codebase = scan_program(&arena, &file, program, &names, PHPVersion::PHP85);
    for (name, allowed) in [("é", true), ("abc", false)] {
        let class_name = TUnion::from_atomic(TAtomic::Scalar(TScalar::ClassLikeString(
            TClassLikeString::literal(word(name)),
        )));
        for container in [general(C::Both), TString::callable_with_casing(C::Both)] {
            assert_eq!(
                union_comparator::is_contained_by(
                    &codebase,
                    &class_name,
                    &union(container),
                    false,
                    false,
                    false,
                    &mut ComparisonResult::new()
                ),
                allowed
            );
        }
    }
    let unknown = TUnion::from_atomic(TAtomic::Scalar(TScalar::ClassLikeString(
        TClassLikeString::class_string(),
    )));
    assert!(!union_comparator::is_contained_by(
        &codebase,
        &unknown,
        &union(general(C::Both)),
        false,
        false,
        false,
        &mut ComparisonResult::new()
    ));
    for (bytes, allowed) in [("!0", true), ("e", false), ("E", false), ("é", true)] {
        let value = union(TString::known_literal(word(bytes)));
        let both = union(general(C::Both));
        assert_eq!(
            union_comparator::can_expression_types_be_identical(
                &codebase, &value, &both, false, false
            ),
            allowed
        );
        assert_eq!(
            union_comparator::can_expression_types_be_identical(
                &codebase, &both, &value, false, false
            ),
            allowed
        );
    }
}

#[test]
fn constructors_ids_equality_hash_order_and_serde_preserve_both() {
    for numeric in [false, true] {
        for truthy in [false, true] {
            for non_empty in [false, true] {
                for callable in [false, true] {
                    let strings: Vec<_> = [C::Unspecified, C::Lowercase, C::Uppercase, C::Both]
                        .into_iter()
                        .map(|casing| {
                            TString::general_with_props(
                                numeric, truthy, non_empty, callable, casing,
                            )
                        })
                        .collect();
                    assert_eq!(strings.iter().collect::<HashSet<_>>().len(), 4);
                    assert_eq!(strings.iter().collect::<BTreeSet<_>>().len(), 4);
                    assert_eq!(
                        strings
                            .iter()
                            .map(TType::get_id)
                            .collect::<HashSet<_>>()
                            .len(),
                        4
                    );
                    for string in strings {
                        let json = serde_json::to_string(&string).unwrap();
                        assert_eq!(serde_json::from_str::<TString>(&json).unwrap(), string);
                        if string.casing == C::Both || numeric {
                            let result = get_string_with_props(
                                numeric,
                                truthy,
                                non_empty,
                                callable,
                                string.casing,
                            );
                            assert_eq!(result.get_single_string().unwrap().casing, string.casing);
                        }
                    }
                }
            }
        }
    }
    assert_eq!(serde_json::to_string(&C::Both).unwrap(), "\"Both\"");
    assert_eq!(
        serde_json::from_str::<C>("\"Lowercase\"").unwrap(),
        C::Lowercase
    );
    assert_eq!(
        serde_json::from_str::<C>("\"Uppercase\"").unwrap(),
        C::Uppercase
    );
    assert_eq!(
        serde_json::from_str::<C>("\"Unspecified\"").unwrap(),
        C::Unspecified
    );

    // PHPStan accepts both as nonempty, but keeps their independent accessories distinct.
    let bare = TString::numeric();
    let explicit = TString::general_with_props(true, false, true, false, C::Unspecified);
    assert!(bare.is_non_empty());
    assert!(explicit.is_non_empty());
    assert_ne!(bare, explicit);
    assert_ne!(bare.get_id(), explicit.get_id());
    let codebase = CodebaseMetadata::default();
    for value in [bare, explicit] {
        assert!(union(value).is_non_empty_string());
        assert!(union_comparator::is_contained_by(
            &codebase,
            &union(value),
            &union(TString::non_empty()),
            false,
            false,
            false,
            &mut ComparisonResult::new(),
        ));
    }
}

#[test]
fn scanner_generalizes_long_literals_without_erasing_either_guarantee() {
    use std::borrow::Cow;

    use mago_allocator::LocalArena;
    use mago_codex::scanner::{inference::get_platform_constant_type, scan_program};
    use mago_database::file::File;
    use mago_names::resolver::NameResolver;
    use mago_php_version::PHPVersion;
    use mago_syntax::parser::parse_file;

    for (value, expected) in [
        ("!0", C::Both),
        ("e!", C::Lowercase),
        ("E!", C::Uppercase),
        ("eE", C::Unspecified),
    ] {
        let source = format!(
            "<?php const VALUE = '{}'; const JOINED = '{}' . '!';",
            value.repeat(501),
            value.repeat(501)
        );
        let arena = LocalArena::new();
        let file = File::ephemeral(
            Cow::Borrowed(b"casing.php"),
            Cow::Owned(source.into_bytes()),
        );
        let program = parse_file(&arena, &file);
        assert!(!program.has_errors());
        let names = NameResolver::new(&arena).resolve(program);
        let codebase = scan_program(&arena, &file, program, &names, PHPVersion::PHP85);
        let value = codebase
            .get_constant(b"VALUE")
            .unwrap()
            .inferred_type
            .as_ref()
            .unwrap();
        assert_eq!(value.get_single_string().unwrap().casing, expected);
        assert_eq!(
            value.get_single_string().unwrap().get_known_literal_value(),
            None
        );
        let joined = codebase
            .get_constant(b"JOINED")
            .unwrap()
            .inferred_type
            .as_ref()
            .unwrap();
        assert_eq!(joined.get_single_string().unwrap().casing, expected);
    }
    let separators = get_platform_constant_type(b"DIRECTORY_SEPARATOR").unwrap();
    assert!(separators.types.iter().all(|ty| matches!(ty,
        TAtomic::Scalar(TScalar::String(s)) if s.is_lowercase() && s.is_uppercase())));
}

#[test]
fn psl_splice_retains_both_but_unicode_case_maps_do_not() {
    use std::borrow::Cow;

    use mago_allocator::LocalArena;
    use mago_analyzer::{
        Analyzer, analysis_result::AnalysisResult, plugin::PluginRegistry, settings::Settings,
    };
    use mago_codex::{
        populator::populate_codebase, reference::SymbolReferences, scanner::scan_program,
    };
    use mago_database::file::File;
    use mago_names::resolver::NameResolver;
    use mago_php_version::PHPVersion;
    use mago_syntax::parser::parse_file;

    let source = r#"<?php
namespace Psl\Str;
function splice(string $string, string $replacement, int $offset): string { return ''; }
function lowercase(string $string): string { return ''; }
namespace Psl\Str\Grapheme;
function uppercase(string $string): string { return ''; }
namespace Probe;
/** @param lowercase-string&uppercase-string $both */
function probe(string $both): void {
    $a = \Psl\Str\splice($both, $both, 0);
    $b = \Psl\Str\splice('0!', '!0', 0);
    $c = \Psl\Str\lowercase('K');
    $d = \Psl\Str\Grapheme\uppercase('ß');
}
"#;
    let arena = LocalArena::new();
    let file = File::ephemeral(
        Cow::Borrowed(b"psl-casing.php"),
        Cow::Borrowed(source.as_bytes()),
    );
    let program = parse_file(&arena, &file);
    let names = NameResolver::new(&arena).resolve(program);
    let mut codebase = scan_program(&arena, &file, program, &names, PHPVersion::PHP85);
    let mut references = SymbolReferences::new();
    populate_codebase(
        &mut codebase,
        &mut references,
        Default::default(),
        Default::default(),
    );
    let registry = PluginRegistry::with_library_providers();
    let analyzer = Analyzer::new(
        &arena,
        &file,
        &names,
        &codebase,
        &registry,
        Settings::default(),
    );
    let artifacts = analyzer
        .analyze_with_artifacts(program, &mut AnalysisResult::new(references))
        .unwrap();
    for (call, expected) in [
        (r"\Psl\Str\splice($both, $both, 0)", C::Both),
        (r"\Psl\Str\splice('0!', '!0', 0)", C::Both),
        (r"\Psl\Str\lowercase('K')", C::Lowercase),
        (r"\Psl\Str\Grapheme\uppercase('ß')", C::Uppercase),
    ] {
        let start = source.find(call).unwrap() as u32;
        let ty = artifacts
            .expression_types
            .get(&(start, start + call.len() as u32))
            .unwrap();
        assert_eq!(ty.get_single_string().unwrap().casing, expected, "{call}");
    }
}
