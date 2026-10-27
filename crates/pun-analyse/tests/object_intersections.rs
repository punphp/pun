//! PHPStan 2.3.1 object intersections and mixed exclusions, through public analysis and type APIs.

use std::path::Path;

use serde_json::Value;

fn compare(file: &str, capture: &str) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/object-intersections");
    let mut project = pun_analyse::Project::new(vec![root.join(file)], vec![]);
    project.php_version = 80400;
    let got: Vec<_> = pun_analyse::run(&project)
        .unwrap()
        .into_iter()
        .map(|e| serde_json::json!([e.line, e.identifier, e.message, e.tip, e.can_be_ignored]))
        .collect();
    let oracle: Value = serde_json::from_str(capture).unwrap();
    assert!(oracle["errors"].as_array().unwrap().is_empty());
    let want: Vec<_> = oracle["files"]
        .as_object()
        .unwrap()
        .values()
        .flat_map(|f| f["messages"].as_array().unwrap())
        .map(|e| {
            serde_json::json!([
                e["line"],
                e["identifier"],
                e["message"],
                e.get("tip").unwrap_or(&Value::Null),
                e["ignorable"]
            ])
        })
        .collect();
    assert_eq!(got, want, "{file}");
}

#[test]
fn conditional_mixed_exclusions_match_phpstan() {
    compare(
        "flow.php",
        include_str!("fixtures/object-intersections/flow.json"),
    );
}

#[test]
fn genuine_intersections_keep_inheritance_generics_and_receiver_identity() {
    compare(
        "objects.php",
        include_str!("fixtures/object-intersections/objects.json"),
    );
}

#[test]
fn exact_instantiations_keep_distinct_classes_and_receiver_producers() {
    // Exercise the public Analyzer-backed API: Pun's separate PHPStan Type::Object
    // representation cannot carry the per-value final override into later unions.
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/object-intersections");
    let mut project = pun_analyse::Project::new(vec![root.join("exact.php")], vec![]);
    project.php_version = 80400;
    let got: Vec<_> = pun_analyse::run_mago(&project, None)
        .unwrap()
        .dumped
        .into_iter()
        .map(|dump| (dump.line, dump.type_id))
        .collect();
    let oracle: Value =
        serde_json::from_str(include_str!("fixtures/object-intersections/exact.json")).unwrap();
    assert!(oracle["errors"].as_array().unwrap().is_empty());
    let want: Vec<_> = oracle["files"]
        .as_object()
        .unwrap()
        .values()
        .flat_map(|file| file["messages"].as_array().unwrap())
        .map(|message| {
            let phpstan = message["message"].as_str().unwrap();
            (
                message["line"].as_u64().unwrap() as u32,
                phpstan
                    .strip_prefix("Dumped type: ")
                    .unwrap()
                    .replace("static(Receiver)", "Receiver&static"),
            )
        })
        .collect();
    assert_eq!(got, want);
}

#[test]
fn object_subtractions_survive_narrowing_copies_queries_and_scope_merges() {
    compare(
        "subtraction.php",
        include_str!("fixtures/object-intersections/subtraction.json"),
    );
}

#[test]
fn operand_walk_branch_transfer_matches_phpstan() {
    compare(
        "transfer.php",
        include_str!("fixtures/object-intersections/transfer.json"),
    );
}

#[test]
fn operand_created_locals_and_iterated_values_match_phpstan() {
    compare(
        "created-locals.php",
        include_str!("fixtures/object-intersections/created-locals.json"),
    );
}

#[test]
fn final_object_alternatives_remove_only_the_reintroduced_exclusion() {
    compare(
        "final-alternatives.php",
        include_str!("fixtures/object-intersections/final-alternatives.json"),
    );
}

#[test]
fn excluded_objects_retain_generic_receiver_identity_and_queries() {
    compare(
        "queries.php",
        include_str!("fixtures/object-intersections/queries.json"),
    );
}

#[test]
fn union_absorption_checks_the_primary_generic_and_receiver_constraints() {
    compare(
        "constraints.php",
        include_str!("fixtures/object-intersections/constraints.json"),
    );
}

