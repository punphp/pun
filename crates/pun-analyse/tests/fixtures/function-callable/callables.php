<?php

namespace PunFunctionCallable;

use function strlen as length;
use function PunFunctionCallable\NamedTarget as imported;

function NamedTarget(): void {}

function names(): void
{
    NamedTarget(...);
    NAMEDTARGET(...);
    \PunFunctionCallable\NAMEDTARGET(...);
    namespace\NAMEDTARGET(...);
    imported(...);
    IMPORTED(...);
    StRlEn(...);
    \StRlEn(...);
    length(...);
    absent(...);
    if (function_exists('guarded')) {
        guarded(...);
    }
    guarded(...);
}

final class Invokable
{
    public function __invoke(): void {}
}

final class NonCallable
{
    public function check(): void
    {
        $this(...);
        ($this)(...);
    }
}

class PossiblyInvokable {}

/** @param callable(): void $callback */
function expressions(
    string $string,
    int $integer,
    callable $callback,
    object $object,
    mixed $explicit,
    $implicit,
    ?\Closure $nullable,
    \Closure|int $union,
    Invokable $invokable,
    NonCallable $nonCallable,
    PossiblyInvokable $possible,
    Unknown $unknown,
    array $array,
): void {
    $string(...);
    $integer(...);
    $callback(...);
    $object(...);
    $explicit(...);
    $implicit(...);
    $nullable(...);
    $union(...);
    $invokable(...);
    $nonCallable(...);
    $possible(...);
    $unknown(...);
    $array(...);
    if (class_exists(Unknown::class)) {
        $unknown(...);
    }
    if (is_callable($string)) {
        $string(...);
    }
}

function literals(): void
{
    $integer = 1;
    $integer(...);
    $null = null;
    $null(...);
    $builtin = 'strlen';
    $builtin(...);
    $user = 'PunFunctionCallable\\NamedTarget';
    $user(...);
    $missing = 'definitely_missing_callable';
    $missing(...);
    $closure = static function (): void {};
    $closure(...);
    $array = [];
    $array(...);
}

/**
 * @param \Closure(int): string $typed
 * @param \Closure(mixed...): mixed $variadic
 * @param callable(mixed...): mixed $variadicCallable
 */
function signatures(\Closure $typed, \Closure $variadic, callable $variadicCallable): void
{
    $typedUnion = rand(0, 1) ? $typed : 1;
    $typedUnion(...);
    $variadicUnion = rand(0, 1) ? $variadic : 1;
    $variadicUnion(...);
    $callableUnion = rand(0, 1) ? $variadicCallable : 1;
    $callableUnion(...);
    $literal = static function (...$args): mixed { return null; };
    $literalUnion = rand(0, 1) ? $literal : 1;
    $literalUnion(...);
}
