<?php
interface A {}
interface B {}
interface C { public function ready(): bool; }
function merged($query, bool $flag): void {
    if ($query instanceof A) {
        return;
    }
    \PHPStan\dumpType($query);
    $selected = $query instanceof C && $flag;
    \PHPStan\dumpType($query);
}
function narrowing($query, bool $flag): void {
    if ($query instanceof A) {
        return;
    }
    if ($query instanceof C) {
        \PHPStan\dumpType($query);
        \PHPStan\dumpType($query->ready());
        if ($query instanceof A) {
            \PHPStan\dumpType($query);
        } else {
            \PHPStan\dumpType($query);
        }
        $copy = $query;
        $value = $flag ? $copy : $query;
        \PHPStan\dumpType($value);
    }
}
function scalar($query): void {
    if (is_int($query) || (is_string($query) && $query !== '')) {
        \PHPStan\dumpType($query);
    } else {
        \PHPStan\dumpType($query);
    }
}
