<?php

namespace CoalesceScope;

function copiedOffset(array $values): void
{
    $copy = $values['optional'];
    echo $copy;
}

function optionalWrite(?string $value): void
{
    $value ?? ($created = 1);
    echo $created;

    $existing = 2;
    $value ?? ($existing = 3);
    \PHPStan\dumpType($existing);
}

function requiredWrite(): void
{
    $null = null;
    $null ?? ($required = 1);
    echo $required;

    $absent ?? ($alsoRequired = 1);
    echo $alsoRequired;
    echo $absent;
}

function skippedWrite(): void
{
    $set = 'set';
    $set ?? ($skipped = 1);
    echo $skipped;
}

function selfWrite(bool $flag): void
{
    if ($flag) {
        $value = 'set';
    }
    $value ?? ($value = 'fallback');
    echo $value;
    \PHPStan\dumpType($value);
}

function terminatingWrite(?string $value): void
{
    $existing = 2;
    $value ?? throw new \RuntimeException((string) ($existing = 3));
    \PHPStan\dumpType($existing);
    \PHPStan\dumpType($value);
}

function lhsSideEffect(array $values): void
{
    $index = 0;
    $values[$index++] ?? throw new \RuntimeException();
    \PHPStan\dumpType($index);
}

function sourceCertaintyIsNotCopied(bool $flag): void
{
    if ($flag) {
        $source = 1;
    }
    $copy = $source;
    echo $copy;
    echo $source;
}

function counterfactualAssignment(): void
{
    $defined = 10;
    $defined ??= $defined;
    \PHPStan\dumpType($defined);

    $plain = 11;
    $plain ?? $plain;
    $outer = 1;
    $outer += ($plain ?? $plain);
    \PHPStan\dumpType($plain);
}

function nullableAssignment(?int $nullable): void
{
    $nullable ?? \PHPStan\dumpType($nullable);
    $nullable ??= $nullable;
    \PHPStan\dumpType($nullable);
}

function optionalAssignment(bool $flag): void
{
    if ($flag) {
        $optional = 13;
    }
    $optional ??= $optional;
    \PHPStan\dumpType($optional);

    $absent ??= $absent;
    \PHPStan\dumpType($absent);
}

/** @param array<string, int|null> $values */
function nullableOffsetRhs(array $values, string $key): void
{
    $values[$key] ?? \PHPStan\dumpType($values[$key]);
    \PHPStan\dumpType($values[$key]);
}

/** @param array<string, int|string|null> $values */
function narrowedOffsetRhs(array $values, string $key): void
{
    if (is_string($values[$key])) {
        $values[$key] ?? \PHPStan\dumpType($values[$key]);
        \PHPStan\dumpType($values[$key]);
    }
}

/** @param array{first: int|null} $values */
function requiredNullableOffsetRhs(array $values): void
{
    $values['first'] ?? \PHPStan\dumpType($values['first']);
    \PHPStan\dumpType($values['first']);
    \PHPStan\dumpType($values);
}

/** @param array{first?: int} $values */
function optionalNonnullableOffsetRhs(array $values): void
{
    $values['first'] ?? \PHPStan\dumpType($values);
    $values['first'] ?? \PHPStan\dumpType($values['first']);
    \PHPStan\dumpType($values['first']);
    \PHPStan\dumpType($values);
}

/** @param array{first?: int|null} $values */
function optionalNullableOffsetRhs(array $values): void
{
    $values['first'] ?? \PHPStan\dumpType($values);
    $values['first'] ?? \PHPStan\dumpType($values['first']);
    \PHPStan\dumpType($values['first']);
    \PHPStan\dumpType($values);
}

/** @param array{first: int|null} $values */
function variableRequiredNullableOffsetRhs(array $values): void
{
    $key = 'first';
    $values[$key] ?? \PHPStan\dumpType($values[$key]);
    \PHPStan\dumpType($values[$key]);
}

/** @param array<string, int|null> $values */
function nullableOffsetRhsWrite(array $values, string $key): void
{
    $values[$key] ?? ($values[$key] = 7);
    \PHPStan\dumpType($values[$key]);
}

