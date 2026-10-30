<?php
namespace DecimalStringNative;

/** @param positive-int $value */
function integer(int $value): void {
    (string) $value; // native
    (string) 0; // native
    (string) -12; // native
    (string) ($value % 2); // native
    $copy = (string) $value;
    $copy; // native
}
