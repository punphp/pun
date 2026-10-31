//! New array values must not inherit callable constraints from their inputs.

#[test]
fn array_map_selected_acceptors_resolve_actual_template_arguments() {
    check_native_array_map("acceptor-values");
}

#[test]
fn array_map_selected_acceptors_preserve_known_keys_and_open_shape_values() {
    check_native_array_map("acceptor-shapes");
}

fn check_native_array_map(case: &str) {
    use std::borrow::Cow;

    use mago_allocator::LocalArena;
    use mago_analyzer::{Analyzer, analysis_result::AnalysisResult, plugin::PluginRegistry, settings::Settings};
    use mago_codex::ttype::atomic::array::TArray;
    use mago_codex::ttype::union::TUnion;
    use mago_codex::{populator::populate_codebase, reference::SymbolReferences, scanner::scan_program};
    use mago_database::file::File;
    use mago_names::resolver::NameResolver;
    use mago_php_version::PHPVersion;
    use mago_span::HasSpan;
    use mago_syntax::{cst::*, parser::parse_file, walker::MutWalker};

    // Observe the native public artifact, not the shared converter. Expected
    // values come from the complete, separately retained PHPStan capture.
    fn describe(ty: &TUnion) -> String {
        if let Some(value) = ty.get_single_literal_int_value() {
            value.to_string()
        } else if ty.is_int() {
            "int".into()
        } else if ty.is_bool() {
            "bool".into()
        } else if ty.is_string() {
            "string".into()
        } else if let Some(array) = ty.get_single_array() {
            assert!(array.callable().is_none());
            let (mut values, required, rest): (Vec<_>, _, _) = match array {
                TArray::List(list) => {
                    let items = list.known_elements.as_ref().unwrap();
                    let values = items
                        .iter()
                        .enumerate()
                        .map(|(index, (key, (optional, value)))| {
                            assert_eq!(*key, index);
                            assert!(!optional);
                            describe(value)
                        })
                        .collect();
                    (
                        values,
                        items.len(),
                        (!list.element_type.is_never()).then(|| format!("...<{}>", describe(&list.element_type))),
                    )
                }
                TArray::Keyed(keyed) => {
                    use mago_codex::ttype::atomic::array::key::ArrayKey;
                    let items = keyed.known_items.as_ref().unwrap();
                    let values = items
                        .iter()
                        .enumerate()
                        .map(|(index, (key, (optional, value)))| {
                            let value = describe(value);
                            match key {
                                ArrayKey::Integer(key) => {
                                    assert_eq!(*key, index as i64);
                                    assert!(!optional);
                                    value
                                }
                                ArrayKey::String(key) => format!(
                                    "{}{optional}: {value}",
                                    String::from_utf8_lossy(key.as_bytes()),
                                    optional = if *optional { "?" } else { "" }
                                ),
                                _ => panic!("unexpected key"),
                            }
                        })
                        .collect();
                    let required = items.values().filter(|(optional, _)| !optional).count();
                    let rest = keyed
                        .parameters
                        .as_ref()
                        .map(|(key, value)| format!("...<{}, {}>", describe(key), describe(value)));
                    (values, required, rest)
                }
            };
            assert_eq!(array.get_minimum_size(), required);
            values.extend(rest);
            format!("array{{{}}}", values.join(", "))
        } else {
            panic!("unexpected native result {ty:?}")
        }
    }

    #[derive(Default)]
    struct Dumps(Vec<(u32, mago_span::Span)>);
    impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Dumps {
        fn walk_in_function_call(&mut self, call: &'ast FunctionCall<'arena>, _: &mut ()) {
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

    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/array-map");
    let source = std::fs::read_to_string(root.join(format!("{case}.php"))).unwrap();
    let oracle: serde_json::Value =
        serde_json::from_slice(&std::fs::read(root.join(format!("{case}.json"))).unwrap()).unwrap();
    let arena = LocalArena::new();
    let file = File::ephemeral(Cow::Owned(case.as_bytes().to_vec()), Cow::Owned(source.as_bytes().to_vec()));
    let program = parse_file(&arena, &file);
    assert!(!program.has_errors());
    let names = NameResolver::new(&arena).resolve(program);
    let mut codebase = scan_program(&arena, &file, program, &names, PHPVersion::PHP85);
    let builtins = File::ephemeral(
        Cow::Borrowed(b"builtins.php"),
        Cow::Borrowed(
            b"<?php function array_map(?callable $callback, array $array, array ...$arrays): array { return []; }",
        ),
    );
    let declarations = parse_file(&arena, &builtins);
    let builtin_names = NameResolver::new(&arena).resolve(declarations);
    codebase.extend(scan_program(
        &arena,
        &builtins,
        declarations,
        &builtin_names,
        PHPVersion::PHP85,
    ));
    let mut references = SymbolReferences::new();
    populate_codebase(&mut codebase, &mut references, Default::default(), Default::default());
    let registry = PluginRegistry::with_library_providers();
    let artifacts = Analyzer::new(&arena, &file, &names, &codebase, &registry, Settings::default())
        .analyze_with_artifacts(program, &mut AnalysisResult::new(references))
        .unwrap();
    let mut dumps = Dumps::default();
    dumps.walk_program(program, &mut ());
    let actual: Vec<_> = dumps
        .0
        .into_iter()
        .map(|(offset, argument)| {
            let line = source.as_bytes()[..offset as usize]
                .iter()
                .filter(|b| **b == b'\n')
                .count()
                + 1;
            (
                line,
                format!(
                    "Dumped type: {}",
                    describe(artifacts.get_expression_type(&argument).unwrap())
                ),
            )
        })
        .collect();
    let expected: Vec<_> = oracle["files"][format!("{case}.php")]["messages"]
        .as_array()
        .unwrap()
        .iter()
        .map(|message| {
            (
                message["line"].as_u64().unwrap() as usize,
                message["message"].as_str().unwrap().to_owned(),
            )
        })
        .collect();
    assert_eq!(actual, expected);
}

#[test]
fn library_array_projections_produce_ordinary_values() {
    use std::borrow::Cow;

    use mago_allocator::LocalArena;
    use mago_analyzer::{Analyzer, analysis_result::AnalysisResult, plugin::PluginRegistry, settings::Settings};
    use mago_codex::ttype::get_array_parameters;
    use mago_codex::{populator::populate_codebase, reference::SymbolReferences, scanner::scan_program};
    use mago_database::file::File;
    use mago_names::resolver::NameResolver;
    use mago_php_version::PHPVersion;
    use mago_syntax::parser::parse_file;

    let source = r#"<?php
namespace {
function array_map(?callable $callback, array $array, array ...$arrays): array { return []; }
function array_pad(array $array, int $length, mixed $value): array { return []; }
function array_is_list(array $array): bool { return false; }
function is_array(mixed $value): bool { return false; }
/** @assert-if-true int $value */
function is_int(mixed $value): bool { return false; }
}
namespace Psl\Async {
/** @template T */
class Awaitable {}
function all(array $awaitables): array { return []; }
function concurrently(array $tasks): array { return []; }
}
namespace Psl\Type {
/** @template T */
interface TypeInterface {}
function shape(array $elements): mixed { return null; }
}
namespace Psl\Regex {
function capture_groups(array $groups): mixed { return null; }
}
namespace Flow\Types\DSL {
function type_structure(array $elements): mixed { return null; }
}
namespace Probe {
function malformed(): void {
    \array_map();
    \array_map(null);
    \array_map(null, 1);
}
/**
 * @param \Psl\Async\Awaitable<int> $awaitable
 * @param array<string, \Psl\Async\Awaitable<int>> $awaitables
 * @param array<string, \Closure(): int> $tasks
 * @param \Psl\Type\TypeInterface<int> $descriptor
 * @param list<int> $numbers
 * @param array<int, int> $open
 */
function probe($awaitable, array $awaitables, array $tasks, $descriptor, array $numbers, array $open, int $index, callable $callback, mixed $mixed): void {
    $a = \Psl\Async\all([$awaitable]);
    $b = \Psl\Async\all(['key' => $awaitable]);
    $c = \Psl\Async\all($awaitables);
    $d = \Psl\Async\concurrently([fn (): int => 1]);
    $e = \Psl\Async\concurrently(['key' => fn (): int => 1]);
    $f = \Psl\Async\concurrently($tasks);
    $g = \Psl\Type\shape([$descriptor]);
    $h = \Psl\Type\shape(['key' => $descriptor]);
    $i = \Psl\Regex\capture_groups(['name']);
    $j = \Flow\Types\DSL\type_structure(['key' => $descriptor]);
    $k = \array_map(null, $numbers, $numbers);
    $l = \array_pad($numbers, 3, 4);
    $m = \array_is_list(['key' => 1]);
    $openIsList = \array_is_list($open);
    if (\is_array($callback)) {
        $n = \array_is_list($callback);
    }
    $nullable = null;
    $nullable[$index] = 4;
    $nullable;
    $appendNull = null;
    $appendNull[] = 4;
    $appendNull;
    $shape = ['x' => 1];
    $shape[] = 4;
    $shape;
    $numbers[$index] = 4;
    $numbers;
    $open[] = 4;
    $open;
    $max = [9223372036854775807 => 1];
    $max[] = 4;
    $max;
    $delete = [1];
    unset($delete[4]);
    $delete;
    $combined = ['a' => 1] + ['b' => 2];
    $combined;
    if (\is_int($mixed['value'])) {
        $mixed;
        $mixed['value'];
    }
}
}
"#;
    let arena = LocalArena::new();
    let file = File::ephemeral(
        Cow::Borrowed(b"callable-array-values.php"),
        Cow::Borrowed(source.as_bytes()),
    );
    let program = parse_file(&arena, &file);
    assert!(!program.has_errors());
    let names = NameResolver::new(&arena).resolve(program);
    let mut codebase = scan_program(&arena, &file, program, &names, PHPVersion::PHP85);
    // This isolated native API test supplies the builtin declarations instead of loading the prelude.
    codebase
        .function_likes
        .get_mut(&(mago_word::empty_word(), mago_word::word("is_array")))
        .unwrap()
        .flags |= mago_codex::metadata::flags::MetadataFlags::BUILTIN;
    let mut references = SymbolReferences::new();
    populate_codebase(&mut codebase, &mut references, Default::default(), Default::default());
    let registry = PluginRegistry::with_library_providers();
    let artifacts = Analyzer::new(&arena, &file, &names, &codebase, &registry, Settings::default())
        .analyze_with_artifacts(program, &mut AnalysisResult::new(references))
        .unwrap();
    // PHPStan retains `array` for missing operands. With a null callback its
    // extension returns the second operand unchanged, even when it is not an array.
    for call in [r"\array_map()", r"\array_map(null)"] {
        let start = source.find(call).unwrap() as u32;
        assert!(artifacts.expression_types[&(start, start + call.len() as u32)].is_array());
    }
    let call = r"\array_map(null, 1)";
    let start = source.find(call).unwrap() as u32;
    assert_eq!(artifacts.expression_types[&(start, start + call.len() as u32)].get_single_literal_int_value(), Some(1));
    for (call, wrapped, minimum, strings) in [
        (r"\Psl\Async\all([$awaitable])", false, 1, false),
        (r"\Psl\Async\all(['key' => $awaitable])", false, 1, false),
        (r"\Psl\Async\all($awaitables)", false, 0, false),
        (r"\Psl\Async\concurrently([fn (): int => 1])", false, 1, false),
        (r"\Psl\Async\concurrently(['key' => fn (): int => 1])", false, 1, false),
        (r"\Psl\Async\concurrently($tasks)", false, 0, false),
        (r"\Psl\Type\shape([$descriptor])", true, 1, false),
        (r"\Psl\Type\shape(['key' => $descriptor])", true, 1, false),
        (r"\Psl\Regex\capture_groups(['name'])", true, 2, true),
        (r"\array_pad($numbers, 3, 4)", false, 3, false),
        (
            r"\Flow\Types\DSL\type_structure(['key' => $descriptor])",
            true,
            1,
            false,
        ),
    ] {
        let start = source.find(call).unwrap() as u32;
        let ty = artifacts
            .expression_types
            .get(&(start, start + call.len() as u32))
            .unwrap();
        let ty = if wrapped {
            &ty.get_single_named_object().unwrap().type_parameters.as_ref().unwrap()[0]
        } else {
            ty
        };
        let array = ty.get_single_array().unwrap();
        assert!(array.callable().is_none(), "{call} must create an ordinary array");
        assert_eq!(array.get_minimum_size(), minimum, "{call}");
        let (_, values) = get_array_parameters(array, &codebase);
        if strings {
            assert!(values.is_string(), "{call}");
        } else {
            assert!(values.is_int(), "{call}");
        }
    }
    let zipped_call = r"\array_map(null, $numbers, $numbers)";
    let start = source.find(zipped_call).unwrap() as u32;
    let zipped = &artifacts.expression_types[&(start, start + zipped_call.len() as u32)];
    let zipped = zipped.get_single_array().unwrap();
    assert!(zipped.callable().is_none());
    let (_, tuple) = get_array_parameters(zipped, &codebase);
    assert_eq!(
        tuple
            .get_single_array()
            .unwrap()
            .get_size()
            .get_single_literal_int_value(),
        Some(2)
    );
    for call in [r"\array_is_list(['key' => 1])", r"\array_is_list($open)", r"\array_is_list($callback)"] {
        let start = source.find(call).unwrap() as u32;
        assert_eq!(
            artifacts.expression_types[&(start, start + call.len() as u32)].as_ref(),
            &mago_codex::ttype::get_bool()
        );
    }
    for (variable, minimum) in [
        ("nullable", 1),
        ("appendNull", 1),
        ("shape", 2),
        ("numbers", 1),
        ("open", 1),
        ("max", 1),
        ("delete", 1),
        ("combined", 2),
    ] {
        let expression = format!("${variable};");
        let start = source.find(&expression).unwrap() as u32;
        let ty = &artifacts.expression_types[&(start, start + expression.len() as u32 - 1)];
        for atomic in ty.types.as_ref() {
            let mago_codex::ttype::atomic::TAtomic::Array(array) = atomic else {
                panic!("{variable} must be an array, got {atomic:?}");
            };
            assert!(array.callable().is_none(), "{variable}");
            assert_eq!(array.get_minimum_size(), minimum, "{variable}");
            assert!(get_array_parameters(array, &codebase).1.is_int(), "{variable}");
        }
    }
    let expression = "$mixed['value'];";
    let start = source.find(expression).unwrap() as u32;
    assert!(artifacts.expression_types[&(start, start + expression.len() as u32 - 1)].is_int());
    let start = source.find("$mixed;").unwrap() as u32;
    for atomic in artifacts.expression_types[&(start, start + 6)].types.as_ref() {
        if let mago_codex::ttype::atomic::TAtomic::Array(array) = atomic {
            assert!(array.callable().is_none());
        }
    }
}
