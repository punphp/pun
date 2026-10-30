<?php

namespace StringCasing;

use function PHPStan\dumpType;

/** @param lowercase-string&uppercase-string $both
 * @param lowercase-string $lower
 * @param uppercase-string $upper
 * @param non-empty-string&numeric-string $numeric
 * @param literal-string&lowercase-string&non-empty-string&numeric-string&uppercase-string $literalBoth
 */
function matrix(string $both, string $lower, string $upper, string $numeric, bool $flag, string $literalBoth): void
{
    dumpType($both);
    dumpType($flag ? $both : $lower);
    dumpType($flag ? $both : $upper);
    dumpType($both . $both);
    dumpType($both . $lower);
    dumpType($both . $upper);
    dumpType($lower . $both);
    dumpType($upper . $both);
    dumpType($both . '!0');
    dumpType('!0' . $both);
    dumpType($flag ? $both : '!0');
    dumpType($flag ? '!0' : $both);
    dumpType($flag ? $lower : '!0');
    dumpType($flag ? '!0' : $upper);
    dumpType($numeric);
    takeLower($both);
    takeUpper($both);
    takeBoth($lower);
    takeBoth($upper);
    takeBoth($numeric);
    takeBoth('!0');
    takeBoth('e');
    takeBoth('E');
    takeBoth('é');
    $copy = $lower;
    assertUpper($copy);
    dumpType($copy);
    dumpType($lower);
    $copy = $upper;
    assertLower($copy);
    dumpType($copy);
    dumpType($upper);
    dumpType($literalBoth);
    dumpType($literalBoth . '!0');
    dumpType($flag ? $literalBoth : $lower);
}

/** @param lowercase-string $value */
function takeLower(string $value): void {}
/** @param uppercase-string $value */
function takeUpper(string $value): void {}
/** @param lowercase-string&uppercase-string $value */
function takeBoth(string $value): void {}
/** @phpstan-assert uppercase-string $value */
function assertUpper(string $value): void {}
/** @phpstan-assert lowercase-string $value */
function assertLower(string $value): void {}

/** @param lowercase-string&uppercase-string $value */
function callableProbe(string $value): void
{
    if (is_callable($value)) {
        dumpType($value);
    }
}