#[test]
fn raw_union_preserves_distinct_primary_parameters_and_receiver_identity() {
    use mago_codex::ttype::{
        atomic::{
            TAtomic,
            object::{TObject, named::TNamedObject},
        },
        combine_union_types, get_int, get_string,
        union::TUnion,
    };

    let codebase = scanned(include_bytes!(
        "fixtures/object-intersections/constraints.php"
    ));
    let boxed = |parameter| {
        let mut object =
            TNamedObject::new_with_type_parameters(mago_word::word("Box"), Some(vec![parameter]));
        object.intersection_types = Some(Box::new(vec![TAtomic::Object(TObject::Named(
            TNamedObject::new(mago_word::word("A")),
        ))]));
        TAtomic::Object(TObject::Named(object))
    };
    let int_box = boxed(get_int());
    let string_box = boxed(get_string());
    // PHPStan's first dump in constraints.json retains both invariant Box alternatives.
    for (left, right) in [
        (int_box.clone(), string_box.clone()),
        (string_box.clone(), int_box.clone()),
    ] {
        let result = combine_union_types(
            &TUnion::from_atomic(left),
            &TUnion::from_atomic(right),
            &codebase,
            Default::default(),
        );
        assert_eq!(result.types.len(), 2);
        assert!(result.types.contains(&int_box));
        assert!(result.types.contains(&string_box));
    }

    // PHPStan 2.3.1's declared self/static/$this parameter unions select the
    // least restrictive receiver in both orders, independently of new self's producer.
    // Use the scanned class's source spelling and its lowercase symbol-table key.
    for name in ["Receiver", "receiver"] {
        let plain = TAtomic::Object(TObject::Named(TNamedObject::new(mago_word::word(name))));
        let late = TAtomic::Object(TObject::Named(TNamedObject::new_static(mago_word::word(
            name,
        ))));
        let this = TAtomic::Object(TObject::Named(TNamedObject::new_this(mago_word::word(
            name,
        ))));
        for (left, right, expected) in [
            (&plain, &late, &plain),
            (&plain, &this, &plain),
            (&late, &this, &late),
        ] {
            for (left, right) in [(left, right), (right, left)] {
                let result = combine_union_types(
                    &TUnion::from_atomic(left.clone()),
                    &TUnion::from_atomic(right.clone()),
                    &codebase,
                    Default::default(),
                );
                assert_eq!(result.types.as_ref(), &[expected.clone()]);
            }
        }
    }
}

