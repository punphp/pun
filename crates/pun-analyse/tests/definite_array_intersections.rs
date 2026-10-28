//! PHPStan's definite constant-array intersection through public scanning and type APIs.

use std::{borrow::Cow, collections::BTreeMap, path::Path};

use mago_allocator::LocalArena;
use mago_codex::{
    populator::populate_codebase,
    reference::SymbolReferences,
    scanner::scan_program,
    ttype::{
        TType,
        atomic::{
            TAtomic,
            array::{TArray, key::ArrayKey, list::TList},
            scalar::{TScalar, int::TInteger},
        },
        get_never, get_non_negative_int, intersect_union_types,
        union::TUnion,
    },
};
use mago_database::file::File;
use mago_names::resolver::NameResolver;
use mago_syntax::parser::parse_file;
use serde_json::{Value, json};

fn describe(union: &TUnion) -> Value {
    if union.is_never() {
        return json!("never");
    }
    if let [TAtomic::Array(array)] = union.types.as_ref() {
        let mut items = BTreeMap::new();
        let tail = match array {
            TArray::Keyed(array) => {
                for (key, (optional, value)) in
                    array.known_items.iter().flat_map(|items| items.iter())
                {
                    let key = match key {
                        ArrayKey::Integer(key) => format!("i:{key}"),
                        ArrayKey::String(key) => format!("s:{key}"),
                        ArrayKey::ClassLikeConstant { .. } => panic!("unresolved array key"),
                    };
                    items.insert(key, json!([optional, describe(value)]));
                }
                array
                    .parameters
                    .as_ref()
                    .map(|(key, value)| json!([describe(key), describe(value)]))
            }
            TArray::List(array) => {
                for (key, (optional, value)) in
                    array.known_elements.iter().flat_map(|items| items.iter())
                {
                    items.insert(format!("i:{key}"), json!([optional, describe(value)]));
                }
                (!array.element_type.is_never()).then(|| {
                    json!([
                        describe(&get_non_negative_int()),
                        describe(&array.element_type)
                    ])
                })
            }
        };
        let nonempty = if array.is_non_empty() {
            "Yes"
        } else if array.is_empty() {
            "No"
        } else {
            "Maybe"
        };
        let constant = match array {
            TArray::Keyed(array) => array.is_constant_shape(),
            TArray::List(array) => {
                array.element_type.is_never()
                    || array
                        .known_elements
                        .as_ref()
                        .is_some_and(|items| !items.is_empty())
            }
        };
        return json!({"items": items, "tail": tail, "callable": array.callable().is_some(), "size": describe(&array.get_size()), "nonempty": nonempty, "constant": constant});
    }
    let atoms: Vec<_> = union
        .types
        .iter()
        .map(|atomic| match atomic {
            TAtomic::Scalar(TScalar::Integer(integer)) => match integer {
                TInteger::Literal(value) => value.to_string(),
                TInteger::Unspecified => "int".into(),
                integer => {
                    let (min, max) = integer.get_bounds();
                    format!(
                        "int<{},{}>",
                        min.map_or_else(|| "min".into(), |v| v.to_string()),
                        max.map_or_else(|| "max".into(), |v| v.to_string())
                    )
                }
            },
            TAtomic::Scalar(TScalar::String(string)) if string.is_known_literal() => {
                format!(
                    "'{}'",
                    String::from_utf8_lossy(string.get_known_literal_value().unwrap())
                )
            }
            atomic => atomic.get_id().to_string().replace(' ', ""),
        })
        .collect();
    json!(atoms.join("|"))
}

// This tests the public type API, not Mago's unsupported explicit-key list PHPDoc syntax.
// Parse the fields as array notation, then supply the independent list constraint directly.
fn list_input(source: &str, union: &TUnion) -> TUnion {
    if !source.starts_with("list{") {
        return union.clone();
    }
    let TAtomic::Array(TArray::Keyed(array)) = union.get_single() else {
        panic!("list fields were not scanned");
    };
    let elements = array
        .known_items
        .as_ref()
        .unwrap()
        .iter()
        .map(|(key, value)| {
            (
                usize::try_from(key.get_integer().unwrap()).unwrap(),
                value.clone(),
            )
        })
        .collect();
    let mut list = TList::from_known_elements(elements);
    if let Some((_, value)) = &array.parameters {
        list.element_type = value.clone();
        list.known_count = None;
    }
    TUnion::from_atomic(TAtomic::Array(TArray::List(list)))
}

fn notation(source: &str) -> String {
    if source.starts_with("list{") {
        source
            .replacen("list{", "array{", 1)
            .replacen("...<", "...<int,", 1)
    } else {
        source.into()
    }
}

#[test]
fn definite_arrays_intersect_in_both_orders_without_losing_fields_or_callable_metadata() {
    let root =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/definite-array-intersections");
    let cases: Vec<(String, String, String)> =
        serde_json::from_slice(&std::fs::read(root.join("cases.json")).unwrap()).unwrap();
    let expected: Value =
        serde_json::from_slice(&std::fs::read(root.join("expected.json")).unwrap()).unwrap();
    let mut source = String::from("<?php\n");
    for (i, (_, left, right)) in cases.iter().enumerate() {
        let left = notation(left);
        let right = notation(right);
        source.push_str(&format!("/**\n * @param {left} $left\n * @param {right} $right\n */\nfunction pair{i}(array $left, array $right): void {{}}\n"));
    }
    let arena = LocalArena::new();
    let file = File::ephemeral(
        Cow::Borrowed(b"definite-arrays.php"),
        Cow::Owned(source.into_bytes()),
    );
    let program = parse_file(&arena, &file);
    assert!(!program.has_errors());
    let names = NameResolver::new(&arena).resolve(program);
    let mut codebase = scan_program(&arena, &file, program, &names, Default::default());
    populate_codebase(
        &mut codebase,
        &mut SymbolReferences::new(),
        Default::default(),
        Default::default(),
    );
    let mut failures = Vec::new();
    for (i, (name, left_source, right_source)) in cases.iter().enumerate() {
        let function = codebase
            .get_function(format!("pair{i}").as_bytes())
            .unwrap();
        let left = &function.parameters[0]
            .type_metadata
            .as_ref()
            .unwrap()
            .type_union;
        let right = &function.parameters[1]
            .type_metadata
            .as_ref()
            .unwrap()
            .type_union;
        let left = &list_input(left_source, left);
        let right = &list_input(right_source, right);
        let intersections = [
            intersect_union_types(left, right, &codebase).unwrap_or_else(get_never),
            intersect_union_types(right, left, &codebase).unwrap_or_else(get_never),
        ];
        if left_source.starts_with("list{") || right_source.starts_with("list{") {
            // PHPStan 2.3.1 reports isList()->Yes in both orders for all five list pairs.
            // Field-only descriptions would miss losing this independent list constraint.
            for intersection in &intersections {
                if !intersection.is_never() {
                    assert!(
                        matches!(intersection.get_single(), TAtomic::Array(array) if array.is_list()),
                        "{name}: intersection lost the list constraint"
                    );
                }
            }
        }
        let actual = json!([describe(&intersections[0]), describe(&intersections[1]),]);
        if actual != expected[name] {
            failures.push(json!({"case": name, "left": describe(left), "right": describe(right), "actual": actual, "expected": expected[name]}));
        }
    }
    assert!(
        failures.is_empty(),
        "{}",
        serde_json::to_string_pretty(&failures).unwrap()
    );
}
