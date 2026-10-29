<?php
namespace NativeInference;

function identity(int $value): int { return $value; }
/** @param false $flag */
function throughUser(bool $flag): int {
    $value = identity(1);
    if ($flag) { return 1; }
    return 2;
}
/** @param false $flag */
function throughBuiltin(bool $flag, int $value): int {
    $checked = is_int($value);
    if ($flag) { return 1; }
    return 2;
}
/** @param false $flag */
function throughDiagnostic(bool $flag): int {
    \PHPStan\dumpType($flag);
    if ($flag) { return 1; }
    return 2;
}
function nativeControl(false $flag, int $value): int {
    $checked = is_int($value);
    if ($flag) { return 1; }
    return 2;
}
function write(mixed &$value): void { $value = 'changed'; }
function referenceControl(int $value, mixed $other): void {
    write($other);
    is_string($value);
    is_int($value);
}
