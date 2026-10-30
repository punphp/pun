<?php
namespace StringUnions;
use function PHPStan\dumpType;

/**
 * @param lowercase-string $lower
 * @param uppercase-string $upper
 * @param lowercase-string&uppercase-string $both
 * @param lowercase-string&non-empty-string $lowerNonempty
 * @param uppercase-string&non-empty-string $upperNonempty
 * @param lowercase-string|uppercase-string $declaredEither
 * @param numeric-string $numeric
 * @param non-empty-string $nonempty
 * @param non-falsy-string $truthy
 * @param literal-string $literal
 */
function unions(bool $flag, bool $secondFlag, string $lower, string $upper, string $both, string $lowerNonempty, string $upperNonempty, string $declaredEither, string $numeric, string $nonempty, string $truthy, string $literal, string $plain): string
{
    dumpType($both);
    dumpType($declaredEither);
    dumpType($flag ? $lower : $upper);
    dumpType($flag ? $upper : $lower);
    dumpType($flag ? $lowerNonempty : $upperNonempty);
    dumpType($flag ? $upperNonempty : $lowerNonempty);
    $either = $flag ? $lower : $upper;
    dumpType($either);
    if ($flag) {
        $merged = $lower;
    } else {
        $merged = $upper;
    }
    dumpType($merged);
    if ($flag) {
        $reversed = $upper;
    } else {
        $reversed = $lower;
    }
    dumpType($reversed);

    dumpType($flag ? $lower : $numeric);
    dumpType($flag ? $numeric : $lower);
    dumpType($flag ? $upper : $numeric);
    dumpType($flag ? $numeric : $upper);
    dumpType($flag ? $lower : $nonempty);
    dumpType($flag ? $nonempty : $lower);
    dumpType($flag ? $upper : $nonempty);
    dumpType($flag ? $nonempty : $upper);
    dumpType($flag ? $lower : $truthy);
    dumpType($flag ? $truthy : $lower);
    dumpType($flag ? $upper : $truthy);
    dumpType($flag ? $truthy : $upper);
    dumpType($flag ? $lower : $literal);
    dumpType($flag ? $literal : $lower);
    dumpType($flag ? $upper : $literal);
    dumpType($flag ? $literal : $upper);
    dumpType($flag ? $numeric : $literal);
    dumpType($flag ? $literal : $numeric);

    dumpType($flag ? $both : $lower);
    dumpType($flag ? $lower : $both);
    dumpType($flag ? $both : $upper);
    dumpType($flag ? $upper : $both);
    dumpType($flag ? $plain : $lower);
    dumpType($flag ? $lower : $plain);
    dumpType($flag ? $plain : $upper);
    dumpType($flag ? $upper : $plain);
    dumpType($flag ? '!0' : $lower);
    dumpType($flag ? $lower : '!0');
    dumpType($flag ? '!0' : $upper);
    dumpType($flag ? $upper : '!0');
    dumpType($flag ? 'e' : 'E');
    dumpType($flag ? $nonempty : $truthy);
    dumpType($flag ? $truthy : $nonempty);
    dumpType($flag ? $nonempty : '');
    dumpType($flag ? '' : $nonempty);
    dumpType($flag ? $truthy : '');
    dumpType($flag ? '' : $truthy);
    dumpType($flag ? $truthy : '0');
    dumpType($flag ? '0' : $truthy);
    dumpType($flag ? $lowerNonempty : '');
    dumpType($flag ? '' : $lowerNonempty);
    dumpType($flag ? ($secondFlag ? $lower : $upper) : $plain);
    dumpType($flag ? $plain : ($secondFlag ? $lower : $upper));
    return $plain;
}
