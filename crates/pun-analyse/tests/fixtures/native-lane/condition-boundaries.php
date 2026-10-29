<?php
/** @param true $flag */
function deadUnsupported(bool $flag): void {
    if ($flag) { $value = 1; } else { $value = [2]; }
    $value; // native
}
function liveUnsupported(bool $flag): void {
    $flag; // native
    if ($flag) { $value = 1; } else { $value = [2]; }
    $value; // native
    $value = 99;
    $value; // native
}
/** @param true $flag */
function selectedContinuation(bool $flag): void {
    $flag ? ($value = 1) : [2]; // native
    $value; // native
    $flag; // native
}
function valueWithoutContinuation(): void {
    $flag = false;
    /** @var true $flag */
    $flag;
    $flag ? [1] : 2; // native
    $flag; // native
}
function unsupportedElseif(bool $flag, int $value): void {
    if ($flag) { $value = 1; }
    elseif ($value + 1) {
        $value; // unavailable
    }
    $value; // unavailable
}
function mixedGuard(mixed $value): void {
    $value; // native
    if ($value) {
        $value; // unavailable
    } else {
        $value; // unavailable
    }
    $value; // unavailable
}
function floatGuard(float $value): void {
    if ($value > 0) {
        $value; // unavailable
    }
    $value; // unavailable
}
function prunedElseWalk(bool $flag): void {
    if ($flag) { $value = 1; }
    elseif (!$flag) { $value = 2; }
    else { $value = 3; }
    $value; // native
}
function floatingLimit(int $x): void {
    if ($x > 0.5) {
        $x; // unavailable
    }
    $x; // unavailable
}
function floatingIdentity(float $x): void {
    if ($x === 0.5) {
        $x; // unavailable
    }
    $x; // unavailable
}
function numericText(float $decimal): void {
    $text = (string) $decimal;
    if ($text) {
        $text; // unavailable
    }
    $text; // unavailable
}
