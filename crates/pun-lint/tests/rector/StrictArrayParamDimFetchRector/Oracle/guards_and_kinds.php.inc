<?php

namespace Probe;

final class Bag implements \ArrayAccess
{
    public function offsetExists(mixed $offset): bool
    {
        return true;
    }

    public function offsetGet(mixed $offset): mixed
    {
        return null;
    }

    public function offsetSet(mixed $offset, mixed $value): void
    {
    }

    public function offsetUnset(mixed $offset): void
    {
    }

    public static function wrap($value)
    {
        return $value;
    }

    public function same($value)
    {
        return $value;
    }
}

abstract class Base
{
    abstract public function read($row);
}

function maybe(): ?array
{
    return null;
}

function wrap($value)
{
    return $value;
}

function reassigned($rows)
{
    $rows = maybe();

    return $rows['a'] ?? null;
}

function staticOnOffset($rows)
{
    echo $rows['class']::$name;
}

function namedArg($rows)
{
    $rows = wrap(value: $rows);

    return $rows['a'];
}

function functionArg($rows)
{
    $rows = wrap($rows);

    return $rows['a'];
}

function nullsafeArg($rows, ?Bag $bag)
{
    $rows = $bag?->same($rows);

    return $rows['a'];
}

function staticArg($rows)
{
    $rows = Bag::wrap($rows);

    return $rows['a'];
}

function newArg($rows)
{
    $rows = new \ArrayObject($rows);

    return $rows['a'];
}

/**
 * @param  string|array  $value
 */
function stringOrArray($value)
{
    return $value[0];
}

/**
 * @param  array{a: bool, b: array}  $shape
 */
function boolAndArray($shape)
{
    return $shape['a'];
}

/**
 * @param  array{a: Bag, b: int}  $shape
 */
function objectAndInt($shape)
{
    return $shape['a'];
}

/**
 * @param  Bag  $bag
 */
function arrayAccess($bag)
{
    return $bag['a'];
}
-----
<?php

namespace Probe;

final class Bag implements \ArrayAccess
{
    public function offsetExists(mixed $offset): bool
    {
        return true;
    }

    public function offsetGet(mixed $offset): mixed
    {
        return null;
    }

    public function offsetSet(mixed $offset, mixed $value): void
    {
    }

    public function offsetUnset(mixed $offset): void
    {
    }

    public static function wrap($value)
    {
        return $value;
    }

    public function same($value)
    {
        return $value;
    }
}

abstract class Base
{
    abstract public function read($row);
}

function maybe(): ?array
{
    return null;
}

function wrap($value)
{
    return $value;
}

function reassigned(array $rows)
{
    $rows = maybe();

    return $rows['a'] ?? null;
}

function staticOnOffset($rows)
{
    echo $rows['class']::$name;
}

function namedArg($rows)
{
    $rows = wrap(value: $rows);

    return $rows['a'];
}

function functionArg($rows)
{
    $rows = wrap($rows);

    return $rows['a'];
}

function nullsafeArg($rows, ?Bag $bag)
{
    $rows = $bag?->same($rows);

    return $rows['a'];
}

function staticArg($rows)
{
    $rows = Bag::wrap($rows);

    return $rows['a'];
}

function newArg($rows)
{
    $rows = new \ArrayObject($rows);

    return $rows['a'];
}

/**
 * @param  string|array  $value
 */
function stringOrArray($value)
{
    return $value[0];
}

/**
 * @param  array{a: bool, b: array}  $shape
 */
function boolAndArray($shape)
{
    return $shape['a'];
}

/**
 * @param  array{a: Bag, b: int}  $shape
 */
function objectAndInt($shape)
{
    return $shape['a'];
}

/**
 * @param  Bag  $bag
 */
function arrayAccess($bag)
{
    return $bag['a'];
}
