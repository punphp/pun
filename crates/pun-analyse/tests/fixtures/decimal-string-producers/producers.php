<?php
namespace DecimalStringProducers;

function integer(int $value): void {
    \PHPStan\dumpType((string) $value);
    $copy = (string) $value;
    \PHPStan\dumpType($copy); echo $copy;
}

/**
 * @param positive-int $positive
 * @param negative-int $negative
 * @param int<-2, 2> $finite
 * @param int<0, 129> $wide
 * @param int<0, max> $zero
 * @param int<-9223372036854775808, -9223372036854775807> $edge
 */
function ranges(int $positive, int $negative, int $finite, int $wide, int $zero, int $edge): void {
    \PHPStan\dumpType((string) $positive);
    \PHPStan\dumpType((string) $negative);
    \PHPStan\dumpType((string) $finite);
    \PHPStan\dumpType((string) $wide);
    \PHPStan\dumpType((string) $zero);
    \PHPStan\dumpType((string) $edge);
}

/**
 * @param decimal-int-string $decimal
 * @param non-decimal-int-string $inverse
 * @param numeric-string $numeric
 * @param decimal-int-string&non-empty-string $nonempty
 * @param decimal-int-string&non-falsy-string $truthy
 */
function documented(string $decimal, string $inverse, string $numeric, string $nonempty, string $truthy): void {
    \PHPStan\dumpType($decimal);
    \PHPStan\dumpType($inverse);
    \PHPStan\dumpType($numeric);
    \PHPStan\dumpType($nonempty);
    \PHPStan\dumpType($truthy);
}

/** @template T of decimal-int-string
 * @param T $value
 */
function template(string $value): void {
    \PHPStan\dumpType($value);
}

function formats(): void {
    \PHPStan\dumpType((string) -12);
    \PHPStan\dumpType((string) 0);
    \PHPStan\dumpType('01');
    \PHPStan\dumpType('+1');
    \PHPStan\dumpType('-0');
    \PHPStan\dumpType('1.0');
    \PHPStan\dumpType('1e2');
    \PHPStan\dumpType('9223372036854775808');
}

/** @param int<0, 128> $value */
function threshold(int $value): void {
    \PHPStan\dumpType((string) $value);
}

function mixedInteger(int|bool $value): void {
    \PHPStan\dumpType((string) $value);
}

/** @param -9223372036854775808 $value */
function minimum(int $value): void {
    \PHPStan\dumpType((string) $value);
}

/**
 * @param non-decimal-int-string&callable-string $inverse
 * @param callable-string&lowercase-string $lowercase
 */
function callableMetadata(string $inverse, string $lowercase): void {
    \PHPStan\dumpType($inverse);
    \PHPStan\dumpType($lowercase);
}
