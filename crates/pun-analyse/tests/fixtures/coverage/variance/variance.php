<?php

namespace CovVariance;

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

/** @template T */
trait Tr
{
}

/** @template-covariant T */
class Base
{
}

/** @template-covariant T of int */
interface Bounded
{
}

/**
 * @template T
 * @param Bounded<covariant string> $a
 * @param Out<covariant T> $b
 * @param Bounded<covariant int> $c
 */
function bounded($a, $b, $c)
{
}

/**
 * @param Out<covariant int> $a
 * @param Out<contravariant int> $b
 * @param In<covariant int> $c
 * @param Inv<covariant int> $d
 * @param Out<*> $e
 * @param In<contravariant int> $f
 * @return Out<covariant int>
 */
function variances($a, $b, $c, $d, $e, $f)
{
}

/**
 * @template U of Out<covariant int>
 * @template V of mixed = In<covariant int>
 */
function templates()
{
}

/**
 * @implements Out<covariant int>
 * @implements Inv<*>
 */
class Impl implements Out, Inv
{
}

/**
 * @extends Base<contravariant int>
 */
class Child extends Base
{
    /** @use Tr<covariant int> */
    use Tr;

    /** @var Out<contravariant int> */
    public $p;

    /**
     * @phpstan-assert Out<covariant int> $a
     */
    public function check(mixed $a): void
    {
        /** @var In<covariant int> $x */
        $x = $a;
    }
}

/**
 * @mixin Out<contravariant int>
 */
class Mixed_
{
}

/**
 * @template T of array{k: int}
 * @param array{a: int}['a'] $h
 * @param array{a: int}['z'] $i
 * @param T['k'] $j
 * @param string[0] $k
 * @return list<int>[0]
 */
function offsets(string $h, $i, $j, int $k): string
{
    return '';
}

/**
 * @template T of array{k: int}
 * @param T $t
 * @return (T['k'] is int ? 1 : 2)
 */
function conditional($t)
{
    return 1;
}

class SelfOut
{
    /**
     * @phpstan-self-out array{a: Missing}['a']
     */
    public function out(): void
    {
    }
}

class SelfOut2
{
    /**
     * @phpstan-self-out Inv<covariant Missing>
     */
    public function out(): void
    {
    }
}

/**
 * @template T of \ArrayAccess
 * @param T $t
 * @return (T['k'] is int ? 1 : 2)
 */
function conditionalObject($t)
{
    return 1;
}

/**
 * @param int $t
 * @return (int['k'] is int ? 1 : 2)
 */
function conditionalError($t)
{
    return 1;
}
