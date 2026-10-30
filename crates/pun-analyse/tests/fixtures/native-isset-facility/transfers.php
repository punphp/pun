<?php

namespace NativeIssetFacility;

/** @param array{left: int, right: ?string, optional?: int, nested: array{leaf: int}} $shape */
function parameter(array $shape, string $key): void
{
    $shape['left']; // observe
    isset($shape['left']); // observe
    isset($shape['right'], $shape['optional']); // observe
    $shape['nested']['leaf'] ?? 0; // observe
    $shape[$key = 'right'] ?? ''; // observe
    $key; // observe
}

/** @param string $value */
function operators(?string $value): void
{
    isset($value); // observe
    $value ?? 0; // observe
    $value ??= 0; // observe
    $value; // observe
}

function arrays(?string $value, string $key): void
{
    $shape = ['left' => 1, 'right' => $value, 'nested' => ['leaf' => 7]];
    $shape['left']; // observe
    $shape['right']; // observe
    $shape['nested']['leaf']; // observe
    $shape['absent']; // observe
    isset($shape['left']); // observe
    isset($shape['right']); // observe
    isset($shape['absent']); // observe
    $shape['absent'] ?? 5; // observe
    $shape['right'] ?? 'fallback'; // observe
    $shape[$key]; // observe
    $key = 'left';
    $shape[$key]; // observe
}

function optional(bool $flag): void
{
    if ($flag) {
        $shape = ['key' => 1];
    } else {
        $shape = [];
    }
    $shape['key']; // observe
    isset($shape['key']); // observe
    $shape['key'] ?? 2; // observe
}

function effects(?string $value): void
{
    $changed = 1;
    $value ?? ($changed = 2); // observe
    $changed; // observe
    $value ??= ($changed = 3); // observe
    $value; // observe
    $changed; // observe
}

function skipped(): void
{
    $value = 1;
    $changed = 'before';
    $value ?? ($changed = 'counterfactual'); // observe
    $changed; // observe
    $value ??= 2; // observe
    $value; // observe
}

function absent(): void
{
    isset($missing); // observe
    $missing ?? 'default'; // observe
    $missing ??= 5; // observe
    $missing; // observe
}

function isolated(?string $value): void
{
    /** @var string $copy */
    $copy = $value;
    isset($copy); // observe
    $copy ?? 3; // observe
    $copy; // observe
}

function unsupported(int $value): void
{
    $value; // observe
    $value << 1; // unavailable
    isset($value); // unavailable
    $value = 2;
    $value ??= 3; // unavailable
    $value; // unavailable
}

function offsetWrite(?int $value): void
{
    $shape = ['key' => $value];
    $shape['key'] ??= 4; // observe
    $shape['key']; // observe
}

function incoming(?int $value): void
{
    $shape = ['first' => $value, 'second' => null];
    isset($shape['first'], $shape[$key = 'second']); // observe
    $key; // observe
    $shape[$key = 'first'] ?? ($later = $value); // observe
    $later; // observe
    $shape[$key = 'first'] ??= ($value = 6); // observe
    $shape['first']; // observe
    $value; // observe
}

/** @param non-empty-string $value */
function selectedPath(?string $value): void
{
    $value ?? ($copy = $value); // observe
    $copy; // observe
    $value; // observe
}

function lists(?int $value, int $index, string $key): void
{
    $shape = [1, $value];
    $shape[0]; // observe
    $shape[1]; // observe
    $shape[2]; // observe
    $shape[$index]; // observe
    $shape[$key]; // observe
    $shape[1] ??= 2; // observe
    $shape[1]; // observe
}

function nullSlots(): void
{
    $shape = ['outer' => ['leaf' => null]];
    isset($shape['outer']['leaf']); // observe
    $shape['outer']['leaf'] ?? 4; // observe
}

function unsupportedArray(string $key): void
{
    $shape = [$key => 1]; // unavailable
    isset($shape[$key]); // unavailable
    $shape = ['key' => 2];
    $shape['key']; // unavailable
}

function unsupportedWrite(mixed $value): void
{
    $value['key'] ??= 4;
    $value; // unavailable
}

function nullableHolder(bool $flag): void
{
    if ($flag) {
        $shape = ['key' => 1];
    } else {
        $shape = null;
    }
    $shape['key']; // observe
    isset($shape['key']); // observe
    $shape['key'] ?? 5; // observe
    $shape['key'] ??= 2; // observe
    $shape['key']; // observe
}

function nullHolder(): void
{
    $shape = null;
    isset($shape['key']); // observe
    $shape['key'] ??= 7; // observe
    $shape['key']; // observe
}

