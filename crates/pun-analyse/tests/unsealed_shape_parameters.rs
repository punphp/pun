//! Omitted unsealed keys must not consume the declaration's value parameter.

use std::{borrow::Cow, path::Path};

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
            array::{TArray, key::ArrayKey},
            scalar::{TScalar, int::TInteger},
        },
        union::TUnion,
    },
};
use mago_database::file::File;
use mago_names::resolver::NameResolver;
use mago_php_version::PHPVersion;
use mago_span::{HasSpan, Span};
use mago_syntax::{cst::*, parser::parse_file, walker::MutWalker};
use serde_json::{Value, json};

fn describe(ty: &TUnion) -> Value {
    if let [TAtomic::GenericParameter(parameter)] = ty.types.as_ref() {
        return json!({"template": parameter.parameter_name.to_string(), "bound": describe(&parameter.constraint)});
    }
    if let [TAtomic::Scalar(TScalar::ArrayKey)] = ty.types.as_ref() {
        return json!("int|string");
    }
    if let [TAtomic::Scalar(TScalar::Integer(integer))] = ty.types.as_ref() {
        if *integer == TInteger::Unspecified {
            return json!("int");
        }
        let (min, max) = integer.get_bounds();
        return json!(format!(
            "int<{},{}>",
            min.map_or_else(|| "min".into(), |n| n.to_string()),
            max.map_or_else(|| "max".into(), |n| n.to_string())
        ));
    }
    json!(ty.get_id().to_string().replace(' ', ""))
}

#[derive(Default)]
struct Dumps(Vec<(u32, Span)>);
impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Dumps {
    fn walk_in_function_call(&mut self, call: &'ast FunctionCall<'arena>, (): &mut ()) {
        if let Expression::Identifier(name) = call.function
            && name.value() == b"\\PHPStan\\dumpType"
        {
            self.0.push((
                call.span().start.offset,
                call.argument_list.arguments.first().unwrap().value().span(),
            ));
        }
    }
}

fn root() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/unsealed-shape-parameters")
}

#[test]
fn native_artifacts_keep_key_domains_values_prefixes_and_nonemptiness() {
    let source = std::fs::read_to_string(root().join("shapes.php")).unwrap();
    let oracle: Value =
        serde_json::from_slice(&std::fs::read(root().join("native-facts.json")).unwrap()).unwrap();
    let arena = LocalArena::new();
    let file = File::ephemeral(
        Cow::Borrowed(b"shapes.php"),
        Cow::Owned(source.as_bytes().to_vec()),
    );
    let program = parse_file(&arena, &file);
    assert!(!program.has_errors());
    let names = NameResolver::new(&arena).resolve(program);
    let mut codebase = scan_program(&arena, &file, program, &names, PHPVersion::PHP85);
    let mut references = SymbolReferences::new();
    populate_codebase(
        &mut codebase,
        &mut references,
        Default::default(),
        Default::default(),
    );
    let registry = PluginRegistry::new();
    let artifacts = Analyzer::new(
        &arena,
        &file,
        &names,
        &codebase,
        &registry,
        Settings::default(),
    )
    .analyze_with_artifacts(program, &mut AnalysisResult::new(references))
    .unwrap();
    let mut dumps = Dumps::default();
    dumps.walk_program(program, &mut ());
    let actual: Vec<_> = dumps.0.into_iter().map(|(offset, expression)| {
        let array = artifacts.get_expression_type(&expression).unwrap().get_single_array().unwrap();
        let items: Vec<_> = match array {
            TArray::Keyed(array) => array.known_items.iter().flat_map(|items| items.iter()).map(|(key, (optional, value))| {
                let key = match key {
                    ArrayKey::Integer(key) => format!("i:{key}"),
                    ArrayKey::String(key) => format!("s:{key}"),
                    _ => panic!("unexpected unresolved key"),
                };
                json!([key, optional, describe(value)])
            }).collect(),
            TArray::List(array) => array.known_elements.iter().flat_map(|items| items.iter())
                .map(|(key, (optional, value))| json!([format!("i:{key}"), optional, describe(value)])).collect(),
        };
        // Key domains are observable in both carriers; list key bounds are not
        // substituted for the native API's broader `int` observation.
        let key = array.get_key_type().unwrap();
        let integer = key.types.iter().any(|ty| matches!(ty, TAtomic::Scalar(TScalar::Integer(_) | TScalar::ArrayKey)));
        let string = key.types.iter().any(|ty| matches!(ty, TAtomic::Scalar(TScalar::String(_) | TScalar::ArrayKey)));
        json!({
            "line": source.as_bytes()[..offset as usize].iter().filter(|c| **c == b'\n').count() + 1,
            "items": items,
            "extra": [{"integer": integer, "string": string}, describe(&array.get_value_type().unwrap())],
            "nonempty": array.is_non_empty(),
            "size": describe(&array.get_size()),
        })
    }).collect();
    eprintln!("complete native artifacts: {}", json!(actual));
    assert_eq!(json!(actual), oracle);
}

// Complete whole-shape expectations remain in shapes.json. Known-prefix and
// nonempty reconstruction in the shared converter are separate from value reads.
#[test]
fn public_analysis_retains_all_complete_phpstan_value_records() {
    let oracle: Value =
        serde_json::from_slice(&std::fs::read(root().join("values.json")).unwrap()).unwrap();
    let mut project = pun_analyse::Project::new(vec![root().join("values.php")], Vec::new());
    project.level = 0;
    project.php_version = 80511;
    let analysis = pun_analyse::analyse_project(&project).unwrap();
    let actual: Vec<_> = analysis
        .errors
        .into_iter()
        .map(|error| {
            assert_eq!(Path::new(&error.file), root().join("values.php"));
            (
                error.line,
                error.identifier,
                error.message,
                error.tip,
                error.can_be_ignored,
            )
        })
        .collect();
    let expected: Vec<_> = oracle["files"]["values.php"]["messages"]
        .as_array()
        .unwrap()
        .iter()
        .map(|error| {
            (
                error["line"].as_i64().unwrap(),
                error["identifier"].as_str().unwrap().to_owned(),
                error["message"].as_str().unwrap().to_owned(),
                error["tip"].as_str().map(str::to_owned),
                error["ignorable"].as_bool().unwrap(),
            )
        })
        .collect();
    eprintln!("complete analysis records: {actual:#?}");
    assert_eq!(actual, expected);
    assert!(analysis.general.is_empty(), "{:?}", analysis.general);
}
