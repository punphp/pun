<?php

namespace CoverageScope;

class Guarded
{
    function f(): void
    {
        if (true === method_exists($this, 'a')) {
            $this->a();
        }
        if (false !== method_exists($this, 'b')) {
            $this->b();
        }
        if (method_exists($this, 'c') !== true) {
            return;
        }
        $this->c();
        if (true !== method_exists($this, 'd')) {
            return;
        }
        $this->d();
        if (method_exists(static::class, 'e')) {
            static::e();
        }
        if (method_exists(self::class, 'f')) {
            self::f();
        }
    }
}

trait Constructs
{
    function __construct(): void
    {
    }

    function value(): int
    {
        return 1;
    }
}

trait Wraps
{
    use Constructs;
}

class Own
{
    use Constructs, Wraps;

    function __construct()
    {
    }

    function value(): int
    {
        return 2;
    }
}

enum Suit
{
    case Hearts;
}

/**
 * @phpstan-type Me $this
 * @phpstan-type Card Suit::Hearts
 */
class Aliased
{
    /** @var array<Card, Me> */
    public array $cards = [];
}

function keys(): array
{
    return [\Nope\Missing::KEY => 1];
}

class ConcreteChoice
{
}

abstract class AbstractChoice
{
}

function guardedClassChoice(): void
{
    $class = class_exists(MissingChoice::class) ? MissingChoice::class : ConcreteChoice::class;
    new $class;
}

function knownClassChoice(bool $first): void
{
    $class = $first ? AbstractChoice::class : ConcreteChoice::class;
    new $class;
}

function knownStringChoice(bool $first): void
{
    $class = $first ? 'CoverageScope\\AbstractChoice' : 'CoverageScope\\ConcreteChoice';
    new $class;
}