function nestedWrite(?int $value): void
{
    $shape = ['outer' => ['leaf' => $value]];
    $shape['outer']['leaf'] ??= 7; // observe
    $shape['outer']['leaf']; // observe
}

function unavailableUnpack(array $values): void
{
    $shape = [...$values]; // unavailable
    $shape; // unavailable
}

function unavailableKey(array $key): void
{
    $shape = ['key' => 1];
    $shape[$key]; // unavailable
    $shape; // unavailable
}

final class NativeBox
{
    public int $key = 1;
}

function propertyBoundary(NativeBox $value): void
{
    isset($value->key); // unavailable-result
    $value; // unavailable
}

function unavailableOperand(int $value): void
{
    isset($shape[$value << 1]); // unavailable
    $value; // unavailable
}

function nullContinuation(?int $value): void
{
    $value ?? ($copy = $value); // observe
    $copy; // observe
    $value; // observe
}

function neverContinuation(?int $value): void
{
    // Throw's native value transfer is outside this facility, even though the
    // full-selected terminating RHS does not contribute a continuation edge.
    $value ?? throw new \Exception();
    $value; // unavailable
}

function incomingHolder(int $value): void
{
    $shape = ['key' => 1];
    isset($shape['key'], $shape[$value = 2]); // observe
    $value; // observe
}

function newlySupported(array $value, ?int $nullable): void
{
    $value; // observe
    $array = [2]; // observe
    $nullable ?? 2; // observe
    $value = 99;
    $value; // observe
}

function receiverWrite(): void
{
    $shape = ['key' => null, 'old' => 1];
    $shape['key'] ??= ($shape = ['changed' => 2])['changed']; // observe
    $shape['changed']; // observe
    $shape['key']; // observe
}

function offsetContinuation(?int $value): void
{
    $shape = ['key' => $value];
    $shape['key'] ?? ($copy = $shape['key']); // observe
    $copy; // observe
}

/** @param array{key: ?int} $shape */
function documentedContinuation(array $shape): void
{
    $shape['key'] ?? ($copy = $shape['key']); // observe
    $copy; // observe
}

/** @param array{key: ?int} $shape */
function documentedHolderContinuation(array $shape): void
{
    $shape['key'] ?? ($copy = $shape); // observe
}

/** @param array{nested: array{leaf: ?int}} $shape */
function documentedNestedHolderContinuation(array $shape): void
{
    $shape['nested']['leaf'] ?? ($copy = $shape); // observe
}

/** @param array{nested: array{leaf: ?int}} $shape */
function documentedNestedContinuation(array $shape): void
{
    $shape['nested']['leaf'] ?? ($copy = $shape['nested']['leaf']); // observe
    $copy; // observe
}

function nullableHolderContinuation(bool $flag): void
{
    if ($flag) {
        $shape = ['key' => 1];
    } else {
        $shape = null;
    }
    $shape['key'] ?? ($copy = $shape); // observe
    $copy; // observe
}

function offsetDependencyWrites(?int $value): void
{
    $shape = ['left' => $value, 'right' => 7];
    $key = 'left';
    $shape[$key] ?? ($copy = $shape[$key]); // observe
    $copy; // observe
    $shape[$key] ?? [$key = 'right', $copy = $shape[$key]]; // observe
    $copy; // observe
}

function unrelatedWrite(?int $value): void
{
    $shape = ['key' => $value];
    $shape['key'] ?? [$other = 1, $copy = $shape['key']]; // observe
    $copy; // observe
}

function annotatedAssignment(?int $value): void
{
    /** @var positive-int $value */
    $value ??= 3; // observe
    $value; // observe
}

function unsupportedScalarWrite(int $value): void
{
    $value['key'] ??= 4;
    $value; // unavailable
}

function literalBoundary(): void
{
    $shape = [
        0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
        0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
        0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
        0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
        0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
        0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
        0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
        0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    ];
    $shape[255]; // observe
}

function unsupportedLargeLiteral(): void
{
    $shape = [
        0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
        0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
        0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
        0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
        0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
        0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
        0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
        0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
        0,
    ];
    $shape; // unavailable
    $shape = ['key' => 1];
    $shape['key']; // unavailable
}

/** @param array{required: int, optional?: string} $shape */
function dynamicParameter(array $shape, string $key): void
{
    $shape[$key]; // observe
    isset($shape[$key]); // observe
    $shape[$key] ?? 4; // observe
}

function unsupportedSkipped(int $value): void
{
    $value = 1;
    $value ?? ($value << 1);
    $value; // unavailable
}
