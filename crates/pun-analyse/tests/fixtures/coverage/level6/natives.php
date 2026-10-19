<?php

namespace CoverageNatives;


interface Shape
{
}

interface Named
{
}

class Base
{
    public function make(): static
    {
        return $this;
    }
}

class Natives extends Base
{
    public function unions(int|string|null $a, (Shape&Named)|null $b, ?float $c, true $d, false $e): bool|int
    {
        return 1;
    }

    public function keywords(array $a, callable $b, object $c, mixed $d, iterable $e, self $f, parent $g): static
    {
        return $this;
    }

    public function never(): never
    {
        throw new \Exception();
    }

    public function nothing(): void
    {
    }

    public function intersect(Shape&Named $a): Shape&Named
    {
        return $a;
    }
}

function free(self $a, parent $c): iterable
{
    return [];
}

class Magic
{
    public function __construct()
    {
    }

    public function __destruct()
    {
    }

    public function __toString()
    {
        return '';
    }

    public function __isset($name)
    {
        return false;
    }

    public function __unset($name)
    {
    }

    public function __sleep()
    {
        return [];
    }

    public function __wakeup()
    {
    }

    public function __clone()
    {
    }

    public static function __set_state($state)
    {
        return new self();
    }
}

class Promoted
{
    public function __construct(
        /** @var array<int, string> */
        public array $named,
        /** @var list<int> $listed */
        public array $listed,
        public array $plain = [],
        public ?array $nullable = null,
        public ?array $upper = NULL,
    ) {
    }
}

function selfish(): self
{
    return new \stdClass();
}

function staticky(): static
{
    return new \stdClass();
}

function nested(Shape&Named&\Countable $a, ?array $b = null, array $c = NULL, ?array $d = \null): void
{
}
