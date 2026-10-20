<?php

namespace CovMissingCallable;

/**
 * @property callable $prop
 * @method void run(\Closure $c)
 * @mixin \Closure
 */
class C
{
    /** @var callable(): void */
    public $ok;

    /** @var \Closure */
    public $closure;

    /** @var callable */
    public const X = 'strlen';

    /**
     * @param callable $a
     * @param array<int, \Closure> $b
     * @param callable(callable): void $c
     * @param-out callable $d
     * @return callable
     */
    public function m(callable $a, array $b, callable $c, &$d)
    {
        return $a;
    }
}

/**
 * @param callable(int): string $ok
 * @return \Closure
 */
function f(callable $ok, callable $plain): \Closure
{
    return fn () => 1;
}

/**
 * @phpstan-assert callable $x
 */
function assertCallable(mixed $x): void
{
}

/**
 * @template T of callable
 * @param T $t
 * @return T
 */
function bound($t)
{
    return $t;
}
