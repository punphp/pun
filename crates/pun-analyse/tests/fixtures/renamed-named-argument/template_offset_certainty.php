<?php
namespace RenamedNamedArgument;

use function PHPStan\dumpType;

/** @param array{present: bool, value: string} $value */
function absentOffset(Contract $object, array $value): void {
    assert(!isset($value['missing']));
    assert(!isset($value['missing']));
    dumpType($value);
    $object->run(...$value);
}

/** @param array{probe?: bool, value: string} $value */
function optionalOffset(Contract $object, array $value): void {
    assert(!isset($value['probe']));
    assert(!isset($value['probe']));
    dumpType($value);
    $object->run(...$value);
}

/** @param list{string, 1?: bool} $value */
function optionalListOffset(array $value): void {
    assert(!isset($value[1]));
    assert(!isset($value[1]));
    dumpType($value);
}

/** @param array{present: bool, value: string} $value */
function unreachableOffset(Contract $object, array $value): void {
    if (is_string($value['present'])) {
        assert(is_bool($value['present']));
        dumpType($value['present']);
    }
    dumpType($value);
    $object->run(...$value);
}

/**
 * @template T of (array{kind: 'left', probe: bool, value: string}|array{kind: 'right', probe: bool, value: int})
 * @param T $value
 */
function offsetUnionAlternatives(Contract $object, $value): void {
    assert(is_bool($value['probe']));
    dumpType($value);
    $object->run(...$value);
}

/** @param array{probe?: bool, value: string} $value */
function optionalSibling(Contract $object, array $value): void {
    assert(is_string($value['value']));
    dumpType($value);
    $object->run(...$value);
}

/** @param list{bool, string} $value */
function requiredListOffset(array $value): void {
    assert(is_bool($value[0]));
    dumpType($value);
}

function stringOffset(string $value): void {
    assert(is_string($value[0]));
    dumpType($value);
}
