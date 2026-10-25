<?php
namespace RenamedNamedArgument;

use function PHPStan\dumpType;

/**
 * @template T of array{value: int|string, tag: 'identity', ...<string, int|string>}
 * @param T $value
 */
function templateIdentity($value): void {
    assert(is_string($value['value']));
    dumpType($value);
}

/**
 * @template T of (array{kind: 'left', value: string, ...<string, string>}|array{kind: 'right', value: string, ...<string, string>}|array{kind: 'integer', value: int, ...<string, int|string>})
 * @param T $value
 */
function asymmetricBranches(Contract $object, $value): void {
    if (is_string($value['value'])) {
        dumpType($value['kind']);
        dumpType($value['value']);
        $object->run(...$value);
    } else {
        dumpType($value['kind']);
        dumpType($value['value']);
        $object->run(...$value);
    }
    dumpType($value['kind']);
    dumpType($value['value']);
}

/**
 * @template T of (array{kind: 'string', value: string, ...<string, string>}|array{kind: 'integer', value: int, ...<string, int|string>})
 * @param T $value
 */
function reassignedTemplate(Contract $object, $value): void {
    assert(is_string($value['value']));
    $value = ['kind' => 'replacement', 'value' => 42];
    dumpType($value['value']);
    $object->run(...$value);
}

/**
 * @template T of (array{kind: 'string', value: string, probe: bool, ...<string, bool|string>}|array{kind: 'integer', value: int, probe: bool, ...<string, bool|int|string>})
 * @param T $value
 */
function nonmatchingOffset(Contract $object, $value): void {
    assert(is_bool($value['probe']));
    dumpType($value['kind']);
    dumpType($value['value']);
    $object->run(...$value);
}

/**
 * @template T of array{payload: array{kind: 'string', value: string}|array{kind: 'integer', value: int}}
 * @param T $value
 */
function nestedOffset(Contract $object, $value): void {
    assert(is_string($value['payload']['value']));
    dumpType($value['payload']['kind']);
    dumpType($value['payload']['value']);
    $object->run(...$value['payload']);
}

/**
 * @template Inner of (array{kind: 'string', value: string}|array{kind: 'integer', value: int})
 * @template Outer of array{payload: Inner}
 * @param Outer $value
 */
function nestedTemplates(Contract $object, $value): void {
    assert(is_string($value['payload']['value']));
    dumpType($value['payload']['kind']);
    dumpType($value['payload']['value']);
    dumpType($value['payload']);
    $object->run(...$value['payload']);
}

/**
 * @template Integers of array{kind: 'integer', value: int, ...<string, int|string>}
 * @template Strings of array{kind: 'string', value: string, ...<string, string>}
 * @param Integers|Strings $value
 */
function unionOfTemplates(Contract $object, $value): void {
    assert(is_string($value['value']));
    dumpType($value['kind']);
    dumpType($value['value']);
    $object->run(...$value);
}

/**
 * @template Integers of array{int}
 * @template Strings of array{string}
 * @param Integers|Strings $value
 */
function tupleTemplates($value): void {
    assert(is_string($value[0]));
    dumpType($value[0]);
}

interface SharedRoot { public function change(string $original): void; }
interface SharedLeft extends SharedRoot {}
interface SharedRight extends SharedRoot {}
interface OwnDiamond extends SharedLeft, SharedRight { public function change(string $original): void; }
class OwnDiamondSubtype implements OwnDiamond { public function change(string $renamed): void {} }
function inheritedDocDiamond(OwnDiamond $object): void { $object->change(original: 'shared undocumented ancestor'); }

/**
 * @template T of array
 * @param T&non-empty-array $value
 */
function opaqueArrayIntersection(Contract $object, $value): void {
    $object->run(...$value);
}

/**
 * @template T of array<string, int|string>
 * @param T $value
 */
function refinedGeneralArray(Contract $object, $value): void {
    assert(is_string($value['value']));
    assert(is_int($value['second']));
    dumpType($value['value']);
    dumpType($value['second']);
    $object->run(...$value);
}

/** @template T of array{value: int|string} */
class OffsetContainer {
    /** @param T $value */
    public function refine($value): void {
        assert(is_string($value['value']));
        dumpType($value['value']);
    }
}

/**
 * @template T of \ArrayAccess<string, int|string>
 * @param T $value
 */
function nonArrayConstraint($value): void {
    assert(is_string($value['value']));
    dumpType($value['value']);
}

/**
 * @template T of (array{kind: 'left', value: string}|array{kind: 'right', value: string}|array{kind: 'integer', value: int})
 * @param T $value
 */
function closedAlternatives(Contract $object, $value): void {
    assert(is_string($value['value']));
    dumpType($value['kind']);
    dumpType($value['value']);
    $object->run(...$value);
}

/**
 * @template Integers of list{int}
 * @template Strings of list{string}
 * @param Integers|Strings $value
 */
function listShapeTemplates($value): void {
    assert(is_string($value[0]));
    dumpType($value);
    dumpType($value[0]);
}

/**
 * @template T of list{int|string}
 * @param T $value
 */
function narrowedListTemplate($value): void {
    assert(is_string($value[0]));
    dumpType($value);
    dumpType($value[0]);
}
