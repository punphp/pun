<?php

namespace Cov;

enum Suit: string
{
    case H = 'h';
    case S = 's';
}

enum One: int
{
    case A = 1;
}

enum Pure
{
    case X;
}

/**
 * @template T
 */
class Box
{
}

/**
 * @param value-of<Suit> $a
 * @param value-of<One> $b
 * @param value-of<Pure> $c
 * @param value-of<Suit::H> $d
 * @param key-of<array<int, string>> $e
 * @param value-of<array<int, string>> $f
 * @param value-of<int> $g
 * @param key-of<Suit> $h
 * @param value-of<Suit, One> $i
 * @return pure-callable(): void
 */
function enums($a, $b, $c, $d, $e, $f, $g, $h, $i)
{
}

/**
 * @template T of Box
 * @param T<int> $a
 * @param pure-closure $b
 * @param static-closure $c
 * @param static-pure-closure $d
 * @param pure-callable(int): int $e
 * @param ?value-of<T> $f
 */
function generic($a, $b, $c, $d, $e, $f)
{
}

/**
 * @template T
 * @param callable<Suit>(Suit): void $a
 * @param callable<T>(T): T $b
 * @param callable<U of int = string>(U): U $c
 * @param callable<U = int, V>(U, V): void $d
 * @param callable<Money>(Money): void $e
 * @param callable<U of resource>(U): void $f
 * @param callable<U of Missing>(): void $g
 * @param callable<U of int = Missing>(U): void $h
 * @param \Closure<U>(U): U $i
 * @return array<callable<T>(T): void>
 */
function templates(callable $a, callable $b, callable $c, callable $d, callable $e, callable $f, callable $g, callable $h, \Closure $i): array
{
    return [];
}

/**
 * @template T
 */
class Holder
{
    /** @var callable<T>(T): void */
    public $a;

    /** @var \Closure<T>(T): void */
    public \Closure $b;

    public \Closure $c {
        /** @param \Closure<T>(T): void $value */
        set(\Closure $value) {
            $this->c = $value;
        }
    }

    /**
     * @param \Closure<T>(T): void $d
     */
    public function __construct(public \Closure $d)
    {
    }

    /**
     * @template U
     * @param callable<U>(U): void $a
     * @param callable<T>(T): void $b
     * @return callable<V>(V): V
     */
    public function method(callable $a, callable $b): callable
    {
        return $a;
    }
}

/**
 * @template T of array{k: int}
 */
class Offsets
{
    /**
     * @param T['k'] $b
     */
    public function __construct(
        /** @var T['k'] */
        public mixed $a,
        public mixed $b,
    ) {
    }
}
