<?php
interface A {}
interface B {}
interface C {}
class ParentClass {}
class ChildClass extends ParentClass {}
/** @template T */
class Box {}

/** @param (A&B&C)|(A&B) $value */
function absorbed($value): void {
    \PHPStan\dumpType($value);
}
/** @param (A&B)|(A&B&C) $value */
function absorbedReverse($value): void {
    \PHPStan\dumpType($value);
}
/** @param A&B $value */
function overlapping($value): void {
    if ($value instanceof A) {
        \PHPStan\dumpType($value);
    }
    if ($value instanceof C) {
        \PHPStan\dumpType($value);
    }
}
function inheritance(ParentClass $value): void {
    if ($value instanceof ChildClass) {
        \PHPStan\dumpType($value);
        if ($value instanceof ParentClass) {
            \PHPStan\dumpType($value);
        }
    }
}
/** @param Box<int> $value */
function generic(Box $value): void {
    if ($value instanceof A) {
        \PHPStan\dumpType($value);
        if ($value instanceof B) {
            \PHPStan\dumpType($value);
        }
    }
}
class Receiver {
    public function inspect(): void {
        if ($this instanceof A) {
            \PHPStan\dumpType($this);
        }
    }
    /** @param static $value */
    public function inspectStatic(self $value): void {
        if ($value instanceof A) {
            \PHPStan\dumpType($value);
        }
    }
}
