//! Current-file metadata must replace successful stale hits, not merely missing entries.
//! Scalar expectations below follow the PHP declarations, independently of the scanner.

use std::borrow::Cow;
use std::sync::{Arc, Mutex};

use mago_allocator::LocalArena;
use mago_analyzer::Analyzer;
use mago_analyzer::analysis_result::AnalysisResult;
use mago_analyzer::plugin::{
    ExpressionHook, ExpressionHookResult, HookContext, HookResult, PluginRegistry, Provider,
    ProviderMeta, create_registry_with_plugins,
};
use mago_analyzer::settings::Settings;
use mago_codex::metadata::CodebaseMetadata;
use mago_codex::misc::GenericParent;
use mago_codex::populator::populate_codebase;
use mago_codex::reference::SymbolReferences;
use mago_codex::scanner::scan_program;
use mago_codex::signature_builder::build_file_signature;
use mago_codex::ttype::TType;
use mago_database::file::{File, FileType};
use mago_names::resolver::NameResolver;
use mago_span::HasSpan;
use mago_syntax::cst::{Expression, Variable};
use mago_syntax::parser::parse_file;
use mago_word::{empty_word, word};

#[derive(Debug, Default, PartialEq)]
struct Observed {
    parameters: Vec<Vec<String>>,
    parameter_flags: Vec<Vec<(bool, bool, bool)>>,
    returns: Vec<Option<String>>,
    variables: Vec<(u32, String)>,
}

struct Observer(Arc<Mutex<Observed>>);
impl Provider for Observer {
    fn meta() -> &'static ProviderMeta {
        static META: ProviderMeta = ProviderMeta::new(
            "fresh-closures",
            "fresh-closures",
            "current signatures and live bodies",
        );
        &META
    }
}
impl ExpressionHook for Observer {
    fn is_observer(&self) -> bool {
        true
    }

    fn before_expression(
        &self,
        expr: &Expression<'_>,
        context: &mut HookContext<'_, '_>,
    ) -> HookResult<ExpressionHookResult> {
        if matches!(expr, Expression::ArrowFunction(_) | Expression::Closure(_)) {
            let metadata = context
                .get_closure_metadata(expr)
                .expect("current closure metadata");
            assert_eq!(metadata.span, expr.span());
            assert!(std::ptr::eq(
                metadata,
                &**context
                    .codebase()
                    .function_likes
                    .get(&(empty_word(), metadata.name))
                    .unwrap()
            ));
            let mut found = self.0.lock().unwrap();
            found.parameters.push(
                metadata
                    .parameters
                    .iter()
                    .map(|p| {
                        p.type_metadata
                            .as_ref()
                            .map_or("untyped".into(), |t| t.type_union.get_id().to_string())
                    })
                    .collect(),
            );
            found.parameter_flags.push(
                metadata
                    .parameters
                    .iter()
                    .map(|p| {
                        (
                            p.flags.is_by_reference(),
                            p.flags.is_variadic(),
                            p.flags.has_default(),
                        )
                    })
                    .collect(),
            );
            found.returns.push(
                metadata
                    .return_type_metadata
                    .as_ref()
                    .map(|t| t.type_union.get_id().to_string()),
            );
        }
        Ok(ExpressionHookResult::Continue)
    }

    fn after_expression(
        &self,
        expr: &Expression<'_>,
        context: &mut HookContext<'_, '_>,
    ) -> HookResult<()> {
        if matches!(expr, Expression::Variable(Variable::Direct(_))) {
            if let Some(ty) = context.get_expression_type(expr) {
                self.0
                    .lock()
                    .unwrap()
                    .variables
                    .push((expr.span().start.offset, ty.get_id().to_string()));
            }
        }
        Ok(())
    }
}

fn file(source: &str) -> File {
    File::new(
        Cow::Borrowed(b"fresh.php"),
        FileType::Host,
        None,
        Cow::Owned(source.as_bytes().to_vec()),
    )
}

