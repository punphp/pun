<?php
namespace RepeatFinite;
use function PHPStan\dumpType;

function values(bool $flag): void
{
    dumpType(str_repeat('ab', 3));
    dumpType(str_repeat($flag ? 'ab' : 'c', 3));
    dumpType(str_repeat('', PHP_INT_MAX));
    dumpType(str_repeat('anything', 0));
    $result = str_repeat('xy', 2);
    dumpType($result);
    dumpType(strlen($result));
    dumpType(str_repeat($result, 2));
    dumpType(str_repeat('0', 2));
    dumpType(str_repeat('1', 2));
    dumpType(str_repeat('a', $flag ? 1 : 2));
    dumpType(str_repeat('a', $flag ? 0 : 1));
    dumpType(str_repeat(string: 'xy', times: 2));
    dumpType(str_repeat(times: 2, string: 'xy'));
    dumpType(987); // Later observable expression: inference must not abort the body.
}

function negative(): void
{
    dumpType(str_repeat('ab', -1));
}

/** @param int<min, 0> $count */
function nonpositive(int $count): void
{
    dumpType(str_repeat('ab', $count));
}

/** @param -1|0 $count */
function nonpositiveUnion(int $count): void
{
    dumpType(str_repeat('ab', $count));
}

function missing(): void
{
    dumpType(str_repeat('a'));
    dumpType(str_repeat());
}
