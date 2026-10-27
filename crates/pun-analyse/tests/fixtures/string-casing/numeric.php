<?php
namespace NumericAccessories;
use function PHPStan\dumpType;

/**
 * @param numeric-string $numeric
 * @param non-empty-string&numeric-string $explicit
 * @param lowercase-string $lower
 * @param uppercase-string $upper
 * @param literal-string $literal
 * @param non-empty-string $nonempty
 */
function numeric(bool $flag, string $numeric, string $explicit, string $lower, string $upper, string $literal, string $nonempty, string $plain): string
{
    dumpType($numeric);
    dumpType($explicit);
    $bareCopy = $numeric;
    $explicitCopy = $explicit;
    dumpType($bareCopy);
    dumpType($explicitCopy);
    dumpType($flag ? $numeric : $upper);
    dumpType($flag ? $explicit : $upper);
    dumpType($flag ? $lower : $numeric);
    dumpType($flag ? $lower : $explicit);
    dumpType($flag ? $numeric : $literal);
    dumpType($flag ? $explicit : $literal);
    if ($flag) {
        $bareBranch = $numeric;
        $explicitBranch = $explicit;
    } else {
        $bareBranch = $upper;
        $explicitBranch = $upper;
    }
    dumpType($bareBranch);
    dumpType($explicitBranch);
    if ($flag) {
        $bareReversed = $upper;
        $explicitReversed = $upper;
    } else {
        $bareReversed = $numeric;
        $explicitReversed = $explicit;
    }
    dumpType($bareReversed);
    dumpType($explicitReversed);
    echo needNonempty($numeric);
    echo needNonempty($explicit);
    echo needNonempty('');
    if (is_numeric($plain)) {
        dumpType($plain);
        echo needNonempty($plain);
    }
    if (is_numeric($nonempty)) {
        dumpType($nonempty);
        echo needNonempty($nonempty);
    }
    return $numeric;
}

/** @param non-empty-string $value */
function needNonempty(string $value): string
{
    return $value;
}
