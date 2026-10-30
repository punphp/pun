<?php
interface a {}
interface b {}
interface c {}
/** @param (a&b&c)|(a&b) $value */
function absorbed($value): void {
    \PHPStan\dumpType($value);
}
/** @param (a&b)|(a&b&c) $value */
function reverse($value): void {
    \PHPStan\dumpType($value);
}
