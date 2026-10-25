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

function newCallable(): void
{
    $make = new \stdClass(...);
}

/**
 * @method static void make(int $count)
 * @method void instance(int $count)
 */
class StaticTagged
{
    static function real(int $first, int $second): void
    {
    }

    function inScope(): void
    {
        self::make();
        self::instance(1);
    }
}

class InheritedTagged extends StaticTagged
{
}

function taggedStaticCalls(): void
{
    StaticTagged::make();
    StaticTagged::make(count: 1);
    StaticTagged::make(unexpected: 1);
    StaticTagged::instance(1);
    InheritedTagged::make();
    StaticTagged::real(1);
}

/**
 * @template T of \Exception
 * @template U
 */
class GenericCalls
{
    static function finish(int $count): void
    {
    }

    function run(): void
    {
        echo self::finish(1);
        echo static::finish(1);
        echo GenericCalls::finish(1);
        self::finish();
    }
}
