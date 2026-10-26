<?php
function nullable(?int $x): void {
    if ($x === null) {
        $x; // native
    } else {
        $x; // native
    }
    $x; // native
    if ($x !== null) { return; }
    $x; // native
}
function integer(int $x): void {
    if ($x === 1) {
        $x; // native
    } else {
        $x; // native
    }
    $x; // native
    if ($x !== 0) {
        $x; // native
    } else {
        $x; // native
    }
}
/** @param 1 $x */
function docInteger(int $x): void {
    if ($x === 1) {
        $x; // native
    } else {
        $x; // native
    }
    $x; // native
}
/** @param 'known' $x */
function docString(string $x): void {
    if ($x === 'known') {
        $x; // native
    } else {
        $x; // native
    }
    $x; // native
}
function text(string $x): void {
    if ('known' === $x) {
        $x; // native
    } else {
        $x; // native
    }
    if ($x) {
        $x; // native
    } else {
        $x; // native
    }
    $x; // native
}
function truthy(?int $x): void {
    if (!$x) {
        $x; // native
    } else {
        $x; // native
    }
    $x; // native
    $x ?: 5; // native
}
/** @param positive-int $x */
function ordered(int $x): void {
    $x > 0 ? $x : -1; // native
    $x; // native
}
function bounds(int $x): void {
    if ($x > 5) {
        $x; // native
    } else {
        $x; // native
    }
    if (5 >= $x) {
        $x; // native
    } else {
        $x; // native
    }
    if ($x < -3) {
        $x; // native
    } else {
        $x; // native
    }
    if ($x >= -3) {
        $x; // native
    } else {
        $x; // native
    }
    $x; // native
}
function booleans(bool $flag): void {
    if ($flag === true) {
        $flag; // native
    } else {
        $flag; // native
    }
    if (false !== $flag) {
        $flag; // native
    } else {
        $flag; // native
    }
    $flag; // native
}
function replacement(?int $x): void {
    if ($x === null) { $x = 1; } else { $x = 2; }
    $x; // native
    $x = 5;
    $x; // native
}
function impossible(bool $flag): void {
    if ($flag) {
        if (!$flag) {
            $flag; // native
        } else {
            $flag; // native
        }
    }
}
