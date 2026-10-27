//! Bottom identity is independent of call evaluation. Expectations come from PHPStan 2.3.1.

use std::borrow::Cow;
use std::rc::Rc;
use std::sync::Arc;

use mago_allocator::LocalArena;
use mago_analyzer::Analyzer;
use mago_analyzer::analysis_result::AnalysisResult;
use mago_analyzer::plugin::create_registry_with_plugins;
use mago_analyzer::settings::Settings;
use mago_codex::metadata::CodebaseMetadata;
use mago_codex::metadata::ttype::TypeMetadata;
use mago_codex::misc::GenericParent;
use mago_codex::populator::populate_codebase;
use mago_codex::scanner::scan_program;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::alias::TAlias;
use mago_codex::ttype::atomic::callable::TCallable;
use mago_codex::ttype::atomic::conditional::TConditional;
use mago_codex::ttype::atomic::derived::TDerived;
use mago_codex::ttype::atomic::derived::properties_of::TPropertiesOf;
use mago_codex::ttype::atomic::derived::template_type::TTemplateType;
use mago_codex::ttype::atomic::generic::TGenericParameter;
use mago_codex::ttype::atomic::object::TObject;
use mago_codex::ttype::atomic::object::named::TNamedObject;
use mago_codex::ttype::atomic::reference::{
    TGlobalReferenceSelector, TReference, TReferenceMemberSelector,
};
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_codex::ttype::atomic::scalar::class_like_string::{
    TClassLikeString, TClassLikeStringKind,
};
use mago_codex::ttype::combiner::CombinerOptions;
use mago_codex::ttype::expander::{TypeExpansionOptions, expand_union};
use mago_codex::ttype::template::{GenericTemplate, TemplateBound, TemplateResult};
use mago_codex::ttype::template::{definition_type_replacer, inferred_type_replacer};
use mago_codex::ttype::union::{NeverKind, TUnion};
use mago_codex::ttype::{
    add_union_type, combine_owned_union_types_rc, combine_union_types,
    combine_union_types_preserving_array_shapes, combine_union_types_rc, get_explicit_never,
    get_int, get_literal_string, get_mixed, get_never, get_non_accepting_never,
    intersect_union_types,
};
use mago_database::file::{File, FileType};
use mago_names::resolver::NameResolver;
use mago_span::{Position, Span};
use mago_syntax::parser::parse_file;
use mago_word::word;
use serde_json::{Value, json};

fn oracle() -> Value {
    serde_json::from_slice(include_bytes!(
        "fixtures/never-identity/expected-phpstan.json"
    ))
    .unwrap()
}

fn kind(ty: &TUnion) -> Value {
    match ty.never_kind() {
        Some(NeverKind::Implicit) => json!("implicit"),
        Some(NeverKind::Explicit) => json!("explicit"),
        Some(NeverKind::NonAccepting) => json!("nonaccepting"),
        None => Value::Null,
    }
}