fn scan(source: &str) -> CodebaseMetadata {
    let file = file(source);
    let arena = LocalArena::new();
    let program = parse_file(&arena, &file);
    assert!(!program.has_errors());
    let names = NameResolver::new(&arena).resolve(program);
    let mut codebase = scan_program(&arena, &file, program, &names, Settings::default().version);
    populate_codebase(
        &mut codebase,
        &mut SymbolReferences::new(),
        Default::default(),
        Default::default(),
    );
    codebase
}

fn analyze(before: &str, after: &str) -> (Observed, Vec<(String, u32)>) {
    let base = scan(before);
    let normal = analyze_with_base(&base, after, false, false);
    let tracked = analyze_with_base(&base, after, true, false);
    assert_eq!(normal, tracked, "native tracking must remain passive");
    normal
}

fn analyze_with_base(
    base: &CodebaseMetadata,
    after: &str,
    track_native_types: bool,
    defaults: bool,
) -> (Observed, Vec<(String, u32)>) {
    let file = file(after);
    let arena = LocalArena::new();
    let program = parse_file(&arena, &file);
    assert!(!program.has_errors());
    let names = NameResolver::new(&arena).resolve(program);
    let mut scanned = scan_program(&arena, &file, program, &names, Settings::default().version);
    scanned.set_file_signature(file.id, build_file_signature(&file, program, &names));
    let current = base.with_file_closures(file.id, scanned);
    let found = Arc::<Mutex<Observed>>::default();
    let mut registry = if defaults {
        create_registry_with_plugins(&[], false)
    } else {
        PluginRegistry::new()
    };
    registry.register_expression_hook(Observer(found.clone()));
    let mut result = AnalysisResult::new(SymbolReferences::new());
    let settings = Settings {
        track_native_types,
        ..Settings::default()
    };
    Analyzer::new(&arena, &file, &names, &current, &registry, settings)
        .analyze_with_artifacts(program, &mut result)
        .expect(Observer::meta().description);
    let issues = result
        .issues
        .iter()
        .filter_map(|issue| {
            let primary = issue.annotations.iter().find(|a| a.kind.is_primary())?;
            Some((issue.code.as_ref()?.to_string(), primary.span.start.offset))
        })
        .collect();
    let found = std::mem::take(&mut *found.lock().unwrap());
    let at = after.rfind("$later;").unwrap() as u32;
    assert!(
        found.variables.contains(&(at, "int(29)".into())),
        "analysis must reach the later expression: {found:?}"
    );
    (found, issues)
}

#[test]
fn colliding_positions_use_the_current_signature_and_named_arguments() {
    let tail =
        "$a(value: 7); $b(value: 'ok'); $a(value: 'bad'); $b(value: 7); $later = 29; $later;";
    let before = format!(
        "<?php\n$a = fn (int    $value): int    => $value;\n$b = fn (string $value): string => $value;\n{tail}"
    );
    let after = format!(
        "<?php\n$b = fn (string $value): string => $value;\n$a = fn (int    $value): int    => $value;\n{tail}"
    );
    assert_eq!(before.len(), after.len());
    let (found, issues) = analyze(&before, &after);
    assert_eq!(found.parameters, [vec!["string"], vec!["int"]]);
    assert_eq!(found.returns, [Some("string".into()), Some("int".into())]);
    assert_eq!(
        issues,
        [
            (
                "invalid-argument".into(),
                after.find("'bad'").unwrap() as u32
            ),
            (
                "invalid-argument".into(),
                (after.find("$b(value: 7)").unwrap() + 10) as u32
            ),
            (
                "unused-statement".into(),
                after.rfind("$later;").unwrap() as u32
            ),
        ]
    );
}

#[test]
fn same_key_parameter_edits_refresh_in_both_directions() {
    for (old, new) in [("int   ", "string"), ("string", "int   ")] {
        let before = format!("<?php $f = fn ({old} $value) => $value; $later = 29; $later;");
        let after = format!("<?php $f = fn ({new} $value) => $value; $later = 29; $later;");
        assert_eq!(before.len(), after.len());
        let (found, _) = analyze(&before, &after);
        assert_eq!(found.parameters, [vec![new.trim()]]);
        let body = (after.find("=> $value").unwrap() + 3) as u32;
        assert!(found.variables.contains(&(body, new.trim().into())));
    }
}