#[test]
fn exact_objects_keep_semantic_identity_without_changing_printed_class_names() {
    use mago_codex::ttype::{
        TType,
        atomic::{
            TAtomic,
            object::{TObject, named::TNamedObject},
        },
        combine_union_types,
        union::TUnion,
    };
    use std::collections::HashSet;

    let codebase = scanned(include_bytes!("fixtures/object-intersections/exact.php"));
    let named = |object| TAtomic::Object(TObject::Named(object));
    let broad = named(TNamedObject::new(mago_word::word("Service")));
    let exact = named(TNamedObject::new(mago_word::word("Service")).with_is_exact(true));
    let child = named(TNamedObject::new(mago_word::word("Child")).with_is_exact(true));
    let late = named(TNamedObject::new_static(mago_word::word("Service")));
    let this = named(TNamedObject::new_this(mago_word::word("Service")));
    assert_ne!(broad, exact);
    assert_eq!(HashSet::from([broad.clone(), exact.clone()]).len(), 2);
    assert_eq!(exact.get_id().as_str(), Some("Service"));
    assert_eq!(exact.get_pretty_id_with_indent(0).as_str(), Some("Service"));
    assert!(
        !codebase
            .get_class_like(b"Service")
            .unwrap()
            .flags
            .is_final()
    );

    // PHPStan's public ObjectType(asFinal)/StaticType/ThisType union matrix.
    for (left, right, expected) in [
        (&broad, &exact, vec![broad.clone()]),
        (&broad, &child, vec![broad.clone()]),
        (&exact, &child, vec![exact.clone(), child.clone()]),
        (&exact, &late, vec![late.clone()]),
        (&exact, &this, vec![exact.clone(), this.clone()]),
        (&child, &late, vec![late.clone()]),
        (&child, &this, vec![child.clone(), this.clone()]),
    ] {
        for (left, right) in [(left, right), (right, left)] {
            assert_eq!(
                combine_union_types(
                    &TUnion::from_atomic(left.clone()),
                    &TUnion::from_atomic(right.clone()),
                    &codebase,
                    Default::default()
                ),
                TUnion::from_vec(expected.clone()),
            );
        }
    }

    // Internal structural identity retains nested exactness even when IDs coincide.
    // PHPStan's GenericObjectType equality ignores this reflection override and prints
    // the same Box<Service>; this checks metadata retention, not its alternative count.
    let boxed = |parameter| {
        named(TNamedObject::new_with_type_parameters(
            mago_word::word("Box"),
            Some(vec![TUnion::from_atomic(parameter)]),
        ))
    };
    let exact_box = boxed(exact.clone());
    let broad_box = boxed(broad);
    for (left, right) in [(&exact_box, &broad_box), (&broad_box, &exact_box)] {
        let result = combine_union_types(
            &TUnion::from_atomic(left.clone()),
            &TUnion::from_atomic(right.clone()),
            &codebase,
            Default::default(),
        );
        assert_eq!(result.types.len(), 2);
        assert!(result.types.contains(&exact_box));
        assert!(result.types.contains(&broad_box));
        assert_eq!(result.get_id().as_str(), Some("Box<Service>"));
    }

    let covariant = |parameter| {
        named(TNamedObject::new_with_type_parameters(
            mago_word::word("CovariantBox"),
            Some(vec![parameter]),
        ))
    };
    let broad_box = covariant(mago_codex::ttype::get_int());
    let literal_box = covariant(mago_codex::ttype::get_literal_int(1));
    // The final exact.php dump is CovariantBox<int>, not two alternatives.
    for (left, right) in [(&broad_box, &literal_box), (&literal_box, &broad_box)] {
        let result = combine_union_types(
            &TUnion::from_atomic(left.clone()),
            &TUnion::from_atomic(right.clone()),
            &codebase,
            Default::default(),
        );
        assert_eq!(result.types.as_ref(), &[broad_box.clone()]);
    }

    // PHPStan retains unrelated generic classes and non-class/unknown nominal types.
    // Moving named objects out of printed-ID buckets must not lose these alternatives.
    for other in [
        exact_box,
        named(TNamedObject::new(mago_word::word("External"))),
        named(TNamedObject::new(mago_word::word("Marker"))),
    ] {
        for (left, right) in [(&broad_box, &other), (&other, &broad_box)] {
            let result = combine_union_types(
                &TUnion::from_atomic(left.clone()),
                &TUnion::from_atomic(right.clone()),
                &codebase,
                Default::default(),
            );
            assert_eq!(result.types.len(), 2);
            assert!(result.types.contains(&broad_box));
            assert!(result.types.contains(&other));
        }
    }

    let mut serialized = serde_json::to_value(&exact).unwrap();
    assert_eq!(
        serde_json::from_value::<TAtomic>(serialized.clone()).unwrap(),
        exact
    );
    serialized["Object"]["Named"]
        .as_object_mut()
        .unwrap()
        .remove("is_exact");
    let legacy: TAtomic = serde_json::from_value(serialized).unwrap();
    assert!(!matches!(legacy, TAtomic::Object(TObject::Named(object)) if object.is_exact));
}

#[test]
fn named_instantiations_retain_exactness_in_full_and_native_artifacts() {
    use mago_allocator::LocalArena;
    use mago_analyzer::{
        Analyzer, analysis_result::AnalysisResult, plugin::PluginRegistry, settings::Settings,
    };
    use mago_codex::{
        populator::populate_codebase, reference::SymbolReferences, scanner::scan_program,
    };
    use mago_database::file::File;
    use mago_names::resolver::NameResolver;
    use mago_span::{Position, Span};
    use mago_syntax::parser::parse_file;
    use std::borrow::Cow;

    let source = b"<?php class Service {} class Child extends Service {}
function inspect(): void { $value = new Child(); $value; }";
    for native in [false, true] {
        let arena = LocalArena::new();
        let file = File::ephemeral(Cow::Borrowed(b"exact-native.php"), Cow::Borrowed(source));
        let program = parse_file(&arena, &file);
        assert!(!program.has_errors());
        let names = NameResolver::new(&arena).resolve(program);
        let mut codebase = scan_program(&arena, &file, program, &names, Default::default());
        let mut references = SymbolReferences::new();
        populate_codebase(
            &mut codebase,
            &mut references,
            Default::default(),
            Default::default(),
        );
        let mut result = AnalysisResult::new(references);
        let registry = PluginRegistry::default();
        let artifacts = Analyzer::new(
            &arena,
            &file,
            &names,
            &codebase,
            &registry,
            Settings {
                track_native_types: native,
                ..Default::default()
            },
        )
        .analyze_with_artifacts(program, &mut result)
        .unwrap();
        let start = source
            .windows(7)
            .rposition(|part| part == b"$value;")
            .unwrap() as u32;
        let span = Span::new(file.id, Position::new(start), Position::new(start + 6));
        let full = artifacts.get_expression_type(&span).unwrap();
        let object = full.get_single_named_object().unwrap();
        assert_eq!(object.name, mago_word::word("Child"));
        assert!(object.is_exact);
        assert!(!object.is_static && !object.is_this);
        if native {
            assert_eq!(artifacts.get_native_expression_type(&span).unwrap(), full);
        } else {
            assert!(artifacts.get_native_expression_type(&span).is_none());
        }
    }
}

