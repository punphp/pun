<?php

namespace CovPhpDocClasses;

trait T
{
}

class Foo
{
}

/**
 * @param Missing $a
 * @param T $b
 * @param array<Gone> $c
 * @param-out Out $d
 * @param \Closure $e
 * @param-closure-this ThisOne $e
 * @return Ret
 */
function f($a, $b, $c, &$d, $e)
{
}

/**
 * @param Missing $a
 */
function both(Missing $a): void
{
}

/** @param foo $a */
function cased($a): void
{
}

/** @return T */
function trait_return()
{
}

/**
 * @template U of Bound
 * @param U $a
 */
function bounded($a): void
{
}

class C
{
    /**
     * @param Missing $a
     * @return Ret
     */
    public function m($a)
    {
    }
}

abstract class P
{
    /** @param Inherited $a */
    abstract public function m($a): void;
}

class Q extends P
{
    public function m($a): void
    {
    }
}

$f = /** @param Missing $a */ function ($a): void {
};

class Outer
{
    /**
     * @template T
     * @param T $t
     */
    public function some($t): object
    {
        return new class($t) {
            /** @param T $t */
            public function __construct(private $t)
            {
            }

            /** @return T */
            public function get()
            {
                return $this->t;
            }
        };
    }
}

/**
 * @template T
 * @param T $t
 */
function wrap($t): object
{
    return new class {
        /** @return T|null */
        public function get()
        {
            return null;
        }

        /** @return Nope */
        public function nope()
        {
            return null;
        }
    };
}
