<?php

namespace CovSigVariance;

/** @template-covariant T */
interface Out
{
}

/** @template-contravariant T */
interface In
{
}

/** @template T */
interface Inv
{
}

/**
 * @template-covariant T
 * @template-contravariant U
 */
interface Pair
{
}

/**
 * @template-covariant U
 * @param U $u
 */
function f($u): void
{
}

/**
 * @template-covariant X
 * @template-contravariant Y
 */
abstract class C
{
    /**
     * @param X $a
     * @param Out<X> $b
     * @param In<X> $c
     * @param Inv<X> $d
     * @param Out<covariant X> $e
     * @param In<*> $f
     * @param array<X> $g
     * @param callable(X): Y $h
     * @param \Closure(Y): X $i
     * @param array{a: X} $j
     * @param class-string<X> $k
     * @param object{p: X} $l
     * @param X|int $m
     * @param X&Out<int> $n
     * @param iterable<X> $o
     * @param-out X $p
     * @param-out Y $q
     */
    public function a($a, $b, $c, $d, $e, $f, $g, $h, $i, $j, $k, $l, $m, $n, $o, &$p, &$q): void
    {
    }

    /** @return Y */
    abstract public function b();

    /** @return (X is int ? Y : X) */
    abstract public function c(mixed $v);

    /** @return \Traversable<int, Y> */
    abstract public function d();

    /** @return \Generator<int, Y, X, Y> */
    abstract public function e();

    /** @return Pair<Y, X> */
    abstract public function g();

    /** @return key-of<X> */
    abstract public function h();

    /** @param X $a */
    private function priv($a): void
    {
    }

    /** @param X $a */
    public function __construct($a)
    {
    }

    /** @param X $a */
    public static function stat($a): void
    {
    }

    /**
     * @template-contravariant V
     * @param V $v
     */
    public function own($v): void
    {
    }
}

/**
 * @template-covariant X
 * @template-contravariant Y
 */
class P
{
    /** @var X */
    public $a;

    /** @var X */
    public readonly mixed $b;

    /**
     * @var X
     * @readonly
     */
    public $c;

    /** @var X */
    public private(set) mixed $d;

    /** @var X */
    public mixed $e {
        get => $this->e;
    }

    /** @var X */
    public mixed $f {
        set => $value;
    }

    /** @var Y */
    public readonly mixed $g;

    /** @var X */
    private $h;

    /** @param X $i */
    public function __construct(public $i, private $j)
    {
    }
}

/**
 * @template-covariant X
 * @readonly
 */
class Q
{
    /** @var X */
    public $a;
}

/**
 * @template-contravariant T
 * @extends P<T, T>
 */
class R extends P
{
}

/**
 * @template-covariant T
 * @implements Pair<T, T>
 */
abstract class S implements Pair
{
}

/**
 * @template-covariant T
 * @extends Pair<T, T>
 */
interface I extends Pair
{
}

/** @template T */
trait Tr
{
}

/** @template-contravariant T */
class U
{
    /** @use Tr<T> */
    use Tr;
}

/** @implements Pair<int, int> */
enum E implements Pair
{
}

/** @template-covariant T */
interface Proc
{
    /** @param iterable<T> $items */
    public function run($items): void;
}

/** @implements Proc<string> */
final class Printer implements Proc
{
    public function run($items): void
    {
    }
}