#[test]
fn subsumed_intersection_union_matches_phpstan_in_both_orders() {
    compare(
        "union.php",
        include_str!("fixtures/object-intersections/union.json"),
    );
}

#[test]
fn union_removes_the_full_key_of_a_subsumed_intersection() {
    use mago_codex::ttype::{
        TType,
        atomic::{
            TAtomic,
            object::{TObject, named::TNamedObject},
        },
        combine_union_types,
        union::TUnion,
    };

    let codebase = scanned(include_bytes!("fixtures/object-intersections/union.php"));
    let named = |name| TAtomic::Object(TObject::Named(TNamedObject::new(mago_word::word(name))));
    let mut ab = named("a");
    ab.add_intersection_type(named("b"));
    let mut abc = ab.clone();
    abc.add_intersection_type(named("c"));
    let oracle: Value =
        serde_json::from_str(include_str!("fixtures/object-intersections/union.json")).unwrap();
    let expected = oracle["files"]
        .as_object()
        .unwrap()
        .values()
        .next()
        .unwrap()["messages"][0]["message"]
        .as_str()
        .unwrap()
        .strip_prefix("Dumped type: ")
        .unwrap();
    for (left, right) in [(abc.clone(), ab.clone()), (ab.clone(), abc)] {
        let result = combine_union_types(
            &TUnion::from_atomic(left),
            &TUnion::from_atomic(right),
            &codebase,
            Default::default(),
        );
        assert_eq!(result.get_id().as_str(), Some(expected));
        assert_eq!(result.types.as_ref(), &[ab.clone()]);
    }
}

#[test]
fn mixed_union_intersects_exclusions_without_repeating_object_constraints() {
    use mago_codex::ttype::{
        TType,
        atomic::{
            TAtomic,
            mixed::TMixed,
            object::{TObject, named::TNamedObject},
        },
        combine_union_types,
        union::TUnion,
    };

    let codebase = scanned(include_bytes!("fixtures/object-intersections/flow.php"));
    let named = |name| TAtomic::Object(TObject::Named(TNamedObject::new(mago_word::word(name))));
    let without =
        |excluded| TUnion::from_atomic(TAtomic::Mixed(TMixed::with_subtracted_type(excluded)));
    let a_or_b = TUnion::from_vec(vec![named("A"), named("B")]);
    let c = TUnion::from_atomic(named("C"));
    let left = without(a_or_b);
    let right = without(c);
    let oracle: Value =
        serde_json::from_str(include_str!("fixtures/object-intersections/flow.json")).unwrap();
    let expected = oracle["files"]
        .as_object()
        .unwrap()
        .values()
        .next()
        .unwrap()["messages"][1]["message"]
        .as_str()
        .unwrap()
        .strip_prefix("Dumped type: ")
        .unwrap();

    for (left, right) in [(&left, &right), (&right, &left)] {
        let result = combine_union_types(left, right, &codebase, Default::default());
        let TAtomic::Mixed(mixed) = result.get_single() else {
            panic!("expected mixed");
        };
        let excluded = mixed.get_subtracted_type().unwrap();
        let mut conjunctions: Vec<_> = excluded
            .types
            .iter()
            .map(|atomic| {
                let mut root = atomic.clone();
                let parts = std::mem::take(root.get_intersection_types_mut().unwrap());
                let mut names: Vec<_> = std::iter::once(root.get_id().to_string())
                    .chain(parts.iter().map(|part| part.get_id().to_string()))
                    .collect();
                names.sort();
                format!("({})", names.join("&"))
            })
            .collect();
        conjunctions.sort();
        assert_eq!(format!("mixed~({})", conjunctions.join("|")), expected);
        assert_eq!(
            combine_union_types(&result, &result, &codebase, Default::default()),
            result
        );
    }
}

