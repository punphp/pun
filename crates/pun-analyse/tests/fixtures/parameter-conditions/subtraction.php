<?php declare(strict_types=1);

namespace ParameterSubtraction;

/**
 * @param 1|2|3 $number
 * @param ($number is 1|2 ? int : string) $value
 */
function literals(int $number, int|string $value): void
{
    if ($number === 3) {
        \PHPStan\dumpType($value);
    } else {
        \PHPStan\dumpType($value);
    }
}

/**
 * @param int<0, 10> $number
 * @param ($number is int<0, 4> ? int : string) $value
 */
function ranges(int $number, int|string $value): void
{
    if ($number >= 5) {
        \PHPStan\dumpType($value);
    } else {
        \PHPStan\dumpType($value);
    }
}

/** @param ($flag is true ? int : string) $value */
function booleans(bool $flag, int|string $value): void
{
    if ($flag) {
        \PHPStan\dumpType($value);
    } else {
        \PHPStan\dumpType($value);
    }
}

/** @param ($input is int ? int : string) $value */
function mixedInput(mixed $input, int|string $value): void
{
    if (is_int($input)) {
        \PHPStan\dumpType($value);
    } else {
        \PHPStan\dumpType($value);
    }
    if ($input === 1) {
        \PHPStan\dumpType($value);
    }
}

/** @param ($input is int ? int : string) $value */
function untypedInput($input, int|string $value): void
{
    if ($input === 1) {
        \PHPStan\dumpType($value);
    }
}

/**
 * @param false $flag
 * @param ($flag is true ? int : string) $value
 */
function impossible(bool $flag, int|string $value): void
{
    if ($flag) {
        \PHPStan\dumpType($value);
    } else {
        \PHPStan\dumpType($value);
    }
}
