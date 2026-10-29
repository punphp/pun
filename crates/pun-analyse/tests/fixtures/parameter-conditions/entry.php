<?php declare(strict_types=1);

namespace ParameterConditions;

class Left { public function foo(): int { return 1; } }
class Right { public function bar(): int { return 2; } }

/**
 * @param ($method is 'foo' ? Left : Right) $value
 * @param 'foo'|'bar' $method
 */
function forward(Left|Right $value, string $method): void
{
    \PHPStan\dumpType($value);
    if ($method === 'foo') {
        \PHPStan\dumpType($value);
        $value->foo();
    } else {
        \PHPStan\dumpType($value);
        $value->bar();
    }
    \PHPStan\dumpType($value);
}

/**
 * @param ($method is not 'foo' ? Right : Left) $value
 * @param 'foo'|'bar' $method
 */
function negated(string $method, Left|Right $value): void
{
    if ($method === 'foo') {
        \PHPStan\dumpType($value);
    } else {
        \PHPStan\dumpType($value);
    }
}

/**
 * @param 'foo'|'bar' $method
 * @param ($method is 'foo' ? Left : Right) $value
 */
function writes(string $method, Left|Right $value, bool $unrelated): void
{
    $unrelated = false;
    if ($method === 'foo') {
        \PHPStan\dumpType($value);
    }
    $method = 'foo';
    \PHPStan\dumpType($value);
    if ($method === 'foo') {
        \PHPStan\dumpType($value);
    }
}

/**
 * @param 'foo'|'bar' $method
 * @param ($method is 'foo' ? Left : Right) $value
 */
function targetWrite(string $method, Left|Right $value, Right $replacement): void
{
    $value = $replacement;
    if ($method === 'foo') {
        \PHPStan\dumpType($value);
    }
}

/** @param ($missing is true ? Left : Right) $value */
function missing(Left|Right $value): void
{
    \PHPStan\dumpType($value);
}

/**
 * @param 'foo'|'bar' $method
 * @param array{item: ($method is 'foo' ? Left : Right)} $values
 */
function nested(string $method, array $values): void
{
    if ($method === 'foo') {
        \PHPStan\dumpType($values['item']);
    }
}

/**
 * @template T
 * @param T $input
 * @param (T is int ? int : string) $value
 */
function templateSubject(mixed $input, int|string $value): void
{
    if (is_int($input)) {
        \PHPStan\dumpType($value);
    }
}

/**
 * @param 'foo'|'bar' $method
 * @param (($method is 'foo' ? Left : Right)|null) $value
 */
function unionSubject(string $method, Left|Right|null $value): void
{
    if ($method === 'foo') {
        \PHPStan\dumpType($value);
    }
}

class Methods
{
    /**
     * @param 'foo'|'bar' $method
     * @param ($method is 'foo' ? Left : Right) $value
     */
    public static function pick(string $method, Left|Right $value): void
    {
        if ($method === 'bar') {
            \PHPStan\dumpType($value);
        }
    }

    /**
     * @param 'foo'|'bar' $method
     * @param ($method is 'foo' ? self : Right) $value
     */
    final public function relative(string $method, self|Right $value): void
    {
        if ($method === 'foo') {
            \PHPStan\dumpType($value);
        }
    }
}
