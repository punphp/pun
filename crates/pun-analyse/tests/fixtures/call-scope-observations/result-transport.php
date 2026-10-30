<?php

final class TransportInspect {
    public function left($values) {}
    public function right($values) {}
}
/** @phpstan-pure */
function transportVoid(): void {}
/** @phpstan-pure */
function transportInt(): int { return 7; }

final class TransportReturns {
    public function empty(): void {}
    public function number(): int { return 7; }
    public static function staticEmpty(): void {}
    public static function staticNumber(): int { return 7; }
}

/** @param 'left'|'right' $name */
function transportResults(string $name, bool $flag): void {
    $inspect = new TransportInspect();
    $callable = 'transportVoid';
    $inspect->$name([
        'callable' => $callable()/* result */,
        'integer' => transportInt()/* result */,
        'name' => $name,
        'named' => transportVoid()/* result */,
    ]); // observe
    $inspect->$name([
        'all' => match ($flag) { true => transportVoid(), false => transportVoid() }/* result */,
        'name' => $name,
        'some' => match ($flag) { true => transportVoid(), false => 1 }/* result */,
    ]); // observe
    $mixedCallable = $flag ? 'transportVoid' : 'transportInt';
    $method = $flag ? 'empty' : 'number';
    $staticMethod = $flag ? 'staticEmpty' : 'staticNumber';
    $returns = new TransportReturns();
    $inspect->$name([
        'callable' => $mixedCallable()/* result */,
        'instance' => $returns->$method()/* result */,
        'name' => $name,
        'static' => TransportReturns::$staticMethod()/* result */,
    ]); // observe
    $remembered = $callable();
    $inspect->$name([
        'branch' => [$flag ? $callable() : 13]/* result */,
        'coalesce' => [$callable() ?? 17]/* result */,
        'name' => $name,
        'named' => [$flag ? transportVoid() : 19]/* result */,
        'remembered' => $remembered/* result */,
    ]); // observe
    $nullableCallable = $flag ? 'transportVoid' : 'transportNull';
    $inspect->$name([
        'name' => $name,
        'named' => [transportVoid() ?? 37]/* result */,
        'nonnullable' => [$mixedCallable() ?? 29]/* result */,
        'null' => [transportNull() ?? 31]/* result */,
        'nullable' => [$nullableCallable() ?? 23]/* result */,
    ]); // observe
}

/** @phpstan-pure */
function transportNull(): null { return null; }
