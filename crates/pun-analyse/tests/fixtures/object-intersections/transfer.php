<?php
interface A {}
interface B {}
interface C { public function ready(): bool; }
interface D {}
function simple($query): void {
    if ($query instanceof A || $query instanceof B) {
        \PHPStan\dumpType($query);
    } else {
        \PHPStan\dumpType($query);
    }
}
function nested($query): void {
    if ($query instanceof A || ($query instanceof B && $query instanceof C)) {
        \PHPStan\dumpType($query);
    } else {
        \PHPStan\dumpType($query);
    }
}
function reversed($query): void {
    if (($query instanceof C && $query instanceof D) || ($query instanceof A && $query instanceof B)) {
        \PHPStan\dumpType($query);
    } else {
        \PHPStan\dumpType($query);
    }
}
function returning($query): void {
    if ($query instanceof A || $query instanceof B || ($query instanceof C && $query->ready())) {
        return;
    }
    \PHPStan\dumpType($query);
}
function expression($query): void {
    ($query instanceof A || $query instanceof B || ($query instanceof C && $query->ready()))
        ? \PHPStan\dumpType($query)
        : \PHPStan\dumpType($query);
}
function rewritten($query, bool $flag): void {
    if ($query instanceof A || (($query = $flag) && $flag)) {
        \PHPStan\dumpType($query);
    } else {
        \PHPStan\dumpType($query);
    }
    \PHPStan\dumpType($query);
}
function rewriteAnd($query, bool $flag): void {
    if ($query instanceof A && ($query = $flag)) {
        \PHPStan\dumpType($query);
    } else {
        \PHPStan\dumpType($query);
    }
    \PHPStan\dumpType($query);
}
function shortCircuit(): void {
    $query = 1;
    true || ($query = 2);
    \PHPStan\dumpType($query);
    false && ($query = 3);
    \PHPStan\dumpType($query);
}
function scalar($query): void {
    if (is_int($query) || (is_string($query) && $query !== '')) {
        \PHPStan\dumpType($query);
    } else {
        \PHPStan\dumpType($query);
    }
}
