//! PHPStan's bounded constant-key builder preserves slots without inventing key correlations.

use std::borrow::Cow;
use std::collections::BTreeSet;
use std::path::Path;

use mago_allocator::LocalArena;
use mago_analyzer::{
    Analyzer, analysis_result::AnalysisResult, plugin::PluginRegistry, settings::Settings,
};
use mago_codex::{
    populator::populate_codebase,
    reference::SymbolReferences,
    scanner::scan_program,
    ttype::{
        TType,
        atomic::{
            TAtomic,
            array::TArray,
            scalar::{TScalar, int::TInteger},
        },
        union::TUnion,
    },
};
use mago_database::file::File;
use mago_names::resolver::NameResolver;
use mago_syntax::parser::parse_file;
use pun_analyse::{Project, run};
use serde_json::{Value, json};

// Compare the public engine's slot contract, not PHPStan/Mago's different shape
// display orders. The unmodified oracle descriptions are retained in the capture.
fn scalars(ty: &TUnion) -> Vec<String> {
    let mut values = BTreeSet::new();
    for atomic in ty.types.iter() {
        if let Some(value) = atomic.get_literal_string_value() {
            values.insert(format!("string:{}", String::from_utf8_lossy(value)));
        } else if let Some(value) = atomic.get_literal_int_value() {
            values.insert(format!("int:{value}"));
        } else if let TAtomic::Scalar(TScalar::Integer(TInteger::Range(min, max))) = atomic {
            assert!(i128::from(*max) - i128::from(*min) < 1024);
            values.extend((*min..=*max).map(|i| format!("int:{i}")));
        } else if atomic.is_true() {
            values.insert("bool:1".into());
        } else if atomic.is_false() {
            values.insert("bool:0".into());
        } else if atomic.is_null() {
            values.insert("null".into());
        } else {
            values.insert(atomic.get_id().to_string());
        }
    }
    if values.contains("string") {
        values.retain(|value| !value.starts_with("string:"));
    }
    values.into_iter().collect()
}

#[test]
fn bounded_finite_array_literal_keys_match_phpstan() {
    let root =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/finite-array-literal-keys");
    let expected: Value =
        serde_json::from_slice(&std::fs::read(root.join("expected.json")).unwrap()).unwrap();
    let source = std::fs::read(root.join("matrix.php")).unwrap();
    let file = File::ephemeral(Cow::Borrowed(b"matrix.php"), Cow::Owned(source));
    let arena = LocalArena::new();
    let program = parse_file(&arena, &file);
    assert!(!program.has_errors());
    let names = NameResolver::new(&arena).resolve(program);
    let settings = Settings::default();
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
    let mut failures = Vec::new();
    for want in expected["types"].as_array().unwrap() {
        let start = want["span"][0].as_u64().unwrap() as u32;
        let end = want["span"][1].as_u64().unwrap() as u32;
        let ty = &artifacts.expression_types[&(start, end)];
        let [TAtomic::Array(array)] = ty.types.as_ref() else {
            panic!("{ty:?}")
        };
        let mut got = json!({"line": want["line"], "span": [start, end], "is_list": array.is_list(), "count": null});
        let mut slots = Vec::new();
        let mut keys = Vec::new();
        let mut values = Vec::new();
        let constant = match array {
            TArray::Keyed(array) => {
                got["non_empty"] = json!(array.non_empty);
                for (key, (optional, value)) in
                    array.known_items.iter().flat_map(|items| items.iter())
                {
                    let key = scalars(&key.to_union());
                    let value = scalars(value);
                    keys.extend(key.clone());
                    values.extend(value.clone());
                    slots.push(json!([key[0], optional, value]));
                }
                if let Some((key, value)) = &array.parameters {
                    keys.extend(scalars(key));
                    values.extend(scalars(value));
                }
                if array.is_constant_shape() && slots.iter().all(|slot| slot[1] == false) {
                    got["count"] = json!(slots.len());
                }
                array.is_constant_shape()
            }
            TArray::List(array) => {
                got["non_empty"] = json!(array.non_empty);
                got["count"] = json!(array.known_count);
                for (key, (optional, value)) in
                    array.known_elements.iter().flat_map(|items| items.iter())
                {
                    slots.push(json!([format!("int:{key}"), optional, scalars(value)]));
                }
                assert!(array.element_type.is_never());
                true
            }
        };
        if constant {
            slots.sort_by(|a, b| a[0].as_str().cmp(&b[0].as_str()));
            got["slots"] = json!(slots);
        } else {
            if keys.iter().any(|key| key == "string") {
                keys.retain(|key| !key.starts_with("string:"));
            }
            keys.sort();
            keys.dedup();
            values.sort();
            values.dedup();
            got["keys"] = json!(keys);
            got["values"] = json!(values);
        }
        if &got != want {
            failures.push(json!({"expected": want, "actual": got,
                "oracle": expected["descriptions"][want["line"].as_u64().unwrap().to_string()]}));
        }
    }
    let mut project = Project::new(vec![root.join("matrix.php")], Vec::new());
    project.level = 5;
    let mut actual: Vec<_> = run(&project)
        .unwrap()
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
    let actual = serde_json::to_value(actual).unwrap();
    let want = expected["diagnostics"].as_array().unwrap();
    let got = actual.as_array().unwrap();
    if actual != expected["diagnostics"] {
        failures.push(json!({
            "missing_diagnostics": want.iter().filter(|record| !got.contains(record)).collect::<Vec<_>>(),
            "extra_diagnostics": got.iter().filter(|record| !want.contains(record)).collect::<Vec<_>>()
        }));
    }
    assert!(
        failures.is_empty(),
        "{}",
        serde_json::to_string_pretty(&failures).unwrap()
    );
}