#[test]
fn same_key_return_annotations_refresh_in_both_directions() {
    for (old, new) in [("int   ", "string"), ("string", "int   ")] {
        let before = format!("<?php $f = fn (): {old} => 'x'; $later = 29; $later;");
        let after = format!("<?php $f = fn (): {new} => 'x'; $later = 29; $later;");
        assert_eq!(before.len(), after.len());
        let (found, issues) = analyze(&before, &after);
        assert_eq!(found.returns, [Some(new.trim().into())]);
        assert_eq!(
            issues
                .iter()
                .any(|(code, _)| code == "invalid-return-statement"),
            new.trim() == "int"
        );
    }
}

#[test]
fn same_key_docblocks_refresh_without_entering_the_native_lane() {
    for (old, new) in [("int   ", "string"), ("string", "int   ")] {
        let before = format!(
            "<?php $f = /** @param {old} $value */ fn ($value) => $value; $later = 29; $later;"
        );
        let after = format!(
            "<?php $f = /** @param {new} $value */ fn ($value) => $value; $later = 29; $later;"
        );
        assert_eq!(before.len(), after.len());
        let (found, _) = analyze(&before, &after);
        assert_eq!(found.parameters, [vec![new.trim()]]);
        let native_before = before.replace(&format!("/** @param {old} $value */"), &" ".repeat(27));
        let native_after = after.replace(&format!("/** @param {new} $value */"), &" ".repeat(27));
        let (native, _) = analyze(&native_before, &native_after);
        assert_eq!(native.parameters, [vec!["untyped"]]);
    }
}

#[test]
fn moved_nested_closures_keep_captures_defaults_references_and_variadics() {
    let before = "<?php\n$seed = 'captured'; $count = 0;\n$outer = function (string &$value, string ...$extra) use ($seed, &$count): string {\n    $count = 1;\n    $inner = fn (string $suffix = '!'): string => $seed . $value . $suffix;\n    $inner();\n    return $inner(suffix: '!');\n};\n$value = 'value'; $outer(value: $value, more: 'extra'); $later = 29; $later;";
    let after = before.replacen("$outer =", "\n\n$outer =", 1);
    let (found, issues) = analyze(before, &after);
    assert_eq!(found.parameters[0], ["string", "string"]);
    assert_eq!(
        found.parameter_flags[0],
        [(true, false, false), (false, true, false)]
    );
    // By-reference invocation can revisit the body. Each nested visit must use the
    // same current signature, rather than asserting an analyzer replay count.
    assert!(found.parameters.len() >= 2);
    assert!(
        found.parameters[1..]
            .iter()
            .all(|parameters| parameters == &["string"])
    );
    assert!(
        found.parameter_flags[1..]
            .iter()
            .all(|flags| flags == &[(false, false, true)])
    );
    assert!(found.returns.iter().all(|r| r.as_deref() == Some("string")));
    let captured = after.find("=> $seed").unwrap() as u32 + 3;
    assert!(
        found
            .variables
            .contains(&(captured, "string('captured')".into())),
        "{found:?}"
    );
    assert!(
        !issues.iter().any(|(code, _)| [
            "invalid-argument",
            "invalid-named-argument",
            "unknown-named-argument"
        ]
        .contains(&code.as_str())),
        "{issues:?}"
    );
    assert!(
        !issues
            .iter()
            .any(|(code, _)| code.contains("too-few-argument")),
        "{issues:?}"
    );
}

fn fresh_scan(file: &File) -> CodebaseMetadata {
    let arena = LocalArena::new();
    let program = parse_file(&arena, file);
    assert!(!program.has_errors());
    let names = NameResolver::new(&arena).resolve(program);
    let mut scanned = scan_program(&arena, file, program, &names, Settings::default().version);
    scanned.set_file_signature(file.id, build_file_signature(file, program, &names));
    scanned
}

