<?php
namespace NativeArrayOrderObservations;

function insertionOrder(): void
{
    $shape = ['second' => null, 'first' => 1];
    \PHPStan\dumpNativeType($shape);
}

function nested(): void
{
    $shape = ['z' => ['second' => null, 'first' => 1], 'a' => 2];
    $copy = $shape['z'];
    \PHPStan\dumpNativeType($copy);
}

function duplicate(): void
{
    $shape = ['second' => 10, 'first' => 1, 'second' => null];
    \PHPStan\dumpNativeType($shape);
}

function integerKeys(): void
{
    $shape = [1 => 'second', 0 => 'first'];
    \PHPStan\dumpNativeType($shape);
}

/** @param array{0: object, 1: non-empty-string}&callable $callback */
function typedCallable(array $callback): void
{
    \PHPStan\dumpNativeType($callback);
}

/** @param array{0: object, 1: non-empty-string}&callable $callback */
function untypedCallable($callback): void
{
    \PHPStan\dumpNativeType($callback);
}

function annotatedCallable($unknown): void
{
    /** @var array{0: object, 1: non-empty-string}&callable $callback */
    $callback = $unknown;
    \PHPStan\dumpNativeType($callback);
}

function unknownKey(string $key): void
{
    $shape = [$key => 1];
    \PHPStan\dumpNativeType($shape); // unavailable
}

function escapedCopy(): void
{
    $shape = ['second' => null, 'first' => 1];
    $reference =& $shape;
    \PHPStan\dumpNativeType($shape);
}
