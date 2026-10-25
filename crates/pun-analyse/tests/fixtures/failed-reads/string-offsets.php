<?php declare(strict_types=1);

namespace FailedStringOffsets;

/** @param non-empty-string $value */
function indexes(string $value, int $valid, string $invalid, int|string $uncertain, mixed $unknown): void
{
    \PHPStan\dumpType($value[0]);
    \PHPStan\dumpType($value[-1]);
    \PHPStan\dumpType($value[$valid]);
    \PHPStan\dumpType($value['foo']);
    \PHPStan\dumpType($value['1']);
    \PHPStan\dumpType($value[1.5]);
    \PHPStan\dumpType($value[false]);
    \PHPStan\dumpType($value[null]);
    \PHPStan\dumpType($value[$invalid]);
    \PHPStan\dumpType($value[$uncertain]);
    \PHPStan\dumpType($value[$unknown]);
    $copy = $value['missing'];
    \PHPStan\dumpType($copy);
}

function literalParent(): void
{
    $value = 'Foo';
    \PHPStan\dumpType($value['foo']);
    $value['foo'] += null;
}

/**
 * @template T of int|string
 * @param non-empty-string $value
 * @param T $index
 */
function uncertainTemplate(string $value, int|string $index): void
{
    \PHPStan\dumpType($value[$index]);
}

/**
 * @template T of string
 * @param non-empty-string $value
 * @param T $index
 */
function invalidTemplate(string $value, string $index): void
{
    \PHPStan\dumpType($value[$index]);
}

/** @param non-empty-string $value */
function indexOnce(string $value): void
{
    $index = 0.5;
    $result = $value[$index++];
    \PHPStan\dumpType($result);
    \PHPStan\dumpType($index);
    \PHPStan\dumpType($value);
}

/** @param non-empty-string $value */
function coalesceControl(string $value): void
{
    \PHPStan\dumpType($value['missing'] ?? 'fallback');
    \PHPStan\dumpType($value[0] ?? 'fallback');
}

/**
 * @param non-empty-string $value
 * @param numeric-string $numeric
 */
function otherScalarIndexes(string $value, float $float, bool $bool, string $numeric): void
{
    \PHPStan\dumpType($value[$float]);
    \PHPStan\dumpType($value[$bool]);
    \PHPStan\dumpType($value[$numeric]);
    \PHPStan\dumpType($value[1.0]);
}

/**
 * @param non-empty-string $value
 * @param scalar $scalar
 * @param numeric $numeric
 * @param array-key $key
 */
function integerCapableIndexes(string $value, mixed $scalar, int|float|string $numeric, int|string $key, int|float $number): void
{
    \PHPStan\dumpType($value[$scalar]);
    \PHPStan\dumpType($value[$numeric]);
    \PHPStan\dumpType($value[$key]);
    \PHPStan\dumpType($value[$number]);
}

/**
 * @template T of float
 * @param non-empty-string $value
 * @param T $index
 */
function invalidFloatTemplate(string $value, float $index): void
{
    \PHPStan\dumpType($value[$index]);
}
