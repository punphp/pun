<?php

namespace CoverageCalls;

/** @method void tagged() */
class Tagged
{
    function f(): void
    {
        $this->tagged();
        $this->untagged();
    }
}

class Apc extends \APCIterator
{
    function f(): void
    {
        parent::rewind();
        \APCIterator::rewind();
        new parent('user');
        echo \APCIterator::NOPE;
    }
}

class Spread
{
    function __construct(int $a, int $b)
    {
    }
}

function two(int $a, int $b): void
{
}

function spreads(array $args): void
{
    new Spread(...$args);
    two(...array(1, 2));
    two(...array(1));
    two(...[...$args]);
    new class(a: 1, b: 2) {
        function __construct(int $a, int $b)
        {
        }
    };
    new class(c: 1) {
        function __construct(int $a)
        {
        }
    };
}

function loops(int $i): void
{
    while ($i > 0):
        $i--;
    endwhile;
}

class Failure extends \Exception
{
    static function make(): static
    {
        return new static('failed');
    }
}