fn types() -> [(&'static str, TUnion); 4] {
    [
        ("implicit", get_never()),
        ("explicit", get_explicit_never()),
        ("nonaccepting", get_non_accepting_never()),
        ("int", get_int()),
    ]
}

#[test]
fn independent_bottom_kinds_and_all_public_union_paths() {
    let oracle = oracle();
    let codebase = CodebaseMetadata::new();
    for (name, ty) in types() {
        assert_eq!(kind(&ty), oracle["types"][name]["kind"]);
        assert_eq!(
            ty.is_explicit_never(),
            name == "explicit" || name == "nonaccepting"
        );
        for (other_name, other) in types() {
            let key = format!("{name}|{other_name}");
            let want = &oracle["unions"][&key]["kind"];
            let a = Rc::new(ty.clone());
            let b = Rc::new(other.clone());
            let options = CombinerOptions::default();
            for result in [
                add_union_type(ty.clone(), &other, &codebase, options),
                combine_union_types(&ty, &other, &codebase, options),
                combine_union_types_preserving_array_shapes(&ty, &other, &codebase, options),
                (*combine_union_types_rc(&a, &b, &codebase, options)).clone(),
                (*combine_owned_union_types_rc(Rc::new(ty.clone()), &b, &codebase, options))
                    .clone(),
                (*combine_owned_union_types_rc(Rc::clone(&a), &b, &codebase, options)).clone(),
            ] {
                assert_eq!(&kind(&result), want, "{key}: {result:?}");
                assert_eq!(result.is_never(), want.is_string(), "{key}");
                if want.is_null() {
                    assert_eq!(result, get_int(), "{key}");
                }
            }
            let key = format!("{name}&{other_name}");
            assert_eq!(
                kind(&intersect_union_types(&ty, &other, &codebase).unwrap()),
                oracle["intersections"][&key]["kind"],
                "{key}"
            );
        }
    }
}

#[test]
fn replacement_and_serde_preserve_kind_without_tainting_computed_bottom() {
    let oracle = oracle();
    for (name, ty) in types() {
        let want = &oracle["types"][name]["kind"];
        assert_eq!(&kind(&ty.clone()), want);
        assert_eq!(&kind(&ty.clone_with_types(ty.types.to_vec())), want);
        let encoded = serde_json::to_vec(&ty).unwrap();
        let decoded: TUnion = serde_json::from_slice(&encoded).unwrap();
        assert_eq!(&kind(&decoded), want);
        assert_eq!(decoded.is_explicit_never(), ty.is_explicit_never());
        let mut original = get_int();
        original.set_had_template(true);
        let replaced = original.clone_with_union(ty);
        assert!(replaced.had_template());
        assert_eq!(&kind(&replaced), want);
    }
    let ordinary = get_non_accepting_never().clone_with_types(get_int().types.to_vec());
    assert!(!ordinary.is_explicit_never());
    let computed = ordinary.clone_with_types(vec![TAtomic::Never]);
    assert_eq!(computed.never_kind(), Some(NeverKind::Implicit));
    assert_eq!(
        get_explicit_never()
            .as_nullable()
            .to_non_nullable()
            .never_kind(),
        Some(NeverKind::Implicit)
    );
    let old = serde_json::to_value(get_never()).unwrap();
    assert_eq!(old["flags"], 0);
    assert_eq!(
        serde_json::from_value::<TUnion>(old).unwrap().never_kind(),
        Some(NeverKind::Implicit)
    );
}

#[test]
fn template_reconstruction_preserves_the_actual_bound_kind() {
    let oracle = oracle();
    let codebase = CodebaseMetadata::new();
    let name = word("T");
    let owner = GenericParent::ClassLike(word("Box"));
    let parameter = TUnion::from_atomic(TAtomic::GenericParameter(TGenericParameter::new(
        name,
        Arc::new(get_mixed()),
        owner,
    )));
    for (kind_name, bound) in types() {
        let want = oracle["types"][kind_name]["kind"].clone();
        let mut templates = TemplateResult::default();
        templates
            .lower_bounds
            .entry(name)
            .or_default()
            .insert(owner, vec![TemplateBound::new(bound.clone(), 0, None)]);
        templates
            .template_types
            .insert(name, vec![GenericTemplate::new(owner, bound.clone())]);
        assert_eq!(
            kind(&inferred_type_replacer::replace(
                &parameter, &templates, &codebase
            )),
            want
        );
        assert_eq!(
            kind(&definition_type_replacer::replace(
                &parameter,
                &mut templates,
                &codebase,
                Default::default()
            )),
            want
        );
        let other_name = word("U");
        let other = TAtomic::GenericParameter(TGenericParameter::new(
            other_name,
            Arc::new(get_mixed()),
            owner,
        ));
        templates
            .lower_bounds
            .entry(other_name)
            .or_default()
            .insert(owner, vec![TemplateBound::new(bound.clone(), 0, None)]);
        let pair = TUnion::from_vec(vec![parameter.get_single().clone(), other]);
        assert_eq!(
            kind(&inferred_type_replacer::replace(
                &pair, &templates, &codebase
            )),
            want
        );
        assert_eq!(
            kind(&definition_type_replacer::replace(
                &bound,
                &mut templates,
                &codebase,
                Default::default()
            )),
            want
        );
        let constrained = TUnion::from_atomic(TAtomic::GenericParameter(TGenericParameter::new(
            name,
            Arc::new(bound.clone()),
            owner,
        )));
        let mut constraints = TemplateResult::default();
        constraints
            .template_types
            .insert(name, vec![GenericTemplate::new(owner, get_mixed())]);
        assert_eq!(
            kind(&definition_type_replacer::replace(
                &constrained,
                &mut constraints,
                &codebase,
                Default::default(),
            )),
            want
        );
        let nested = TUnion::from_atomic(TAtomic::GenericParameter(TGenericParameter::new(
            other_name,
            Arc::new(bound),
            owner,
        )));
        constraints
            .template_types
            .insert(name, vec![GenericTemplate::new(owner, nested)]);
        assert_eq!(
            kind(&definition_type_replacer::replace(
                &parameter,
                &mut constraints,
                &codebase,
                definition_type_replacer::DefinitionReplacementOptions {
                    iteration_depth: 15,
                    ..Default::default()
                },
            )),
            want
        );
    }
}

#[test]
fn conditional_expansion_unions_bottom_identities_without_marking_ordinary_results() {
    let oracle = oracle();
    let codebase = CodebaseMetadata::new();
    for (name, yes) in types() {
        for (other_name, no) in types() {
            let mut conditional = TUnion::from_atomic(TAtomic::Conditional(TConditional::new(
                Arc::new(get_int()),
                Arc::new(get_int()),
                Arc::new(yes.clone()),
                Arc::new(no),
                false,
            )));
            expand_union(
                &codebase,
                &mut conditional,
                &TypeExpansionOptions::default(),
            );
            let key = format!("{name}|{other_name}");
            assert_eq!(kind(&conditional), oracle["unions"][&key]["kind"], "{key}");
        }
    }
}

#[test]
fn expansion_keeps_selected_metadata_and_template_values() {
    let oracle = oracle();
    let source = b"<?php const VALUE = 1; /** @template T */ class Box { public const VALUE = 1; }";
    let file = File::new(
        Cow::Borrowed(b"metadata.php"),
        FileType::Host,
        None,
        Cow::Borrowed(source),
    );
    let arena = LocalArena::new();
    let program = parse_file(&arena, &file);
    let names = NameResolver::new(&arena).resolve(program);
    let mut codebase = scan_program(&arena, &file, program, &names, Default::default());
    let span = Span::new(file.id, Position::new(0), Position::new(1));
    let cycle = TUnion::from_atomic(TAtomic::Alias(TAlias::new(word("Box"), word("Cycle"))));
    codebase
        .class_likes
        .get_mut(&word("box"))
        .unwrap()
        .type_aliases
        .insert(
            word("Cycle"),
            TypeMetadata::from_docblock(cycle.clone(), span),
        );
    for (name, ty) in types() {
        let want = &oracle["types"][name]["kind"];
        for inferred in [false, true] {
            let class = codebase.class_likes.get_mut(&word("box")).unwrap();
            class.type_aliases.insert(
                word("Bottom"),
                TypeMetadata::from_docblock(ty.clone(), span),
            );
            let constant = class.constants.get_mut(&word("VALUE")).unwrap();
            constant.type_metadata = Some(TypeMetadata::new(ty.clone(), span));
            constant.inferred_type = inferred.then(|| ty.clone());
            let constant = codebase.constants.get_mut(&word("VALUE")).unwrap();
            constant.type_metadata = Some(TypeMetadata::new(ty.clone(), span));
            constant.inferred_type = inferred.then(|| ty.clone());
            let object = TUnion::from_atomic(TAtomic::Object(TObject::Named(
                TNamedObject::new_with_type_parameters(word("Box"), Some(vec![ty.clone()])),
            )));
            for preserve_initializer_union_order in [false, true] {
                let options = TypeExpansionOptions {
                    preserve_initializer_union_order,
                    ..Default::default()
                };
                for mut value in [
                    TUnion::from_atomic(TAtomic::Alias(TAlias::new(word("Box"), word("Bottom")))),
                    TUnion::from_atomic(TAtomic::Reference(TReference::new_member(
                        word("Box"),
                        TReferenceMemberSelector::Identifier(word("Bottom")),
                    ))),
                    TUnion::from_atomic(TAtomic::Reference(TReference::new_member(
                        word("Box"),
                        TReferenceMemberSelector::Identifier(word("VALUE")),
                    ))),
                    TUnion::from_atomic(TAtomic::Reference(TReference::Global {
                        selector: TGlobalReferenceSelector::Identifier(word("VALUE")),
                    })),
                    TUnion::from_atomic(TAtomic::Derived(TDerived::TemplateType(
                        TTemplateType::new(
                            Arc::new(object.clone()),
                            Arc::new(get_literal_string(word("Box"))),
                            Arc::new(get_literal_string(word("T"))),
                        ),
                    ))),
                ] {
                    expand_union(&codebase, &mut value, &options);
                    assert_eq!(
                        &kind(&value),
                        want,
                        "{name}, inferred={inferred}: {value:?}"
                    );
                }
                let mut properties = TUnion::from_atomic(TAtomic::Derived(TDerived::PropertiesOf(
                    TPropertiesOf::new(Arc::new(object.clone())),
                )));
                expand_union(&codebase, &mut properties, &options);
                assert!(properties.is_array());
                assert_eq!(properties.never_kind(), None);
                let mut class_string = TUnion::from_atomic(TAtomic::Scalar(
                    TScalar::ClassLikeString(TClassLikeString::of_type(
                        TClassLikeStringKind::Class,
                        object.get_single().clone(),
                    )),
                ));
                expand_union(&codebase, &mut class_string, &options);
                assert_eq!(class_string.never_kind(), None);
                assert!(matches!(
                    class_string.get_single(),
                    TAtomic::Scalar(TScalar::ClassLikeString(TClassLikeString::OfType { .. }))
                ));
            }
        }
    }
    for mut value in [
        cycle.clone(),
        TUnion::from_atomic(TAtomic::Alias(TAlias::new(word("Missing"), word("Bottom")))),
    ] {
        let original = value.clone();
        expand_union(&codebase, &mut value, &TypeExpansionOptions::default());
        assert_eq!(value, original);
        assert!(!value.is_explicit_never());
    }
    let mut missing = TUnion::from_atomic(TAtomic::Derived(TDerived::TemplateType(
        TTemplateType::new(
            Arc::new(get_mixed()),
            Arc::new(get_literal_string(word("Missing"))),
            Arc::new(get_literal_string(word("T"))),
        ),
    )));
    expand_union(&codebase, &mut missing, &TypeExpansionOptions::default());
    assert_eq!(missing, get_mixed());
}

#[test]
fn scanned_native_and_phpdoc_never_are_nonaccepting() {
    let source = include_bytes!("fixtures/never-identity/types.php");
    let oracle: Value = serde_json::from_slice(include_bytes!(
        "fixtures/never-identity/expected-scopes.json"
    ))
    .unwrap();
    let file = File::new(
        Cow::Borrowed(b"never.php"),
        FileType::Host,
        None,
        Cow::Borrowed(source),
    );
    let arena = LocalArena::new();
    let program = parse_file(&arena, &file);
    assert!(!program.has_errors());
    let names = NameResolver::new(&arena).resolve(program);
    let metadata = scan_program(&arena, &file, program, &names, Default::default());
    for (name, expected) in oracle["declarations"].as_object().unwrap() {
        let function = metadata.get_function(name.as_bytes()).unwrap();
        if let Some(metadata) = &function.return_type_metadata {
            assert_eq!(
                kind(&metadata.type_union),
                expected["kind"],
                "{name}: {metadata:?}"
            );
        } else {
            assert!(expected["kind"].is_null(), "{name}");
        }
    }
}

#[test]
fn selected_conditional_branches_keep_their_actual_bottom_identity() {
    let source = include_bytes!("fixtures/never-identity/types.php");
    let oracle: Value = serde_json::from_slice(include_bytes!(
        "fixtures/never-identity/expected-scopes.json"
    ))
    .unwrap();
    let file = File::new(
        Cow::Borrowed(b"never.php"),
        FileType::Host,
        None,
        Cow::Borrowed(source),
    );
    let arena = LocalArena::new();
    let program = parse_file(&arena, &file);
    let names = NameResolver::new(&arena).resolve(program);
    let prelude = mago_prelude::Prelude::build();
    let mut codebase = prelude.metadata;
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
    let plugins = create_registry_with_plugins(&[], false);
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
    for (offset, expected) in oracle["calls"].as_object().unwrap() {
        let start = offset.parse::<u32>().unwrap();
        let expression = expected["expression"].as_str().unwrap();
        let span = Span::new(
            file.id,
            Position::new(start),
            Position::new(start + expression.len() as u32),
        );
        let ty = artifacts
            .get_expression_type(&span)
            .unwrap_or_else(|| panic!("missing {expression}"));
        assert_eq!(kind(ty), expected["kind"], "{expression}: {ty:?}");
        assert_eq!(
            ty.is_int(),
            expected["type"] == "int",
            "{expression}: {ty:?}"
        );
        if !expected["return"].is_null() {
            let TAtomic::Callable(TCallable::Signature(signature)) = ty.get_single() else {
                panic!("not a callable: {ty:?}");
            };
            let TAtomic::Conditional(conditional) =
                signature.get_return_type().unwrap().get_single()
            else {
                panic!("lost deferred conditional: {signature:?}");
            };
            assert_eq!(kind(&conditional.then), expected["return"]["if"]["kind"]);
            assert_eq!(
                kind(&conditional.otherwise),
                expected["return"]["else"]["kind"]
            );
            assert!(conditional.otherwise.is_int());
        }
    }
}
