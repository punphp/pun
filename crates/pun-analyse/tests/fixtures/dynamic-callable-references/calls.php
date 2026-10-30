<?php
function callableReferenceMatrix(array $values, bool $condition): void {
    $reference = function (&$value): void { $value = 11; };
    $read = function ($value): void { echo $value; };
    $reference($values[]);
    $read($values[]);
    $arrow = fn (&$value) => $value = 12;
    $arrow($values[]);
    $reference(value: $values[]);
    $variadic = fn (&...$items) => count($items);
    $variadic($values[], $values[]);
    ($condition ? $reference : $read)($values[]);
    $holder = new stdClass();
    $holder->callback = $reference;
    ($holder->callback)($values[]);
    $factory = fn () => $reference;
    $factory()($values[]);
    $notCallable = 17;
    $notCallable($values[]);
}
class DynamicCallableOutput {
    public function write(&$value): void { $value = 13; }
    public function read($value): void { echo $value; }
    public static function staticWrite(&$value): void { $value = 14; }
    public function __invoke(&$value): void { $value = 15; }
}
function callableObjectMatrix(array $values): void {
    $object = new DynamicCallableOutput();
    $method = [$object, 'write'];
    $method($values[]);
    $read = [$object, 'read'];
    $read($values[]);
    $firstClass = $object->write(...);
    $firstClass($values[]);
    $static = 'DynamicCallableOutput::staticWrite';
    $static($values[]);
    $staticArray = [DynamicCallableOutput::class, 'staticWrite'];
    $staticArray($values[]);
    $object($values[]);
}
function dynamicOutput(&$value): void { $value = 16; }
function callableStringMatrix(array $values): void {
    $reference = 'dynamicOutput';
    $reference($values[]);
    $read = 'strlen';
    $read($values[]);
    $pop = array_pop(...);
    $pop($values[]);
    $scan = sscanf(...);
    $scan('17', '%d', $values[]);
    $scan('17', '%d');
    $scan(string: '17', format: '%d', vars: $values[]);
}
function maybeDynamicCallable(array $values, bool $condition): void {
    if ($condition) {
        $maybe = function (&$value): void { $value = 18; };
    }
    $maybe($values[]);
}
function unknownDynamicCallable(array $values, callable $unknown): void {
    $unknown($values[]);
    $missing($values[]);
}
function identicalCallableSources(array $values, bool $condition): void {
    $left = function (&$value): void { echo $value; };
    $right = function (&$value): void { echo $value; };
    ($condition ? $left : $right)($values[]);
    $left = function ($value): void { echo $value; };
    $right = function ($value): void { echo $value; };
    ($condition ? $left : $right)($values[]);
    $referenceInt = function (int &$value): void { echo $value; };
    $readString = function (string $value): void { echo $value; };
    ($condition ? $referenceInt : $readString)($values[]);
}
