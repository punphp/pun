<?php
namespace RepeatAccessories;
use function PHPStan\dumpType;

/**
 * @param non-empty-string $nonempty
 * @param non-falsy-string $truthy
 * @param numeric-string $numeric
 * @param literal-string $literal
 * @param lowercase-string $lower
 * @param uppercase-string $upper
 * @param lowercase-string&uppercase-string $both
 * @param literal-string&numeric-string $literalNumeric
 * @param positive-int $positive
 * @param non-negative-int $nonnegative
 */
function accessories(string $string, string $nonempty, string $truthy, string $numeric, string $literal, string $lower, string $upper, string $both, string $literalNumeric, int $positive, int $nonnegative, int $integer): void
{
    dumpType(str_repeat($string, 1));
    dumpType(str_repeat($string, 0));
    dumpType(str_repeat($nonempty, $positive));
    dumpType(str_repeat($nonempty, $nonnegative));
    dumpType(str_repeat($truthy, $positive));
    dumpType(str_repeat($truthy, $integer));
    dumpType(str_repeat($numeric, $positive));
    dumpType(str_repeat($numeric, $nonnegative));
    dumpType(str_repeat($literal, $positive));
    dumpType(str_repeat($literal, $integer));
    dumpType(str_repeat($lower, $positive));
    dumpType(str_repeat($upper, $positive));
    dumpType(str_repeat($both, $positive));
    dumpType(str_repeat($literalNumeric, $positive));
    dumpType(str_repeat($literalNumeric, $nonnegative));
    dumpType(str_repeat('19', $positive));
    dumpType(str_repeat('19', $nonnegative));
    dumpType(str_repeat('0', $positive));
    dumpType(str_repeat('', $positive));
    dumpType(str_repeat('1e3', $positive));
    dumpType(str_repeat('1E3', $positive));
    dumpType(str_repeat('+1', $positive));
    dumpType(str_repeat(' 1 ', $positive));
    dumpType(str_repeat('1.5', $positive));
    dumpType(987);
}
