<?php
final class NativeReturnBox { public int $present; }
/** @return object{present: int} */
function objectFactory(): object { return (object) ['present' => 1]; }
function namedFactory(): NativeReturnBox { return new NativeReturnBox(); }
function argumentsInOrder(mixed $first, mixed $second): int { return 1; }
/** @param-out string $value */
function writeReference(mixed &$value): int { $value = 'written'; return 1; }
function diagnosticContinuation(int $value): void {
    \PHPStan\dumpType($value /* native */) /* native */;
    $value /* native */;
    \PHPStan\dumpNativeType($value /* native */, $value = 2 /* native */) /* native */;
    $value /* native */;
    \PHPStan\dumpPhpDocType($value /* native */) /* native */;
    $value /* native */;
}
/** @param object{present: int} $shape */
function diagnosticShape($shape): void {
    \PHPStan\dumpType($shape /* native */);
    property_exists($shape /* native */, 'absent' /* native */) /* native */;
    $shape /* native */;
}
function realObjectReturns(): void {
    $object = objectFactory() /* native */;
    $object /* native */;
    property_exists($object /* native */, 'absent' /* native */) /* native */;
    $object = namedFactory() /* native */;
    $object /* native */;
    property_exists($object /* native */, $property = 'present' /* native */) /* native */;
    $property /* native */;
}
function assignedArguments(int $value): void {
    argumentsInOrder($value /* native */, $value = 7 /* native */) /* native */;
    $value /* native */;
    argumentsInOrder($value = 9 /* native */, $value /* native */) /* native */;
    is_string($value = 'changed' /* native */) /* native */;
    $value /* native */;
    is_int($value /* native */) /* native */;
    $value /* native */;
}
function unknownContinuation(int $value): void {
    missingFunction($value /* native */, $value = 3 /* native */) /* unavailable */;
    $value /* unavailable */;
    $value = 99;
    $value /* unavailable */;
}
function referenceContinuation(int $value): void {
    writeReference($value /* native */) /* unavailable */;
    $value /* unavailable */;
}
function unsupportedChild(int $value): void {
    argumentsInOrder($value /* native */, missingFunction($value /* native */) /* unavailable */) /* unavailable */;
    $value /* unavailable */;
}
function laterArgumentAfterEffect(int $value): void {
    missingFunction(missingFunction($value /* native */) /* unavailable */, $value /* unavailable */) /* unavailable */;
    $value /* unavailable */;
}
function diagnosticChild(int $value): void {
    \PHPStan\dumpType(writeReference($value /* native */) /* unavailable */) /* unavailable */;
    $value /* unavailable */;
}
function unsupportedBuiltin(int $value): void {
    strlen($value = 'text' /* native */) /* unavailable */;
    $value /* native */;
}