fn scanned(source: &'static [u8]) -> mago_codex::metadata::CodebaseMetadata {
    use mago_allocator::LocalArena;
    use mago_codex::{
        populator::populate_codebase, reference::SymbolReferences, scanner::scan_program,
    };
    use mago_database::file::File;
    use mago_names::resolver::NameResolver;
    use mago_syntax::parser::parse_file;
    use std::borrow::Cow;

    let arena = LocalArena::new();
    let file = File::ephemeral(Cow::Borrowed(b"objects.php"), Cow::Borrowed(source));
    let program = parse_file(&arena, &file);
    assert!(!program.has_errors());
    let names = NameResolver::new(&arena).resolve(program);
    let mut codebase = scan_program(&arena, &file, program, &names, Default::default());
    let mut references = SymbolReferences::new();
    populate_codebase(
        &mut codebase,
        &mut references,
        Default::default(),
        Default::default(),
    );
    codebase
}

#[test]
fn overlapping_and_nested_object_conjunctions_retain_every_distinct_constraint() {
    use mago_codex::ttype::{
        TType,
        atomic::{
            TAtomic,
            object::{TObject, named::TNamedObject},
        },
        intersect_union_types,
        union::TUnion,
    };

    let codebase = scanned(include_bytes!("fixtures/object-intersections/objects.php"));
    let object = |name, parts: Vec<_>| {
        let mut object = TNamedObject::new(mago_word::word(name));
        if !parts.is_empty() {
            object.intersection_types = Some(Box::new(parts));
        }
        TAtomic::Object(TObject::Named(object))
    };
    let a = object("A", vec![]);
    let b = object("B", vec![]);
    let c = object("C", vec![]);
    let ab = object("A", vec![b.clone()]);
    let bc = object("B", vec![c.clone()]);
    let oracle: Value =
        serde_json::from_str(include_str!("fixtures/object-intersections/objects.json")).unwrap();
    let message = oracle["files"]
        .as_object()
        .unwrap()
        .values()
        .next()
        .unwrap()["messages"][3]["message"]
        .as_str()
        .unwrap();
    let expected: Vec<_> = message
        .strip_prefix("Dumped type: ")
        .unwrap()
        .split('&')
        .collect();
    for (left, right) in [
        (a.clone(), bc.clone()),
        (ab.clone(), bc.clone()),
        (bc, ab),
        (a, object("B", vec![b, c])),
    ] {
        let left = TUnion::from_atomic(left);
        let right = TUnion::from_atomic(right);
        let result = intersect_union_types(&left, &right, &codebase).unwrap();
        let mut root = result.get_single().clone();
        let parts = std::mem::take(root.get_intersection_types_mut().unwrap());
        assert!(parts.iter().all(|part| !part.has_intersection_types()));
        let mut names: Vec<_> = std::iter::once(root.get_id().to_string())
            .chain(parts.iter().map(|part| part.get_id().to_string()))
            .collect();
        names.sort();
        assert_eq!(names, expected);
        assert_eq!(
            intersect_union_types(&result, &left, &codebase).unwrap(),
            result
        );
    }
}

