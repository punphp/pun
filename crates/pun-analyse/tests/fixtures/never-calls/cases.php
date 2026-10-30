<?php

namespace NeverCalls;

use function PHPStan\dumpType;

function acceptInt(int $value): void {}

function stop(): never
{
    throw new \RuntimeException();
}

/** @return never */
function docStop()
{
    throw new \RuntimeException();
}

/** @return ($value is 0 ? never : int) */
function conditionalStop(int $value, string $label): int
{
    if ($value === 0) {
        throw new \RuntimeException($label);
    }
    return $value;
}

class Terminator
{
    public function stop(): never
    {
        throw new \RuntimeException();
    }

    /** @return never */
    public static function docStop()
    {
        throw new \RuntimeException();
    }

    public function take(int $value): int
    {
        return $value;
    }
}

function direct(): void
{
    filter_input(-1, 'first');
    acceptInt('after-direct');
}

function assigned(): void
{
    $value = filter_input(-1, 'assigned', FILTER_VALIDATE_INT);
    dumpType($value);
    acceptInt('after-assignment');
}

function parenthesized(): void
{
    (filter_input(-1, 'parenthesized'));
    acceptInt('after-parentheses');
}

function nested(): void
{
    acceptInt(filter_input(-1, 'nested'));
    acceptInt('after-nesting');
}

function nativeStop(): void
{
    stop();
    acceptInt('unreachable-native');
}

function documentedStop(): void
{
    docStop();
    acceptInt('unreachable-doc');
}

function callableStop(): void
{
    $stop = stop(...);
    $stop();
    acceptInt('unreachable-callable');
}

function methodStop(Terminator $object): void
{
    $object->stop();
    acceptInt('unreachable-method');
}

function staticStop(): void
{
    Terminator::docStop();
    acceptInt('unreachable-static');
}

function conditionalNever(): void
{
    conditionalStop(label: 'stop', value: 0);
    acceptInt('unreachable-conditional');
}

function conditionalInt(): void
{
    dumpType(conditionalStop(label: 'continue', value: 3));
    acceptInt('after-conditional');
}

function closureImplicit(): void
{
    $closure = fn () => filter_input(-1, 'closure');
    dumpType($closure());
    acceptInt('after-implicit-closure');
}

function closureExplicit(): void
{
    $closure = fn () => exit();
    $closure();
    acceptInt('unreachable-explicit-closure');
}

function forwardedStatement(): void
{
    call_user_func(fn () => exit());
    acceptInt('unreachable-forwarded-statement');
}

function forwardedNested(): void
{
    acceptInt(call_user_func(fn () => exit()));
    acceptInt('after-nested-forwarding');
}

function shortCircuit(bool $flag): void
{
    $flag && stop();
    acceptInt('after-short-circuit');
}

function unevaluated(): void
{
    $closure = fn () => stop();
    acceptInt('after-unevaluated');
}

function nullable(?Terminator $object): void
{
    $object?->stop();
    acceptInt('after-nullsafe');
}

function loop(bool $continue): void
{
    while ($continue) {
        filter_input(-1, 'loop');
        acceptInt('after-loop-call');
        break;
    }
    acceptInt('after-loop');
}

function implicitFinally(): int
{
    try {
        return 1;
    } finally {
        filter_input(-1, 'finally');
        acceptInt('after-implicit-finally');
    }
}

function explicitFinally(): int
{
    try {
        return 1;
    } finally {
        stop();
        acceptInt('unreachable-explicit-finally');
    }
}

function forwardedArrayStatement(): void
{
    call_user_func_array(fn () => exit(), []);
    acceptInt('unreachable-array-forwarded-statement');
}

function forwardedArrayNested(): void
{
    acceptInt(call_user_func_array(fn () => exit(), []));
    acceptInt('after-array-nested-forwarding');
}

function nonnullNullsafe(Terminator $object): void
{
    acceptInt($object?->stop());
    acceptInt('unreachable-nonnull-nullsafe');
}

function explicitLoop(bool $continue): void
{
    while ($continue) {
        stop();
        acceptInt('unreachable-loop');
    }
    acceptInt('after-explicit-loop');
}

/** @param never $value */
function neverArgument($value): void
{
    acceptInt($value);
    acceptInt('after-never-argument');
}

function closureWithoutReturn(): void
{
    $closure = function () { exit(); };
    dumpType($closure);
    $closure();
    acceptInt('unreachable-body-without-return');
}

function closureWithImplicitReturn(): void
{
    $closure = function () { return filter_input(-1, 'returned'); };
    dumpType($closure);
    dumpType($closure());
    acceptInt('after-returned-implicit-never');
}

function closureWithImplicitStatement(): void
{
    $closure = function () { filter_input(-1, 'statement'); };
    dumpType($closure);
    dumpType($closure());
    acceptInt('after-implicit-body-statement');
}

function someTargetsReturn(bool $choose): void
{
    $callback = $choose ? stop(...) : fn () => 17;
    dumpType($callback());
    acceptInt('after-one-returning-target');
}

function everyTargetStops(bool $choose): void
{
    $callback = $choose ? stop(...) : docStop(...);
    $callback();
    acceptInt('unreachable-after-all-targets');
}

function nestedDeclaredStop(): void
{
    acceptInt(stop());
    acceptInt('unreachable-after-nested-declaration');
}

function nullableArgumentStop(?Terminator $object): void
{
    $object?->take(stop());
    acceptInt('after-nullable-argument-stop');
}

function nonnullNullsafeArgumentStop(Terminator $object): void
{
    $object?->take(stop());
    acceptInt('after-nonnull-nullsafe-argument-stop');
}

/** @return never */
function objectDeclarationStop(): object
{
    throw new \RuntimeException();
}

/** @return never */
function arrayDeclarationStop(): array
{
    throw new \RuntimeException();
}

/** @return never */
function voidDeclarationStop(): void
{
    throw new \RuntimeException();
}

function documentedObjectStop(): void
{
    objectDeclarationStop();
    acceptInt('unreachable-object-declaration');
}

function documentedArrayStop(): void
{
    arrayDeclarationStop();
    acceptInt('unreachable-array-declaration');
}

function documentedVoidStop(): void
{
    voidDeclarationStop();
    acceptInt('unreachable-void-declaration');
}

function forwardedImplicit(): void
{
    $value = call_user_func(fn () => filter_input(-1, 'forwarded-implicit'));
    dumpType($value);
    acceptInt('after-forwarded-implicit');
}
