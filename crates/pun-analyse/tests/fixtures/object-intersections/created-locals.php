<?php
function introduced(bool $flag): void {
    $flag || ($right = 1);
    \PHPStan\dumpType($right);
    $flag && ($other = 2);
    \PHPStan\dumpType($other);
}

/** @phpstan-assert iterable<int, string> $value */
function ensureStrings($value): void {}

interface Excluded {}

/** @param ArrayIterator<int, mixed> $values */
function traversable(ArrayIterator $values): void {
    if ($values instanceof Excluded) {
        return;
    }
    ensureStrings($values);
    \PHPStan\dumpType($values instanceof Excluded);
    foreach ($values as $value) {
        \PHPStan\dumpType($value);
    }
}