#[test]
fn public_object_type_queries_and_unions_match_phpstan() {
    use pun_phpstan::types::{
        Cast, Level, NeverKind, StrictMixed, Type, accepts, cast_is_error, intersect,
        is_super_type_of, union,
    };
    use std::collections::BTreeMap;

    let object = |name: &str| Type::Object(name.into(), vec![]);
    let types = BTreeMap::from([
        ("A", object("A")),
        ("B", object("B")),
        ("C", object("C")),
        (
            "C-not-A",
            Type::object_subtracted(
                object("C"),
                Type::Intersection(vec![object("A"), object("C")]),
            ),
        ),
        (
            "C-not-B",
            Type::object_subtracted(
                object("C"),
                Type::Intersection(vec![object("B"), object("C")]),
            ),
        ),
        ("A-not-A", Type::object_subtracted(object("A"), object("A"))),
        (
            "C-not-A-or-B",
            Type::object_subtracted(
                object("C"),
                union(vec![
                    Type::Intersection(vec![object("A"), object("C")]),
                    Type::Intersection(vec![object("B"), object("C")]),
                ]),
            ),
        ),
        (
            "C-not-never",
            Type::object_subtracted(object("C"), Type::Never(NeverKind::Implicit)),
        ),
        (
            "B-and-C",
            Type::Intersection(vec![object("B"), object("C")]),
        ),
        (
            "B-and-C-not-A",
            Type::Intersection(vec![
                object("B"),
                Type::object_subtracted(
                    object("C"),
                    Type::Intersection(vec![object("A"), object("C")]),
                ),
            ]),
        ),
        ("not-A", Type::mixed_subtracted(false, object("A"))),
        ("not-zero", Type::mixed_subtracted(false, Type::ConstInt(0))),
        (
            "not-C-not-A",
            Type::mixed_subtracted(
                false,
                Type::object_subtracted(
                    object("C"),
                    Type::Intersection(vec![object("A"), object("C")]),
                ),
            ),
        ),
    ]);
    let oracle: Value =
        serde_json::from_str(include_str!("fixtures/object-intersections/type-api.json")).unwrap();
    let tri = |value| format!("{value:?}");
    let mut failures = Vec::new();
    for (name, ty) in &types {
        let want = &oracle["types"][name];
        for (query, got) in [
            ("description", Value::from(ty.describe(Level::Precise))),
            ("string", Value::from(tri(ty.is_string()))),
            ("iterable", Value::from(tri(ty.is_iterable()))),
            ("names", serde_json::json!(ty.object_class_names())),
            (
                "key",
                Value::from(ty.iterable_key_type().describe(Level::Precise)),
            ),
            (
                "value",
                Value::from(ty.iterable_value_type().describe(Level::Precise)),
            ),
            (
                "stringCastError",
                Value::from(cast_is_error(ty, Cast::String, StrictMixed::default())),
            ),
            (
                "traverse",
                Value::from(ty.traverse(&Type::clone).describe(Level::Precise)),
            ),
        ] {
            if got != want[query] {
                failures.push(format!("{name} {query}: {got} != {}", want[query]));
            }
        }
        for (other_name, other) in &types {
            let want = &oracle["relations"][name][other_name];
            for (query, got) in [
                ("super", is_super_type_of(ty, other)),
                ("accepts", accepts(ty, other, true)),
            ] {
                if tri(got) != want[query].as_str().unwrap() {
                    failures.push(format!(
                        "{name} {query} {other_name}: {got:?} != {}",
                        want[query]
                    ));
                }
            }
        }
    }
    for (pair, want) in oracle["unions"].as_object().unwrap() {
        let (left, right) = pair.split_once('|').unwrap();
        let got = union(vec![types[left].clone(), types[right].clone()]).describe(Level::Precise);
        if got != want.as_str().unwrap() {
            failures.push(format!("{pair}: {got} != {want}"));
        }
    }
    for (pair, want) in oracle["intersections"].as_object().unwrap() {
        let (left, right) = pair.split_once('&').unwrap();
        let got =
            intersect(vec![types[left].clone(), types[right].clone()]).describe(Level::Precise);
        if got != want.as_str().unwrap() {
            failures.push(format!("{pair}: {got} != {want}"));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn canonical_object_subtractions_keep_identity_and_children() {
    use mago_codex::ttype::{
        TType, TypeRef,
        atomic::{
            TAtomic,
            mixed::TMixed,
            object::{TObject, named::TNamedObject},
        },
        combine_union_types, intersect_union_types,
        union::TUnion,
    };
    let codebase = scanned(include_bytes!("fixtures/object-intersections/flow.php"));
    let named = |name| TAtomic::Object(TObject::Named(TNamedObject::new(mago_word::word(name))));
    let mixed =
        |excluded| TUnion::from_atomic(TAtomic::Mixed(TMixed::with_subtracted_type(excluded)));
    let a = TUnion::from_atomic(named("A"));
    let c = TUnion::from_atomic(named("C"));
    let without_a = mixed(a.clone());
    let without_b = mixed(TUnion::from_atomic(named("B")));
    let ca = intersect_union_types(&without_a, &c, &codebase).unwrap();
    let cb = intersect_union_types(&without_b, &c, &codebase).unwrap();
    let TAtomic::Object(TObject::Named(object)) = ca.get_single() else {
        panic!("not a named object");
    };
    let oracle: Value =
        serde_json::from_str(include_str!("fixtures/object-intersections/type-api.json")).unwrap();
    let description = oracle["types"]["C-not-A"]["description"].as_str().unwrap();
    let excluded = description.strip_prefix("C~").unwrap();
    // Codex IDs retain the primary conjunct. PHPStan sorts its public descriptions.
    let conjuncts = |description: &str| {
        let mut parts: Vec<_> = description.split('&').map(str::to_owned).collect();
        parts.sort();
        parts
    };
    assert_eq!(
        conjuncts(
            &object
                .subtracted_type
                .as_ref()
                .unwrap()
                .get_id()
                .to_string()
        ),
        conjuncts(excluded)
    );
    assert_eq!(object.get_id().as_str(), Some("C~(C&A)"));
    assert_eq!(
        object.get_pretty_id_with_indent(0).as_str(),
        Some("C~(C&A)")
    );
    let children = object.get_child_nodes();
    assert!(
        matches!(children.as_slice(), [TypeRef::Union(child)] if *child == object.subtracted_type.as_ref().unwrap().as_ref())
    );
    for (left, right) in [(&ca, &cb), (&cb, &ca)] {
        let result = combine_union_types(left, right, &codebase, Default::default());
        let TAtomic::Object(TObject::Named(object)) = result.get_single() else {
            panic!("not a named object");
        };
        assert_eq!(object.name, mago_word::word("C"));
        let expected = oracle["unions"]["C-not-A|C-not-B"]
            .as_str()
            .unwrap()
            .strip_prefix("C~")
            .unwrap();
        assert_eq!(
            conjuncts(
                &object
                    .subtracted_type
                    .as_ref()
                    .unwrap()
                    .get_id()
                    .to_string()
            ),
            conjuncts(expected)
        );
    }
    for (left, right) in [(&c, &ca), (&ca, &c)] {
        assert_eq!(
            combine_union_types(left, right, &codebase, Default::default()),
            c
        );
    }
    assert!(
        intersect_union_types(&without_a, &a, &codebase)
            .unwrap()
            .is_never()
    );
    let result = intersect_union_types(&mixed(ca), &c, &codebase).unwrap();
    assert_eq!(
        conjuncts(&result.get_id().to_string()),
        conjuncts(oracle["intersections"]["not-C-not-A&C"].as_str().unwrap())
    );
}

#[test]
fn codex_object_exclusion_queries_preserve_negative_constraints() {
    use mago_codex::ttype::{
        TType,
        atomic::{
            TAtomic,
            mixed::TMixed,
            object::{TObject, named::TNamedObject},
        },
        comparator::{ComparisonResult, union_comparator},
        intersect_union_types,
        union::TUnion,
    };
    use std::collections::BTreeMap;

    let codebase = scanned(include_bytes!("fixtures/object-intersections/flow.php"));
    let object = |name| TAtomic::Object(TObject::Named(TNamedObject::new(mago_word::word(name))));
    let conjunction = |name, child| {
        let mut root = object(name);
        root.add_intersection_type(child);
        TUnion::from_atomic(root)
    };
    let subtracted = |name, excluded| {
        TAtomic::Object(TObject::Named(
            TNamedObject::new(mago_word::word(name)).with_subtracted_type(Some(excluded)),
        ))
    };
    let without =
        |excluded| TUnion::from_atomic(TAtomic::Mixed(TMixed::with_subtracted_type(excluded)));
    let ac = conjunction("C", object("A"));
    let bc = conjunction("C", object("B"));
    let ca = subtracted("C", ac.clone());
    let cb = subtracted("C", bc.clone());
    let types = BTreeMap::from([
        ("A", TUnion::from_atomic(object("A"))),
        ("B", TUnion::from_atomic(object("B"))),
        ("C", TUnion::from_atomic(object("C"))),
        ("C-not-A", TUnion::from_atomic(ca.clone())),
        ("C-not-B", TUnion::from_atomic(cb)),
        ("not-A", without(TUnion::from_atomic(object("A")))),
        ("not-C-not-A", without(TUnion::from_atomic(ca))),
        ("B-and-C", bc),
    ]);
    let oracle: Value =
        serde_json::from_str(include_str!("fixtures/object-intersections/type-api.json")).unwrap();
    for (name, container) in &types {
        for (other_name, input) in &types {
            // Codex containment is boolean (and reflexive), unlike ObjectType's
            // trinary query. The unknown-class oracle's Maybes give no boolean answer.
            let want = oracle["relations"][name][other_name]["super"]
                .as_str()
                .unwrap();
            if want == "Maybe" {
                continue;
            }
            let got = union_comparator::is_contained_by(
                &codebase,
                input,
                container,
                false,
                false,
                false,
                &mut ComparisonResult::default(),
            );
            assert_eq!(got, want == "Yes", "{name} contains {other_name}");
        }
    }
    // Set inclusion in the scanned interfaces: excluding C&A from C excludes all
    // possible A instances. Excluding C&B instead gives no such guarantee.
    for (input, want) in [("C-not-A", true), ("C-not-B", false), ("C", false)] {
        assert_eq!(
            union_comparator::is_contained_by(
                &codebase,
                &types[input],
                &types["not-A"],
                false,
                false,
                false,
                &mut ComparisonResult::default(),
            ),
            want,
            "{input} excludes A"
        );
    }
    // Repeated negative guards combine the old and new exclusions rather than replacing either.
    let narrowed = intersect_union_types(&types["not-A"], &types["C-not-B"], &codebase).unwrap();
    // A wider excluded set gives a narrower value set, independently of the
    // unknown-class reflection API's trinary approximation above.
    let without_a_b = without(TUnion::from_vec(vec![object("A"), object("B")]));
    for (input, container, want) in [
        (&narrowed, &types["C-not-A"], true),
        (&types["C-not-A"], &narrowed, false),
        (&types["C"], &types["C-not-A"], false),
        (&types["A"], &types["C-not-A"], false),
        (&without_a_b, &types["not-A"], true),
        (&types["not-A"], &without_a_b, false),
    ] {
        assert_eq!(
            union_comparator::is_contained_by(
                &codebase,
                input,
                container,
                false,
                false,
                false,
                &mut ComparisonResult::default(),
            ),
            want
        );
    }
    for (name, container) in &types {
        let want = oracle["relations"][name]["C-not-A-or-B"]["super"]
            .as_str()
            .unwrap();
        if want == "Maybe" {
            continue;
        }
        let got = union_comparator::is_contained_by(
            &codebase,
            &narrowed,
            container,
            false,
            false,
            false,
            &mut ComparisonResult::default(),
        );
        assert_eq!(got, want == "Yes", "{name} contains repeated guards");
    }
}

#[test]
fn mixed_unions_consume_all_subtractable_object_conjuncts() {
    use mago_codex::ttype::{
        TType,
        atomic::{
            TAtomic,
            mixed::TMixed,
            object::{TObject, named::TNamedObject},
        },
        combine_union_types, get_mixed,
        union::TUnion,
    };
    let codebase = scanned(include_bytes!("fixtures/object-intersections/flow.php"));
    let object = |name| TAtomic::Object(TObject::Named(TNamedObject::new(mago_word::word(name))));
    let negative = |name| {
        let mut excluded = object(name);
        excluded.add_intersection_type(object("A"));
        TAtomic::Object(TObject::Named(
            TNamedObject::new(mago_word::word(name))
                .with_subtracted_type(Some(TUnion::from_atomic(excluded))),
        ))
    };
    let mut two = negative("B");
    two.add_intersection_type(negative("C"));
    let mut iterable = object("B");
    iterable.add_intersection_type(negative("C"));
    iterable.add_intersection_type(TAtomic::Iterable(
        mago_codex::ttype::atomic::iterable::TIterable::new(
            std::sync::Arc::new(get_mixed()),
            std::sync::Arc::new(get_mixed()),
        ),
    ));
    let mixed = TUnion::from_atomic(TAtomic::Mixed(TMixed::with_subtracted_type(
        TUnion::from_atomic(object("A")),
    )));
    let oracle: Value =
        serde_json::from_str(include_str!("fixtures/object-intersections/type-api.json")).unwrap();
    for (name, other) in [("two-exclusions", two), ("iterable-conjunct", iterable)] {
        let other = TUnion::from_atomic(other);
        for (left, right) in [(&mixed, &other), (&other, &mixed)] {
            let result = combine_union_types(left, right, &codebase, Default::default());
            let TAtomic::Mixed(mixed) = result.get_single() else {
                panic!("not mixed")
            };
            let excluded = mixed.get_subtracted_type().unwrap();
            let mut conjunctions: Vec<_> = excluded
                .types
                .iter()
                .map(|atomic| {
                    let mut root = atomic.clone();
                    let parts = std::mem::take(root.get_intersection_types_mut().unwrap());
                    let mut names: Vec<_> = std::iter::once(root.get_id().to_string())
                        .chain(parts.iter().map(|part| part.get_id().to_string()))
                        .collect();
                    names.sort();
                    names.join("&")
                })
                .collect();
            conjunctions.sort();
            let description = if conjunctions.len() == 1 {
                format!("mixed~{}", conjunctions[0])
            } else {
                format!("mixed~(({}))", conjunctions.join(")|("))
            };
            assert_eq!(
                description,
                oracle["compoundUnions"][name].as_str().unwrap()
            );
        }
    }
}
