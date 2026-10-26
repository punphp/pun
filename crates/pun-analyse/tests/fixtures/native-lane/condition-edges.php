<?php
function unions(int|string|null $value): void {
    if ($value === 2) {
        $value; // native
    } else {
        $value; // native
    }
    if ($value === 'yes') {
        $value; // native
    } else {
        $value; // native
    }
    $value; // native
}
function literalBranches(): void {
    $text = 'yes';
    if ($text === 'yes') {
        $text; // native
    } else {
        $text; // native
    }
    $text; // native
    $flag = false;
    if (!$flag) {
        $flag; // native
    } else {
        $flag; // native
    }
    $number = 1;
    if ($number !== 1) {
        $number; // native
    } else {
        $number; // native
    }
    if (false) { $number = 3; }
    $number; // native
    false ? ($number = 4) : ($number = 5); // native
    $number; // native
}
/** @param false $flag */
function falseTernary(bool $flag): void {
    $flag ? ($value = 6) : ($value = 7); // native
    $flag; // native
    $value; // native
}
function integerEdges(int $x): void {
    if ($x > 0) {
        $x > 0 ? 1 : 2; // native
        $x >= 0 ? 1 : 2; // native
        $x < 0 ? 1 : 2; // native
        $x <= 0 ? 1 : 2; // native
        if ($x !== 0) {
            $x; // native
        }
    } else {
        $x > 0 ? 1 : 2; // native
        $x >= 1 ? 1 : 2; // native
        $x < 1 ? 1 : 2; // native
        $x <= 0 ? 1 : 2; // native
    }
    if ($x >= 9223372036854775807) {
        $x; // native
    }
    if ($x > 9223372036854775807) {
        $x; // native
    }
    if ($x <= -9223372036854775807 - 1) {
        $x; // unavailable: arithmetic condition is deliberately outside this slice
    }
}
function nullableTruth(?string $x): void {
    if ($x) {
        $x; // native
    } else {
        $x; // native
    }
    $x; // native
}
function assignmentOnly(bool $flag): void {
    $flag ? ($new = 2) : 3; // native
    $new; // native
}
function rangeIdentity(int $x): void {
    ($x % 2) === 1; // native
    $x; // native
}
/** @param 1 $x */
function disjointInteger(int $x): void {
    if ($x === 2) {
        $x; // native
    } else {
        $x; // native
    }
    $x; // native
    if ($x !== 2) {
        $x; // native
    } else {
        $x; // native
    }
}
/** @param 'known' $x */
function disjointString(string $x): void {
    if ($x === 'other') {
        $x; // native
    } else {
        $x; // native
    }
    $x; // native
    if ($x !== 'other') {
        $x; // native
    } else {
        $x; // native
    }
}
function literalTruth(): void {
    $x = '0';
    if ($x) {
        $x; // native
    } else {
        $x; // native
    }
    $x = 'yes';
    if ($x) {
        $x; // native
    } else {
        $x; // native
    }
}
function nonemptyTruth(string $text): void {
    $x = '0' . $text;
    if ($x) {
        $x; // native
    } else {
        $x; // native
    }
    $x; // native
}
function neverTruth(): void {
    $flag = false;
    /** @var true $flag */
    $flag;
    if ($flag) {
        if ($flag) {
            $flag; // native
        } else {
            $flag; // native
        }
    }
}
/** @param positive-int $x */
function decidedZero(int $x): void {
    if ($x !== 0) {
        $x; // native
    } else {
        $x; // native
    }
    $x; // native
}
/** @param int $x */
function decidedNull(?int $x): void {
    if ($x === null) {
        $x; // native
    } else {
        $x; // native
    }
    $x; // native
}
function broaderFull(): void {
    $x = 1;
    /** @var int $x */
    $x;
    if ($x === 2) {
        $x; // native
    } else {
        $x; // native
    }
    $text = 'yes';
    /** @var string $text */
    $text;
    if ($text === 'other') {
        $text; // native
    } else {
        $text; // native
    }
}
function weakOrdered(int $x): void {
    $x <= 5 ? 1 : 2; // native
}
function literalBoolean(): void {
    $flag = false;
    if ($flag === true) {
        $flag; // native
    } else {
        $flag; // native
    }
}
