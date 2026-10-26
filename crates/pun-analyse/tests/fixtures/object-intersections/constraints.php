<?php
interface A {}
interface B {}
/** @template T */
class Box {}

/** @param (Box<int>&A)|(Box<string>&A) $value */
function genericAlternatives($value): void {
    \PHPStan\dumpType($value);
}
/** @param (A&B)|(Box<int>&B) $value */
function distinctPrimaryConjuncts($value): void {
    \PHPStan\dumpType($value);
}
class Receiver {
    /** @param (static&A)|(self&A) $value */
    public function staticAlternatives($value): void {
        \PHPStan\dumpType($value);
    }
    /** @param ($this&A)|(self&A) $value */
    public function thisAlternatives($value): void {
        \PHPStan\dumpType($value);
    }
}
