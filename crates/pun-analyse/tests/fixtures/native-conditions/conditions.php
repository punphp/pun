<?php declare(strict_types=1);

namespace NativeConditions;

/** @param false $flag */
function documented(bool $flag): int {
    if ($flag) { return 1; }
    return 2;
}

function native(false $flag): int {
    if ($flag) { return 1; }
    return 2;
}

function assigned(bool $flag): int {
    $flag = false;
    if ($flag) { return 1; }
    return 2;
}

/** @param false $flag */
function overwritten(bool $flag): int {
    $flag = true;
    if ($flag) { return 1; }
    return 2;
}

function literal(): int {
    if (false) { return 1; }
    return 2;
}

function narrowed(bool $flag): int {
    if (!$flag) {
        if ($flag) { return 1; }
    }
    return 2;
}

/** @param false $flag */
function implicit($flag): int {
    if ($flag) { return 1; }
    return 2;
}
