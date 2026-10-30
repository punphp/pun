<?php

namespace IssetCertaintyNativeObservations;

function afterCall(int $value): void
{
    \PHPStan\dumpNativeType($value);
    var_dump($value);
    \PHPStan\dumpNativeType($value); // supported native call continuation
}

function afterUnset(): void
{
    $shape = ['present' => 1];
    \PHPStan\dumpNativeType($shape);
    unset($shape['present']);
    \PHPStan\dumpNativeType($shape); // supported native unset transfer
}

function insertionOrder(): void
{
    $shape = ['second' => null, 'first' => 1];
    \PHPStan\dumpNativeType($shape);
}

/** @param array{0: object, 1: non-empty-string}&callable $callback */
function typedCallableArray(array $callback): void
{
    \PHPStan\dumpNativeType($callback);
}

/** @param array{0: object, 1: non-empty-string}&callable $callback */
function untypedCallableArray($callback): void
{
    \PHPStan\dumpNativeType($callback);
}