#[test]
fn nested_templates_use_current_parents_and_project_symbol_kinds() {
    let source = "<?php namespace App; use Library\\Contract as Imported;\nclass Scope { public function make() {\n$outer = /**\n * @template T of Imported\n * @param T $value\n * @return T\n */ function (Imported $value): Imported {\n    $inner = /** @param T $nested */ fn (Imported $nested): Imported => $nested;\n    return $inner(nested: $value);\n};\n} }";
    let mut base = scan(source);
    let library = File::new(
        Cow::Borrowed(b"library.php"),
        FileType::Host,
        None,
        Cow::Borrowed(b"<?php namespace Library; enum Contract { case One; }"),
    );
    base.extend(fresh_scan(&library));
    populate_codebase(
        &mut base,
        &mut SymbolReferences::new(),
        Default::default(),
        Default::default(),
    );
    let moved = file(&source.replace("$outer =", "\n\n$outer ="));
    let snapshot = base.with_file_closures(moved.id, fresh_scan(&moved));
    let outer = snapshot
        .function_likes
        .values()
        .find(|m| m.kind.is_closure())
        .unwrap();
    let inner = snapshot
        .function_likes
        .values()
        .find(|m| m.kind.is_arrow_function())
        .unwrap();
    let template = &outer.template_types[&word("T")];
    assert_eq!(
        template.defining_entity,
        GenericParent::FunctionLike((empty_word(), outer.name))
    );
    assert_eq!(
        template.constraint.get_id().as_str(),
        Some("enum(Library\\Contract)")
    );
    let inherited = inner
        .type_resolution_context
        .as_ref()
        .unwrap()
        .get_template_definition(word("T"))
        .unwrap();
    assert_eq!(inherited[0].defining_entity, template.defining_entity);
    assert!(outer.flags.is_populated() && inner.flags.is_populated());
}

