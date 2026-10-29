<?php declare(strict_types=1);

namespace ParameterCaptures;

class Left {}
class Right {}

/**
 * @param 'foo'|'bar' $method
 * @param ($method is 'foo' ? Left : Right) $value
 */
function captures(string $method, Left|Right $value): void
{
    $complete = function () use ($value, $method): void {
        if ($method === 'foo') {
            \PHPStan\dumpType($value);
        } else {
            \PHPStan\dumpType($value);
        }
    };
    $incomplete = function (string $method) use ($value): void {
        if ($method === 'foo') {
            \PHPStan\dumpType($value);
        }
    };
    $byReference = function () use (&$value, $method): void {
        if ($method === 'foo') {
            \PHPStan\dumpType($value);
        }
    };
    $guardReference = function () use ($value, &$method): void {
        if ($method === 'foo') {
            \PHPStan\dumpType($value);
        }
    };
    $arrow = fn () => $method === 'foo'
        ? \PHPStan\dumpType($value)
        : \PHPStan\dumpType($value);
    $staticArrow = static fn () => $method === 'foo'
        ? \PHPStan\dumpType($value)
        : \PHPStan\dumpType($value);
    $shadowGuard = fn (string $method) => $method === 'foo'
        ? \PHPStan\dumpType($value)
        : \PHPStan\dumpType($value);
    $shadowTarget = fn (Right $value) => $method === 'foo'
        ? \PHPStan\dumpType($value)
        : \PHPStan\dumpType($value);
    $nested = fn () => function () use ($method, $value): void {
        if ($method === 'bar') {
            \PHPStan\dumpType($value);
        }
    };
    \PHPStan\dumpType($value);
}

/**
 * @param 'foo'|'bar' $method
 * @param ($method is 'foo' ? Left : Right) $value
 */
function captureWrites(string $method, Left|Right $value): void
{
    $guardWrite = function () use ($method, $value): void {
        $method = 'foo';
        \PHPStan\dumpType($value);
    };
    $targetWrite = function () use ($method, $value): void {
        $value = new Right();
        if ($method === 'foo') {
            \PHPStan\dumpType($value);
        }
    };
    $uncaptured = function () use ($method): void {
        $value = new Right();
        if ($method === 'foo') {
            \PHPStan\dumpType($value);
        }
    };
}
