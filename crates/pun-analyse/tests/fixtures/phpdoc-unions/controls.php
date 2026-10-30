<?php

namespace PhpdocUnions;

use function PHPStan\dumpType;

/**
 * @param null|100|101 $left
 * @param 100|null|101 $middle
 * @param 100|101|null $right
 * @param null|(100|101) $nestedRight
 * @param (100|101)|null $nestedLeft
 * @param 100|(null|101) $insideRight
 * @param (null|100)|101 $insideLeft
 * @param null|null|100 $duplicates
 */
function nullOrder($left, $middle, $right, $nestedRight, $nestedLeft, $insideRight, $insideLeft, $duplicates): void
{
    dumpType($left);
    dumpType($middle);
    dumpType($right);
    dumpType($nestedRight);
    dumpType($nestedLeft);
    dumpType($insideRight);
    dumpType($insideLeft);
    dumpType($duplicates);
    dumpType(strlen($left));
    dumpType(strlen($middle));
    dumpType(strlen($right));
    dumpType(strlen($nestedRight));
    dumpType(strlen($nestedLeft));
    dumpType(strlen($insideRight));
    dumpType(strlen($insideLeft));
    dumpType(strlen($duplicates));
}

/**
 * @param null|string $string
 * @param int|null $int
 * @param null|float $float
 * @param object|null $object
 * @param null|scalar $scalar
 * @param null|null $null
 * @param null|never $bottom
 * @param never|(null|never) $nestedBottom
 * @param mixed|null|int $mixedLeft
 * @param int|null|mixed $mixedRight
 * @param (?int)|float|null $nullable
 */
function nullableKinds($string, $int, $float, $object, $scalar, $null, $bottom, $nestedBottom,
    $mixedLeft, $mixedRight, $nullable): void
{
    dumpType($string);
    dumpType($int);
    dumpType($float);
    dumpType($object);
    dumpType($scalar);
    dumpType($null);
    dumpType($bottom);
    dumpType($nestedBottom);
    dumpType($mixedLeft);
    dumpType($mixedRight);
    dumpType($nullable);
}

/**
 * @template T of 'a'|'ccc'
 * @param null|T $left
 * @param T|null $right
 * @param T|(null|T) $nested
 */
function templates($left, $right, $nested): void
{
    dumpType($left);
    dumpType($right);
    dumpType($nested);
    dumpType(strlen($left));
    dumpType(strlen($right));
    dumpType(strlen($nested));
}

/** @template T = 13 */
class Defaults
{
    /** @var T */
    public $value;
}

/**
 * @param null|Defaults $left
 * @param Defaults|null $right
 * @param Defaults|(null|Defaults) $nested
 */
function defaultMetadata($left, $right, $nested): void
{
    if ($left !== null) {
        dumpType($left->value);
    }
    if ($right !== null) {
        dumpType($right->value);
    }
    if ($nested !== null) {
        dumpType($nested->value);
    }
}