/** @param array{first: int|null} $values */
function requiredNullableOffsetRhsWrite(array $values): void
{
    $values['first'] ?? ($values['first'] = 'written');
    \PHPStan\dumpType($values['first']);
    \PHPStan\dumpType($values);
}

function actualNullOffsetRhsWrite(): void
{
    $values = ['first' => null];
    $values['first'] ?? ($values['first'] = 'written');
    \PHPStan\dumpType($values['first']);
    \PHPStan\dumpType($values);
}

/** @param array<string, int|string|null> $values */
function narrowedOffsetRhsNullWrite(array $values, string $key): void
{
    if (is_string($values[$key])) {
        $values[$key] ?? ($values[$key] = null);
        \PHPStan\dumpType($values[$key]);
    }
}

/** @param array<string, int|string|null> $values */
function narrowedOffsetRhsIntegerWrite(array $values, string $key): void
{
    if (is_string($values[$key])) {
        $values[$key] ?? ($values[$key] = 9);
        \PHPStan\dumpType($values[$key]);
    }
}

/** @param array<string, int|string|null> $values */
function narrowedOffsetRhsReceiverWrite(array $values, string $key): void
{
    if (is_string($values[$key])) {
        $values[$key] ?? ($values = ['other' => 2]);
        \PHPStan\dumpType($values[$key]);
    }
}

/** @param array<string, int|string|null> $values */
function narrowedOffsetRhsIndexWrite(array $values, string $key): void
{
    if (is_string($values[$key])) {
        $values[$key] ?? ($key = 'other');
        \PHPStan\dumpType($values[$key]);
    }
}

/** @param array{first?: int|null} $values */
function optionalOffsetRhsWrite(array $values): void
{
    $values['first'] ?? ($values['first'] = 7);
    \PHPStan\dumpType($values['first']);
    \PHPStan\dumpType($values);
}

/** @param array{first: int, second: null} $values */
function postfixOffsetCoalesce(array $values): void
{
    $keys = ['first', 'second'];
    $index = 0;
    $values[$keys[$index++]] ?? \PHPStan\dumpType($index);
    \PHPStan\dumpType($index);
    \PHPStan\dumpType($values['second']);
}

/** @param array{first: int} $values */
function nonnullableOffsetCounterfactualRhs(array $values): void
{
    $values['first'] ?? \PHPStan\dumpType($values['first']);
    \PHPStan\dumpType($values['first']);
}

/** @param array{0: int, 1?: int} $values */
function optionalListOffsetRhs(array $values): void
{
    $values[1] ?? \PHPStan\dumpType($values);
    \PHPStan\dumpType($values);
}

/** @param array{0: int, 1?: int|null} $values */
function nullableListOffsetRhs(array $values): void
{
    $values[1] ?? \PHPStan\dumpType($values);
    \PHPStan\dumpType($values);
}

/** @param array{kind: 'present', first: int}|array{kind: 'absent'} $values */
function constantOffsetAlternativesRhs(array $values): void
{
    $values['first'] ?? \PHPStan\dumpType($values);
    \PHPStan\dumpType($values);
}

/** @param array{kind: 'nonnull', first?: int}|array{kind: 'nullable', first?: null} $values */
function nullableOffsetAlternativesRhs(array $values): void
{
    $values['first'] ?? \PHPStan\dumpType($values);
    \PHPStan\dumpType($values);
}

/** @param array{nested?: array{first: int|null}} $values */
function nestedOffsetRhs(array $values): void
{
    $values['nested']['first'] ?? \PHPStan\dumpType($values);
    \PHPStan\dumpType($values);
}

/** @param array<int, int|null> $values */
function generalPostfixOffsetRhs(array $values, int $index): void
{
    $values[$index++] ?? \PHPStan\dumpType($index);
    \PHPStan\dumpType($index);
    \PHPStan\dumpType($values);
}

function nativeOptionalListOffsetRhs(bool $flag): void
{
    $values = $flag ? [1, 2] : [1];
    $values[1] ?? \PHPStan\dumpType($values);
    \PHPStan\dumpType($values);
}

function nativePossiblyEmptyListOffsetRhs(bool $flag): void
{
    $values = $flag ? [1] : [];
    $values[0] ?? \PHPStan\dumpType($values);
    \PHPStan\dumpType($values);
}
