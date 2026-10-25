<?php

namespace DebugTemplates;

use function PHPStan\dumpType as dtype;
use function PHPStan\dumpPhpDocType as ptype;

class Bound {}

/**
 * @template T of Bound
 * @param T $value
 * @param class-string<T> $class
 * @param array{a: T, z: class-string<T>} $shape
 * @return T
 */
function typed($value, string $class, array $shape)
{
    dtype($value);
    dtype($class);
    ptype($class);
    dtype([$value, $class]);
    dtype($shape);
    dtype(typed($value, $class, $shape));
    return $value;
}

/** @template T of Bound */
class GenericBox
{
    /** @param T $value */
    public function inspect($value): void
    {
        dtype($value);
        dtype([$value]);
    }

    /** @return T */
    public function get()
    {
        throw new \RuntimeException();
    }

    public function inspectReturn(): void
    {
        dtype($this->get());
    }
}

/**
 * @template T of Bound
 * @template U of T
 * @param U $value
 */
function nestedBound($value): void
{
    dtype($value);
    ptype($value);
}