#[test]
fn replacement_masks_deleted_keys_shares_project_backing_and_detaches_only_on_mutation() {
    let original = "<?php class Kept {} class_alias(Kept::class, 'AliasKept'); function kept(): int { return 1; } $old = fn (): int => 1;";
    let mut base = scan(original);
    let other = File::new(
        Cow::Borrowed(b"other.php"),
        FileType::Host,
        None,
        Cow::Borrowed(b"<?php $other = fn (): string => 'other';"),
    );
    base.extend(fresh_scan(&other));
    let old_key = *base
        .function_likes
        .iter()
        .find(|(_, m)| m.kind.is_arrow_function() && m.span.file_id == file(original).id)
        .unwrap()
        .0;
    let current_file = file("<?php $new = function (): string { return 'new'; };");
    let snapshot = base.with_file_closures(current_file.id, fresh_scan(&current_file));
    assert!(!snapshot.function_likes.contains_key(&old_key));
    assert!(base.function_likes.contains_key(&old_key));
    assert_eq!(snapshot.function_likes.len(), 3);
    assert_eq!(snapshot.function_likes.keys().count(), 3);
    assert_eq!(snapshot.function_likes.values().count(), 3);
    assert!(!snapshot.function_likes.is_empty());
    assert!(std::ptr::eq(&*snapshot.class_likes, &*base.class_likes));
    assert!(std::ptr::eq(&*snapshot.symbols, &*base.symbols));
    assert!(std::ptr::eq(
        &*snapshot.file_signatures,
        &*base.file_signatures
    ));
    assert!(std::ptr::eq(
        snapshot.get_function(b"kept").unwrap(),
        base.get_function(b"kept").unwrap()
    ));
    let new_key = *snapshot
        .function_likes
        .iter()
        .find(|(_, m)| m.kind.is_closure())
        .unwrap()
        .0;
    assert_eq!(
        snapshot.function_likes[&new_key]
            .return_type_metadata
            .as_ref()
            .unwrap()
            .type_union
            .get_id()
            .as_str(),
        Some("string")
    );
    let other_metadata = base
        .function_likes
        .values()
        .find(|m| m.span.file_id == other.id)
        .unwrap();
    assert!(std::ptr::eq(
        &**other_metadata,
        &**snapshot
            .function_likes
            .get(&(empty_word(), other_metadata.name))
            .unwrap()
    ));

    // The existing prelude persistence API must serialize the effective table, not
    // its shared hidden base or an implementation-specific overlay representation.
    let prelude = mago_prelude::Prelude {
        metadata: snapshot.clone(),
        ..Default::default()
    };
    let decoded = mago_prelude::Prelude::decode(&prelude.encode().unwrap()).unwrap();
    assert_eq!(decoded.metadata, snapshot);
    assert!(!decoded.metadata.function_likes.contains_key(&old_key));
    assert_ne!(decoded.metadata, base);

    // A later replacement cannot resurrect either the original or the preceding overlay.
    let empty = file("<?php // all anonymous functions removed\n");
    let deleted = snapshot.with_file_closures(empty.id, fresh_scan(&empty));
    assert_eq!(deleted.function_likes.len(), 2);
    assert!(
        deleted
            .function_likes
            .values()
            .all(|m| m.span.file_id != empty.id || m.kind.is_function())
    );
    let all_deleted = deleted.with_file_closures(other.id, CodebaseMetadata::new());
    assert_eq!(all_deleted.function_likes.len(), 1);

    // Mutable collection APIs materialize the effective view, never the hidden base.
    let mut detached = snapshot.clone();
    let key = (empty_word(), word("kept"));
    detached
        .function_likes
        .get_mut(&key)
        .unwrap()
        .return_type_metadata = None;
    assert!(
        snapshot
            .get_function(b"kept")
            .unwrap()
            .return_type_metadata
            .is_some()
    );
    let removed = detached.function_likes.remove(&key).unwrap();
    assert!(!detached.function_likes.contains_key(&old_key));
    detached.function_likes.extend([(key, removed.clone())]);
    assert!(detached.function_likes.contains_key(&key));
    assert!(detached.function_likes.insert(key, removed).is_some());
    assert!(
        detached
            .get_function(b"kept")
            .unwrap()
            .return_type_metadata
            .is_none()
    );
    assert_eq!(detached.function_likes.values_mut().count(), 3);
    assert_eq!(detached.function_likes.into_iter().count(), 3);
    assert!(snapshot.function_likes.contains_key(&key));
    assert!(base.function_likes.contains_key(&old_key));

    let only_closure = scan("<?php $old = fn () => 1;");
    let emptied = only_closure.with_file_closures(empty.id, fresh_scan(&empty));
    assert!(emptied.function_likes.is_empty());
    assert_eq!(emptied.function_likes.len(), 0);
}

#[test]
fn shared_scans_still_move_into_patch_metadata_without_changing_the_source_snapshot() {
    let patch_file = File::new(
        Cow::Borrowed(b"patch.php"),
        FileType::Patch,
        None,
        Cow::Borrowed(
            b"<?php class Patched {} const FLAG = 1; function patched(): int { return 1; }",
        ),
    );
    let mut patch = fresh_scan(&patch_file);
    let original = patch.clone();
    patch.convert_partial_to_patch();
    assert!(
        patch.class_likes.is_empty()
            && patch.function_likes.is_empty()
            && patch.constants.is_empty()
    );
    assert!(!patch.symbols.contains(word("patched")));
    assert_eq!(patch.patch_class_likes.len(), 1);
    assert_eq!(patch.patch_function_likes.len(), 1);
    assert_eq!(patch.patch_constants.len(), 1);
    assert_eq!(original.class_likes.len(), 1);
    assert_eq!(original.function_likes.len(), 1);
    assert_eq!(original.constants.len(), 1);
    assert!(original.symbols.contains(word("patched")));
}

