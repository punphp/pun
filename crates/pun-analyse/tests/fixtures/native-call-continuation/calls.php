<?php

namespace NativeCallContinuation;

function afterCall(int $value): void
{
    \PHPStan\dumpNativeType($value);
    $value; // observe
    var_dump($value); // unavailable
    \PHPStan\dumpNativeType($value);
    $value; // observe
    printf('%d', $value, 2); // unavailable
    $value; // observe
    strlen('hello'); // unavailable
    $value; // observe
    (var_dump($value)); // unavailable
    $value; // observe
    var_dump($value = 7, $value); // unavailable
    $value; // observe
}

function variadic(int ...$values): array { return $values; }
function returningArray(): array { return []; }
function noReturnHint() { return 1; }

function userCalls(int $value): void
{
    variadic($value, 4); // unavailable
    $value; // observe
    returningArray(); // unavailable
    $value; // observe
    noReturnHint(); // observe
    $value; // observe
}

function assignedResult(int $value): void
{
    $result = var_dump($value); // unavailable
    $value; // unavailable
}

function unpacked(int $value): void
{
    var_dump(...[1, 2]); // unavailable
    $value; // unavailable
}

function named(int $value): void
{
    var_dump(value: $value); // unavailable
    $value; // unavailable
}

function mixedArgument(int $value, $unknown): void
{
    var_dump($unknown); // unavailable
    $value; // unavailable
}

class Box { public int $value = 1; }
function objectArgument(int $value, Box $box): void
{
    var_dump($box); // unavailable
    $value; // unavailable
}

function reference(int &$value): array { $value = 3; return []; }
function byReference(int $value): void
{
    reference($value); // unavailable
    $value; // unavailable
}

function globals(): array { global $value; $value = 'changed'; return []; }
function globalEffects(int $value): void
{
    globals(); // unavailable
    $value; // unavailable
}

function implicitGlobal(int $value): void
{
    file_get_contents('file.txt'); // unavailable
    $value; // unavailable
}

function callback(int $value): void
{
    array_map('intval', [1]); // unavailable
    $value; // unavailable
}

function unknown(int $value): void
{
    missing(); // unavailable
    $value; // unavailable
}

function dynamic(int $value, string $function): void
{
    $function(1); // unavailable
    $value; // unavailable
}

/** @phpstan-assert int $value */
function assertion($value): array { return []; }
function assertionEffects(int $value): void
{
    assertion($value); // unavailable
    $value; // unavailable
}

function scalarCallback(int $value): void
{
    call_user_func('intval', 1); // unavailable
    $value; // unavailable
}

function implicitOpenGlobal(int $value): void
{
    fopen('file.txt', 'r'); // unavailable
    $value; // unavailable
}

function builtinAssertion(int $value): void
{
    assert($value); // unavailable
    $value; // unavailable
}

function unsupportedArgument(int $value): void
{
    var_dump(missing()); // unavailable
    $value; // unavailable
}

/** @param int $unknown */
function documentedArgument(int $value, $unknown): void
{
    var_dump($unknown); // unavailable
    $value; // unavailable
}

function arrayArgument(int $value): void
{
    var_dump([1, 2]); // unavailable
    $value; // unavailable
}

function scalarWrite(int $value): void
{
    strlen($value = 'text'); // unavailable
    $value; // observe
}
