<?php
/** @param object{present: int} $shape */
function docParameter($shape): void {
    $shape; // before
    property_exists($shape, 'absent');
    property_exists($shape, 'present');
}
function localAnnotation($input): void {
    /** @var object{present: int} $shape */
    $shape = $input;
    $shape; // before
    property_exists($shape, 'absent');
    property_exists($shape, 'present');
}
/** @return object{present: int} */
function factory(): object { return (object) ['present' => 1]; }
function returned(): void {
    $shape = factory();
    $shape; // before
    property_exists($shape, 'absent');
    property_exists($shape, 'present');
}
final class NativeBox { public int $present; }
function nativeObject(NativeBox $object): void {
    $object; // before
    property_exists($object, 'absent');
    property_exists($object, 'present');
}
function nativeScalar(int $value): void {
    $value; // before
    is_string($value);
    is_int($value);
}
/** @param int $value */
function docScalar($value): void {
    $value; // before
    is_string($value);
    is_int($value);
}
/** @param 'known' $value */
function docStringNativeString(string $value): void {
    $value; // before
    is_int($value);
    is_string($value);
}
/** @param int $value */
function reassigned($value): void {
    $value = 1;
    $value; // before
    is_string($value);
    is_int($value);
}
