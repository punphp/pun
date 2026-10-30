<?php

namespace CallableArrayDescriptionNative;

/** @param array{0: object, 1: non-empty-string}&callable $value */
function typed(array $value): void
{
    \PHPStan\dumpNativeType($value);
    $value = 42;
    \PHPStan\dumpNativeType($value);
}

/** @param array{0: object, 1: non-empty-string}&callable $value */
function untyped($value): void
{
    \PHPStan\dumpNativeType($value);
}

/** @param array{0: object, 1: non-empty-string}&callable $value */
function reference(array &$value): void
{
    \PHPStan\dumpNativeType($value); // unavailable: reference parameter
}
