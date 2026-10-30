<?php
namespace BoundaryConcat;
use function PHPStan\dumpType;

/**
 * @param non-empty-string $a
 * @param non-empty-string $b
 * @param non-falsy-string $truthy
 * @param lowercase-string&uppercase-string&non-empty-string $both
 * @param lowercase-string&non-empty-string $lower
 * @param uppercase-string&non-empty-string $upper
 * @param literal-string&non-empty-string $literal
 * @param (lowercase-string&non-empty-string)|uppercase-string $partlyNonempty
 */
function concat(string $a, string $b, string $plain, string $truthy, string $both, string $lower, string $upper, string $literal, string $partlyNonempty): void
{
    dumpType($a);
    dumpType($a . $b);
    dumpType($b . $a);
    dumpType($a . $a);
    dumpType($a . $plain);
    dumpType($plain . $a);
    dumpType($plain . $plain);
    dumpType($truthy . $plain);
    dumpType($plain . $truthy);
    dumpType($a . '0');
    dumpType('0' . $a);
    dumpType($a . '');
    dumpType('' . $a);
    dumpType($a . '!');
    dumpType('!' . $a);
    dumpType($plain . '0');
    dumpType('0' . $plain);
    dumpType($both . $both);
    dumpType($both . $lower);
    dumpType($upper . $both);
    dumpType($lower . $upper);
    dumpType($literal . $literal);
    dumpType(str_repeat($a, 2));
    dumpType(str_repeat($a, 0));
    dumpType($a . $plain . $b);
    dumpType('0' . '0');
    dumpType('' . '0');
    dumpType($partlyNonempty . $a);
    dumpType($a . $partlyNonempty);
}
