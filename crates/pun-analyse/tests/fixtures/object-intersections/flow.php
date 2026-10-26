<?php
interface A {}
interface B {}
interface C { public function ready(): bool; }
function observe($query): void {
    if ($query instanceof A
        || $query instanceof B
        || ($query instanceof C && $query->ready())) {
        \PHPStan\dumpType($query);
    } else {
        \PHPStan\dumpType($query);
    }
}
function builderShape($query) {
    return $query instanceof A
        || $query instanceof B
        || ($query instanceof C && $query->ready()) ? 1 : 0;
}
interface D {}
function twoPairs($query): void {
    if (($query instanceof A && $query instanceof B)
        || ($query instanceof C && $query instanceof D)) {
        \PHPStan\dumpType($query);
    } else {
        \PHPStan\dumpType($query);
    }
}