#[test]
fn stdlib_plugins_and_raw_callback_gather_see_current_signatures() {
    let generic = "/**\n * @template R\n * @param callable(): R $callback\n * @return R\n */\nfunction gather(callable $callback) { return $callback(); }\n";
    for helper in ["", generic] {
        for (old, new) in [("int   ", "string"), ("string", "int   ")] {
            let call = |declaration: &str| {
                if helper.is_empty() {
                    format!("array_map(fn (int $value): {declaration} => $value, [1, 2])")
                } else {
                    let default = if declaration.trim() == "int" {
                        "0"
                    } else {
                        "'x'"
                    };
                    format!("gather(fn ({declaration} $value = {default}) => $value)")
                }
            };
            let source = |declaration| {
                format!(
                    "<?php {helper}$mapped = {}; $mapped; $later = 29; $later;",
                    call(declaration)
                )
            };
            let before = source(old);
            let after = source(new);
            let mut base = mago_prelude::Prelude::build().metadata;
            base.extend(fresh_scan(&file(&before)));
            populate_codebase(
                &mut base,
                &mut SymbolReferences::new(),
                Default::default(),
                Default::default(),
            );
            for native in [false, true] {
                let (found, _) = analyze_with_base(&base, &after, native, true);
                let at = after.find("; $mapped;").unwrap() as u32 + 2;
                let ty = &found
                    .variables
                    .iter()
                    .find(|(offset, _)| *offset == at)
                    .unwrap()
                    .1;
                assert!(
                    ty.contains(new.trim()),
                    "current callback output must reach its caller: {ty}"
                );
                assert!(
                    !ty.contains(old.trim()),
                    "old callback output must not leak: {ty}"
                );
            }
        }
    }
}

#[test]
fn moved_named_scopes_are_owned_but_real_duplicates_still_report() {
    for declaration in [
        "class Holder { public function run() { $f = fn (int $value): int => $value; } }",
        "trait Holder { public function run() { $f = fn (int $value): int => $value; } }",
        "enum Holder { case One; public function run() { $f = fn (int $value): int => $value; } }",
        "function run() { $f = fn (int $value): int => $value; }",
        "interface Holder { public function run(): string; }",
    ] {
        let before = format!("<?php {declaration} $later = 29; $later;");
        let after = before.replace("<?php ", "<?php\n\n");
        let (found, issues) = analyze(&before, &after);
        assert!(
            !issues
                .iter()
                .any(|(code, _)| code == "duplicate-definition"),
            "{declaration}: {issues:?}"
        );
        if !declaration.starts_with("interface") {
            assert_eq!(
                found.parameters,
                [vec!["int"]],
                "the moved body must be analyzed: {declaration}"
            );
        }
    }

    // A same-file duplicate must not be accepted merely because its FileId matches.
    for declaration in [
        "class Duplicate {} class Duplicate {}",
        "function duplicate() {} function duplicate() {}",
    ] {
        let before = format!("<?php {declaration} $later = 29; $later;");
        let after = before.replace("<?php ", "<?php\n\n");
        let (_, issues) = analyze(&before, &after);
        assert_eq!(
            issues
                .iter()
                .filter(|(code, _)| code == "duplicate-definition")
                .count(),
            1,
            "{issues:?}"
        );
    }

    let before = "<?php class Methods { public function run() { $f = fn (int $value) => $value; } public function run() { $f = fn (string $value) => $value; } } $later = 29; $later;";
    let after = before.replace("<?php ", "<?php\n\n");
    let (found, _) = analyze(before, &after);
    // The scanner selects the first method (entry.or_insert). A FileId-only
    // shortcut would also publish the duplicate body's distinct string signature.
    assert_eq!(found.parameters, [vec!["int"]]);

    // A declaration selected from a different project file keeps its ownership.
    let other = File::new(
        Cow::Borrowed(b"owner.php"),
        FileType::Host,
        None,
        Cow::Borrowed(b"<?php class Taken {}"),
    );
    let mut base = fresh_scan(&other);
    populate_codebase(
        &mut base,
        &mut SymbolReferences::new(),
        Default::default(),
        Default::default(),
    );
    let after = "<?php class Taken { public function run() { $f = fn (string $value) => $value; } } $later = 29; $later;";
    let (found, issues) = analyze_with_base(&base, after, false, false);
    assert!(
        found.parameters.is_empty(),
        "a foreign-owned duplicate must still be skipped"
    );
    assert_eq!(
        issues
            .iter()
            .filter(|(code, _)| code == "duplicate-definition")
            .count(),
        1
    );
}
