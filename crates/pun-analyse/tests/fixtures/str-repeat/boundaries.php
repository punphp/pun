<?php
namespace RepeatBoundaries;
use function PHPStan\dumpType;

function boundaries(bool $flag): void
{
    dumpType(str_repeat('a', 99));
    dumpType(str_repeat('a', 100));
    dumpType(str_repeat('a', 101));
    dumpType(str_repeat($flag ? 'a' : 'b', 49));
    dumpType(str_repeat($flag ? 'a' : 'b', 50));
    dumpType(str_repeat($flag ? 'ab' : 'c', 33));
    dumpType(str_repeat($flag ? 'ab' : 'c', 34));
    dumpType(str_repeat('é', 49));
    dumpType(str_repeat('é', 50));
    dumpType(str_repeat('€', 33));
    dumpType(str_repeat('€', 34));
    dumpType(str_repeat($flag ? '' : 'x', 99));
    dumpType(str_repeat($flag ? '' : 'x', 100));
    dumpType(str_repeat('a', PHP_INT_MAX));
    dumpType(str_repeat('ab', PHP_INT_MAX));
    dumpType(str_repeat('0', 100));
    dumpType(str_repeat('1', 100));
    dumpType(str_repeat('1e3', 100));
    dumpType(str_repeat('1E3', 100));
    dumpType(str_repeat('+1', 100));
    dumpType(str_repeat(' 1 ', 100));
    dumpType(str_repeat('1.5', 100));
    dumpType(str_repeat('19', 100));
    dumpType(str_repeat("1\n", 100));
    dumpType(str_repeat('', 9223372036854775807));
    dumpType(str_repeat('ab', 9223372036854775807));
    dumpType(987);
}
