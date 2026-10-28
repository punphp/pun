//! New array values must not inherit callable constraints from their inputs.

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
