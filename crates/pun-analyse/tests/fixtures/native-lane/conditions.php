<?php
/** @param true $flag */
function docTrue(bool $flag): void {
    $flag; // native
    if ($flag) {
        $flag; // native
        $value = 1;
    } else {
        $flag; // native
        $value = 2;
    }
    $value; // native
}
/** @param false $flag */
function docFalse(bool $flag): void {
    if ($flag) {
        $flag; // native
        $value = 1;
    } else {
        $flag; // native
        $value = 2;
    }
    $value; // native
}
function nativeBool(bool $flag): void {
    if ($flag) {
        $flag; // native
        $value = 1;
    } else {
        $flag; // native
        $value = 2;
    }
    $flag; // native
    $value; // native
    $flag ? 11 : 22; // native
    $flag ?: 33; // native
}
/** @param true $flag */
function firstTernary(bool $flag): void {
    $flag ? ($value = 3) : ($value = 4); // native
    $value; // native
    $flag; // native
}
/** @param true $flag */
function firstElvis(bool $flag): void {
    $flag ?: 33; // native
    $flag; // native
}
function contradictoryIf(): void {
    $flag = false;
    /** @var true $flag */
    $flag; // native
    if ($flag) {
        $flag; // native
        $value = 1;
    } else {
        $flag; // native
        $value = 2;
    }
    $value; // native
}
function contradictoryTernary(): void {
    $flag = false;
    /** @var true $flag */
    $flag;
    $flag ? ($value = 1) : ($value = 2); // native
    $value; // native
    $flag; // native
}
/** @param positive-int $x */
function truthyTernary(int $x): void {
    $x ? ($value = 3) : ($value = 4); // native
    $value; // native
    $x; // native
}
/** @param positive-int $x */
function truthyIf(int $x): void {
    if ($x) { $value = 3; } else { $value = 4; }
    $value; // native
    $x; // native
}
function literalTernary(): void {
    1 ? ($value = 3) : ($value = 4); // native
    $value; // native
}
/** @param true $flag */
function chain(bool $flag, bool $other): void {
    if ($flag) { $value = 1; }
    elseif ($other) {
        $flag; // native
        $other; // native
        $value = 2;
    } else {
        $flag; // native
        $other; // native
        $value = 3;
    }
    $value; // native
}
function nativeChain(bool $first, bool $second): void {
    if ($first) { $value = 1; }
    elseif ($second) { $value = 2; }
    else { $value = 3; }
    $value; // native
    $first; // native
    $second; // native
}
/** @param true $flag */
function nested(bool $flag, bool $other): void {
    if ($flag) {
        if ($other) { $value = 1; } else { $value = 2; }
    } else { $value = 3; }
    $value; // native
}
function implicit(bool $flag): void {
    $value = 0;
    if ($flag) { $value = 1; $new = 2; }
    $value; // native
    $new; // native
    $flag; // native
    if (!$flag) { $value = 3; } else { $value = 4; }
    $value; // native
}
function target(bool $flag): void {
    if ($value = $flag) {
        $value; // native
        $flag; // native
    } else {
        $value; // native
        $flag; // native
    }
    $value; // native
}
function returns(bool $flag): int {
    if ($flag) { return 1; }
    $flag; // native
    return 2;
}
function bare(bool $flag): void {
    if ($flag) { return; } else { $value = 2; }
    $flag; // native
    $value; // native
}
