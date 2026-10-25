<?php
namespace RenamedNamedArgument;

/**
 * @template TName of string
 * @param TName&'run' $name
 */
function refinedName(Contract $object, $name): void {
    $object->$name(value: 'refined template name');
}
/**
 * @param 'run'|int $name
 * @param callable-string $callbackName
 */
function mixedName(Contract $object, $name, string $callbackName): void {
    $object->$name(value: 'a non-string alternative vetoes constant strings');
    $object->$callbackName(value: 'callable-string is not a constant method name');
}
/**
 * @template TArgs of array{value: string}
 * @param TArgs $args
 */
function templateArray(Contract $object, $args): void {
    $object->run(...$args);
}
/**
 * @param array{value: string}|string $mixed
 * @param array{value: string, ...<string, string>} $open
 * @param array{0: class-string, 1: string}&callable $callback
 */
function refinedArrays(Contract $object, $mixed, array $open, array $callback): void {
    $object->run(...$mixed);
    $object->run(...$open);
    $object->run(...$callback, second: 1);
    $object->run(...$callback, 'extra positional', value: 'not visited beyond signature');
    $object->run(...$open, 'one', 'two', value: 'nonempty unknown unpack occupies a position');
    $object->run('one', ...$open, 'two', value: 'no fallback when arguments already exist');
}

/**
 * @template TOpen of (array{value: string, ...<string, string>}&non-empty-array)
 * @template TClosed of (array{value: string, ...<never, never>}&non-empty-array)
 * @param TOpen $open
 * @param TClosed $closed
 */
function explicitSealing(Contract $object, $open, $closed): void {
    $object->run(...$open);
    $object->run('one', ...$open, 'two', value: 'open shape contributes no named key');
    $object->run(...$closed);
}

/**
 * @template TOpen of (array{value: string, ...<string, string>}|array{second: int, ...<string, int>})
 * @template TCallback of (array{0: class-string, 1: string}&callable)
 * @param TOpen $open
 * @param TCallback $callback
 */
function compoundTemplates(Contract $object, $open, $callback): void {
    $object->run(...$open);
    $object->run('one', ...$open, 'two', value: 'unsealed union expands no named keys');
    $object->run(...$callback, second: 1);
    $object->run(...$callback, 'extra positional', value: 'past callable array offsets');
}

/** @param callable&array $callback */
function callableArray(Contract $object, array $callback): void {
    $object->run(...$callback, second: 1);
    $object->run(...$callback, 'extra positional', value: 'past guaranteed callable offsets');
}

/** @param array{value: string, ...<string, string>} $open */
function refinedOffset(Contract $object, array $open): void {
    assert(isset($open['value']));
    $object->run(...$open);
    $object->run('one', ...$open, 'two', value: 'required offset preserves a nonempty open shape');
}
