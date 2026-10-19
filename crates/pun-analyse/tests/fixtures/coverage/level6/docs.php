<?php

namespace CoverageDocs;

class Holder
{
    const A = 1;
}

/**
 * @param array<int> ...$rest
 * @param int $1bad
 * @param int $
 */
function variadic(...$rest): void
{
}

/**
 * @psalm-param Holder::NOPE $a
 * @psalm-param-out Holder::NOPE $a
 * @param array<int, int> $a
 * @psalm-return Holder::NOPE
 * @return array<int, int>
 */
function psalm($a)
{
    return $a;
}

/**
 * @return PHP_INT_MAX
 */
function constant()
{
    return PHP_INT_MAX;
}

/**
 * @method void quoted(string $s = 'a\'b,c')
 * @method array {}
 * @method void nested(callable(int): void $c, array $d)
 * @method array dup()
 * @method int dup()
 * @property int $
 * @property int noDollar
 * @phpstan-import-type Y from <Z>
 * @phpstan-import-type X from
 * @template
 */
class Tags
{
    /**
     * @psalm-var Holder::NOPE
     * @var array<int, int>
     */
    public $psalmVar = [];

    /**
     * @var array $named
     * @var array<int, int> $named
     */
    public $named = [];

    /**@var array */
    public $glued = [];
}
