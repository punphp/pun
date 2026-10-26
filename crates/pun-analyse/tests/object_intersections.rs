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
fn union_absorption_checks_the_primary_generic_and_receiver_constraints() {
    compare(
        "constraints.php",
        include_str!("fixtures/object-intersections/constraints.json"),
    );
}

#[test]
fn raw_union_preserves_distinct_primary_parameters() {
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
