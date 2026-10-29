<?php
namespace CertaintyControls;

function assignments(int $value): void {
    $result = is_string($value = 'changed');
    is_int($value);
    $result = is_int($value = 7);
    is_string($value);
}
/** @param int $value */
function reassigned($value): void {
    $result = is_string($value = 'changed');
    is_int($value);
}
function unavailable(int $value, string $text): void {
    $length = strlen($text);
    is_string($value);
    is_int($value);
}
function write(mixed &$other): void { $other = 'written'; }
function afterReference(int $value, mixed $other): void {
    write($other);
    is_string($value);
    is_int($value);
}
function remaining(int $value, bool $other): void {
    if (is_int($value)) {
        echo 'first';
    } elseif ($other) {
        echo 'second';
    }
}
/** @phpstan-assert int $value */
function assertInt(mixed $value): void {}
/** @param int $value */
function documentedAssertion($value): void { assertInt($value); }
function nativeAssertion(int $value): void { assertInt($value); }
/** @param true $flag */
function documentedAssert(bool $flag): void { assert($flag); }
function nativeAssert(): void { assert(true); }
function boolCheck(int $value): void { is_bool($value); }
function floatCheck(int $value): void { is_float($value); }
function nullCheck(int $value): void { is_null($value); }
function arrayCheck(int $value): void { is_array($value); }
function objectCheck(int $value): void { is_object($value); }
function iterableCheck(int $value): void { is_iterable($value); }
/** @param int $value */
function removeRemaining($value, bool $other): void {
    if ($other) {
        echo 'first';
    } elseif (is_int($value)) {
        echo 'second';
    } else {
        echo 'last';
    }
}
final class Assertions {
    /** @phpstan-assert int $value */
    public function check(mixed $value): void {}
    /** @phpstan-assert int $value */
    public static function staticCheck(mixed $value): void {}
    public function nativeChecks(int $value): void {
        $this->check($value);
        self::staticCheck($value);
    }
}
function nativeFalseAssert(): void { assert(false); }
function scalarFallback(int $value): void { is_scalar($value); }
