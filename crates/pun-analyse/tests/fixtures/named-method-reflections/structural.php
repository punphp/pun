<?php

namespace CompoundBinding;

interface Wide
{
    public function run(mixed $left, mixed $tail = null): void;
}

interface Narrow
{
    public function run(mixed $right): void;
}

function asymmetric(Wide|Narrow $union, Narrow|Wide $reversed, Wide&Narrow $intersection, Narrow&Wide $reverseIntersection): void
{
    $union->run(left: 1);
    $reversed->run(right: 1);
    $union->run();
    $reversed->run(1, 2, 3);
    $intersection->run(left: 1);
    $reverseIntersection->run(left: 1);
    $intersection->run(right: 1);
    $reverseIntersection->run(right: 1);
    $union->run(...['left|right' => 1]);
    $union->run(1, ...['left|right' => 2]);
    $intersection->run(1, left: 2);
    $intersection->run(...['left' => 1]);
    $union->run(...['bogus' => 1]);
}

interface Tail
{
    public function run(mixed ...$items): void;
}

function variadic(Tail|Wide $union, Wide|Tail $reversed, Tail&Wide $intersection, Wide&Tail $reverseIntersection): void
{
    $union->run(tail: 1, extra: 2);
    $reversed->run(tail: 1, extra: 2);
    $intersection->run(extra: 1);
    $reverseIntersection->run(extra: 1);
    $union->run(1, 2, 3, 4);
    $reversed->run(1, 2, 3, 4);
}

interface Open
{
    public function run(mixed $value): void;
}

class Hidden
{
    private function run(mixed $value, mixed $tail = null): void {}

    public function __construct() { $this->run(1); }
}

class Restricted
{
    protected function run(mixed $value): void {}
}

function visibility(Open|Hidden $private, Hidden|Open $reversePrivate, Open|Restricted $protected, Open&Hidden $publicIntersection, Hidden&Open $reverseIntersection): void
{
    $private->run(1);
    $reversePrivate->run(1);
    $protected->run(1);
    $publicIntersection->run(1);
    $reverseIntersection->run(1);
}

/** @no-named-arguments */
interface Closed
{
    public function run(mixed $value): void;
}

function policy(Open|Closed $union, Closed|Open $reversed, Open&Closed $intersection, Closed&Open $reverseIntersection): void
{
    $union->run(value: 1);
    $reversed->run(value: 1);
    $intersection->run(value: 1);
    $reverseIntersection->run(value: 1);
}

/** @template T */
class Generic
{
    /** @param T $value */
    public function run(mixed $value): void {}
}

class Text
{
    public function run(string $value): void {}
}

/** @extends Generic<int> */
class Inherited extends Generic {}

/** @param Generic<int>|Text $generic */
function templates(Generic|Text $generic, Inherited|Text $inherited): void
{
    // The combined parameter is int|string, not the first leaf's int. Structural activation must not
    // accidentally send a surviving User callee to the semantic checker.
    $generic->run('text');
    $generic->run(1);
    $generic->run(extra: 1);
    $inherited->run(extra: 1);
}

/** @mixin Wide|Narrow */
class UnionProxy {}

/** @mixin Narrow&Wide */
class IntersectionProxy {}

/** @mixin Generic<int>|Text */
class GenericProxy {}

function mixins(UnionProxy $union, IntersectionProxy $intersection, GenericProxy $generic): void
{
    $union->run(left: 1);
    $intersection->run(right: 1);
    $intersection->run(left: 1);
    $generic->run('text');
    $generic->run(extra: 1);
}

function casing(Wide|Narrow $union, Wide&Narrow $intersection): void
{
    $name = 'RUN';
    $union->$name(1);
    $intersection->$name(1);
    $union->$name(extra: 1);
}

/** @param-closure-this \stdClass $callback */
function closureBinding(\Closure $callback): ?\Closure
{
    $accepted = $callback->bindTo(new \stdClass());
    return $callback->bindTo(new Text());
}
